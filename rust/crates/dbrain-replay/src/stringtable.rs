//! Checked wire-only preflight for per-entry Snappy, before the pinned backend consumes it.
//! No string table, baseline, entity, or second gameplay parser is implemented here.
use crate::{DecodeBudget, ReplayFailure, frame::SharedState};
use valveprotos::common::CsvcMsgCreateStringTable;

const MAX_USER_DATA: usize = 1 << 17; // Actual fixed scratch-buffer capacity of the pinned backend.
#[derive(Clone)]
pub(crate) struct TableSpec {
    pub name: String,
    fixed_bits: Option<usize>,
    compressed_entries: bool,
    varint_lengths: bool,
}
impl TableSpec {
    pub fn from_message(msg: &CsvcMsgCreateStringTable) -> Result<Self, ReplayFailure> {
        if msg.name().is_empty()
            || msg.name().len() > 256
            || msg.name().chars().any(char::is_control)
            || msg.flags() < 0
            || msg.flags() & !1 != 0
            || msg.user_data_size() < 0
            || msg.user_data_size_bits() < 0
            || msg.user_data_size() as usize > MAX_USER_DATA
            || msg.user_data_size_bits() as usize > MAX_USER_DATA * 8
        {
            return Err(ReplayFailure::UnknownStructure);
        }
        let fixed_bits = if msg.user_data_fixed_size() {
            let bits = msg.user_data_size_bits() as usize;
            // Do not expose stale bytes from the backend's scratch buffer beyond witnessed bits.
            if bits.div_ceil(8) != msg.user_data_size() as usize {
                return Err(ReplayFailure::UnknownStructure);
            }
            Some(bits)
        } else {
            None
        };
        Ok(Self {
            name: msg.name().into(),
            fixed_bits,
            compressed_entries: msg.flags() & 1 != 0,
            varint_lengths: msg.using_varint_bitcounts(),
        })
    }
    pub fn preflight(
        &self,
        data: &[u8],
        entries: u32,
        budget: &DecodeBudget,
        state: &SharedState,
    ) -> Result<(), ReplayFailure> {
        if entries > budget.max_entities {
            return Err(ReplayFailure::BudgetExceeded);
        }
        let mut bits = CheckedBits { data, position: 0 };
        let mut scratch = vec![0u8; MAX_USER_DATA];
        let mut index = -1i64;
        for _ in 0..entries {
            index = if bits.bits(1)? != 0 {
                index + 1
            } else {
                i64::from(bits.varint()?) + 1
            };
            if index < 0 || index >= i64::from(budget.max_entities) {
                return Err(ReplayFailure::BudgetExceeded);
            }
            if bits.bits(1)? != 0 {
                let prefix = if bits.bits(1)? != 0 {
                    bits.bits(5)?;
                    bits.bits(5)? as usize
                } else {
                    0
                };
                let mut terminated = false;
                for _ in prefix..1024 {
                    if bits.bits(8)? == 0 {
                        terminated = true;
                        break;
                    }
                }
                if !terminated {
                    return Err(ReplayFailure::DamagedReplay);
                }
            }
            if bits.bits(1)? == 0 {
                continue;
            }
            if let Some(size) = self.fixed_bits {
                bits.skip(size)?;
                continue;
            }
            let compressed = self.compressed_entries && bits.bits(1)? != 0;
            let stored = if self.varint_lengths {
                bits.ubitvar()? as usize
            } else {
                bits.bits(17)? as usize
            };
            if stored > MAX_USER_DATA {
                return Err(ReplayFailure::BudgetExceeded);
            }
            if compressed {
                for byte in &mut scratch[..stored] {
                    *byte = bits.bits(8)? as u8;
                }
                let decoded = snap::raw::decompress_len(&scratch[..stored])
                    .map_err(|_| ReplayFailure::DamagedReplay)?;
                if decoded > MAX_USER_DATA {
                    return Err(ReplayFailure::BudgetExceeded);
                }
                state.borrow_mut().charge_decoded(decoded as u64, budget)?;
            } else {
                bits.skip(stored * 8)?;
            }
        }
        Ok(())
    }
}
struct CheckedBits<'a> {
    data: &'a [u8],
    position: usize,
}
impl CheckedBits<'_> {
    fn skip(&mut self, count: usize) -> Result<(), ReplayFailure> {
        let end = self
            .position
            .checked_add(count)
            .ok_or(ReplayFailure::DamagedReplay)?;
        if end > self.data.len() * 8 {
            return Err(ReplayFailure::DamagedReplay);
        }
        self.position = end;
        Ok(())
    }
    fn bits(&mut self, count: usize) -> Result<u32, ReplayFailure> {
        if count > 32 {
            return Err(ReplayFailure::DamagedReplay);
        }
        let start = self.position;
        self.skip(count)?;
        let mut value = 0u32;
        for i in 0..count {
            value |= u32::from((self.data[(start + i) / 8] >> ((start + i) % 8)) & 1) << i;
        }
        Ok(value)
    }
    fn varint(&mut self) -> Result<u32, ReplayFailure> {
        let mut value = 0;
        for i in 0..5 {
            let byte = self.bits(8)?;
            if i == 4 && byte & 0xf0 != 0 {
                return Err(ReplayFailure::DamagedReplay);
            }
            value |= (byte & 127) << (7 * i);
            if byte & 128 == 0 {
                return Ok(value);
            }
        }
        Err(ReplayFailure::DamagedReplay)
    }
    fn ubitvar(&mut self) -> Result<u32, ReplayFailure> {
        let base = self.bits(6)?;
        Ok((base & 15)
            | match base & 48 {
                0 => 0,
                16 => self.bits(4)? << 4,
                32 => self.bits(8)? << 4,
                _ => self.bits(28)? << 4,
            })
    }
}
