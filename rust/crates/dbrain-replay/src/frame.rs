//! Checked forward-only framing replaces haste's fixed-buffer DemoFile reader.
//! In particular, a truncated final varint MUST NOT be mistaken for clean EOF by Parser::run.
use crate::{DecodeBudget, RawLocator, ReplayFailure, hash};
use haste_core::demostream::{
    CmdHeader, DecodeCmdError, DemoStream, ReadCmdError, ReadCmdHeaderError,
};
use prost::Message;
use std::{
    cell::RefCell,
    io::{self, Read},
    rc::Rc,
};
use valveprotos::common::{
    CDemoClassInfo, CDemoFullPacket, CDemoPacket, CDemoSendTables, EDemoCommands,
};

#[derive(Default)]
pub(crate) struct State {
    pub failure: Option<ReplayFailure>,
    pub locator: Option<RawLocator>,
    pub commands: u64,
    pub packets: u64,
    pub decoded: u64,
}
pub(crate) type SharedState = Rc<RefCell<State>>;
impl State {
    pub fn charge_decoded(
        &mut self,
        amount: u64,
        budget: &DecodeBudget,
    ) -> Result<(), ReplayFailure> {
        let total = self
            .decoded
            .checked_add(amount)
            .ok_or(ReplayFailure::BudgetExceeded)?;
        if amount > budget.max_command_bytes || total > budget.max_total_decoded_bytes {
            return Err(ReplayFailure::BudgetExceeded);
        }
        self.decoded = total;
        Ok(())
    }
}
pub(crate) struct BoundedStream<R: Read> {
    reader: R,
    budget: DecodeBudget,
    state: SharedState,
    length: u64,
    offset: u64,
    command_offset: u64,
    fileinfo_offset: u64,
    spawngroups_offset: u64,
    fileinfo_found: bool,
    spawngroups_found: bool,
    seen_stop: bool,
    clean_end: bool,
    data: Vec<u8>,
}
impl<R: Read> BoundedStream<R> {
    pub fn new(
        mut reader: R,
        length: u64,
        budget: DecodeBudget,
        state: SharedState,
    ) -> Result<Self, ReplayFailure> {
        if length > budget.max_file_bytes {
            return Err(ReplayFailure::BudgetExceeded);
        }
        let mut header = [0u8; 16];
        reader
            .read_exact(&mut header)
            .map_err(|_| ReplayFailure::DamagedReplay)?;
        if &header[..8] != b"PBDEMS2\0" {
            return Err(ReplayFailure::InvalidContainer);
        }
        let info = i32::from_le_bytes(
            header[8..12]
                .try_into()
                .map_err(|_| ReplayFailure::InvalidContainer)?,
        );
        let spawn = i32::from_le_bytes(
            header[12..16]
                .try_into()
                .map_err(|_| ReplayFailure::InvalidContainer)?,
        );
        for p in [info, spawn] {
            if p != 0 && (p < 16 || p as u64 >= length) {
                return Err(ReplayFailure::DamagedReplay);
            }
        }
        Ok(Self {
            reader,
            budget,
            state,
            length,
            offset: 16,
            command_offset: 16,
            fileinfo_offset: info as u64,
            spawngroups_offset: spawn as u64,
            fileinfo_found: info == 0,
            spawngroups_found: spawn == 0,
            seen_stop: false,
            clean_end: false,
            data: Vec::new(),
        })
    }
    fn error(&self, reason: ReplayFailure) -> io::Error {
        self.state.borrow_mut().failure = Some(reason);
        io::Error::new(io::ErrorKind::InvalidData, reason)
    }
    fn varint(&mut self) -> Result<Option<(u32, u8)>, io::Error> {
        let mut value = 0u32;
        for index in 0..5u8 {
            let mut byte = [0u8];
            match self.reader.read_exact(&mut byte) {
                Ok(()) => {
                    self.offset += 1;
                }
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof && index == 0 => {
                    return Ok(None);
                }
                Err(_) => return Err(self.error(ReplayFailure::DamagedReplay)),
            }
            if self.offset > self.length || self.offset > self.budget.max_file_bytes {
                return Err(self.error(ReplayFailure::DamagedReplay));
            }
            let b = byte[0];
            if index == 4 && b & 0xf0 != 0 {
                return Err(self.error(ReplayFailure::DamagedReplay));
            }
            value |= u32::from(b & 0x7f) << (7 * index);
            if b & 0x80 == 0 {
                if index > 0 && b == 0 {
                    return Err(self.error(ReplayFailure::DamagedReplay));
                }
                return Ok(Some((value, index + 1)));
            }
        }
        Err(self.error(ReplayFailure::DamagedReplay))
    }
    fn required_varint(&mut self) -> Result<(u32, u8), io::Error> {
        self.varint()?
            .ok_or_else(|| self.error(ReplayFailure::DamagedReplay))
    }
}
impl<R: Read> DemoStream for BoundedStream<R> {
    fn is_at_eof(&mut self) -> Result<bool, io::Error> {
        Ok(self.clean_end)
    }
    fn read_cmd_header(&mut self) -> Result<CmdHeader, ReadCmdHeaderError> {
        self.clean_end = false;
        self.command_offset = self.offset;
        let Some((raw_command, command_n)) = self.varint()? else {
            if self.seen_stop
                && self.offset == self.length
                && self.fileinfo_found
                && self.spawngroups_found
            {
                self.clean_end = true;
                return Err(io::Error::from(io::ErrorKind::UnexpectedEof).into());
            }
            return Err(self.error(ReplayFailure::DamagedReplay).into());
        };
        let compressed = raw_command & 64 != 0;
        let number = raw_command & !64;
        // The pinned EDemoCommands defines actual commands 0..=18; 19 and 64 are sentinels.
        if number > 18 {
            return Err(self.error(ReplayFailure::UnknownStructure).into());
        }
        let cmd = EDemoCommands::try_from(number as i32)
            .map_err(|_| self.error(ReplayFailure::UnknownStructure))?;
        let (tick_raw, tick_n) = self.required_varint()?;
        if tick_raw > i32::MAX as u32 && tick_raw != u32::MAX {
            return Err(self.error(ReplayFailure::UnknownStructure).into());
        }
        let (body_size, size_n) = self.required_varint()?;
        if u64::from(body_size) > self.budget.max_command_bytes {
            return Err(self.error(ReplayFailure::BudgetExceeded).into());
        }
        if u64::from(body_size) > self.length.saturating_sub(self.offset) {
            return Err(self.error(ReplayFailure::DamagedReplay).into());
        }
        let count = self.state.borrow().commands;
        if count >= self.budget.max_commands {
            return Err(self.error(ReplayFailure::BudgetExceeded).into());
        }
        if (count == 0) != (cmd == EDemoCommands::DemFileHeader) {
            return Err(self.error(ReplayFailure::InvalidContainer).into());
        }
        if self.seen_stop
            && !matches!(
                cmd,
                EDemoCommands::DemFileInfo | EDemoCommands::DemSpawnGroups
            )
        {
            return Err(self.error(ReplayFailure::DamagedReplay).into());
        }
        if self.command_offset == self.fileinfo_offset {
            if cmd != EDemoCommands::DemFileInfo {
                return Err(self.error(ReplayFailure::DamagedReplay).into());
            }
            self.fileinfo_found = true;
        }
        if self.command_offset == self.spawngroups_offset {
            if cmd != EDemoCommands::DemSpawnGroups {
                return Err(self.error(ReplayFailure::DamagedReplay).into());
            }
            self.spawngroups_found = true;
        }
        if cmd == EDemoCommands::DemStop {
            self.seen_stop = true;
        }
        self.state.borrow_mut().commands += 1;
        Ok(CmdHeader {
            cmd,
            body_compressed: compressed,
            tick: tick_raw as i32,
            body_size,
            size: command_n + tick_n + size_n,
        })
    }
    fn read_cmd(&mut self, h: &CmdHeader) -> Result<&[u8], ReadCmdError> {
        let size = h.body_size as usize;
        let mut stored = Vec::new();
        stored
            .try_reserve_exact(size)
            .map_err(|_| self.error(ReplayFailure::BudgetExceeded))?;
        stored.resize(size, 0);
        self.reader
            .read_exact(&mut stored)
            .map_err(|_| self.error(ReplayFailure::DamagedReplay))?;
        self.offset += size as u64;
        let decoded_size = if h.body_compressed {
            snap::raw::decompress_len(&stored)
                .map_err(|_| self.error(ReplayFailure::DamagedReplay))?
        } else {
            size
        };
        let charge = self
            .state
            .borrow_mut()
            .charge_decoded(decoded_size as u64, &self.budget);
        charge.map_err(|e| self.error(e))?;
        if h.body_compressed {
            self.data.clear();
            self.data
                .try_reserve_exact(decoded_size)
                .map_err(|_| self.error(ReplayFailure::BudgetExceeded))?;
            self.data.resize(decoded_size, 0);
            let actual = snap::raw::Decoder::new()
                .decompress(&stored, &mut self.data)
                .map_err(|_| self.error(ReplayFailure::DamagedReplay))?;
            if actual != decoded_size {
                return Err(self.error(ReplayFailure::DamagedReplay).into());
            }
        } else {
            self.data = stored;
        }
        let mut state = self.state.borrow_mut();
        state.locator = Some(RawLocator {
            command_index: state.commands - 1,
            file_byte_offset: self.command_offset,
            stored_byte_length: self.offset - self.command_offset,
            compressed: h.body_compressed,
            payload_sha256: hash(&self.data),
            packet_index: None,
            entity_callback_index: None,
            requires_state_prefix: false,
        });
        Ok(&self.data)
    }
    fn skip_cmd(&mut self, h: &CmdHeader) -> Result<(), io::Error> {
        self.read_cmd(h)
            .map(|_| ())
            .map_err(|_| self.error(ReplayFailure::DamagedReplay))
    }
    fn decode_cmd_send_tables(data: &[u8]) -> Result<CDemoSendTables, DecodeCmdError> {
        CDemoSendTables::decode(data).map_err(DecodeCmdError::DecodeProtobufError)
    }
    fn decode_cmd_class_info(data: &[u8]) -> Result<CDemoClassInfo, DecodeCmdError> {
        CDemoClassInfo::decode(data).map_err(DecodeCmdError::DecodeProtobufError)
    }
    fn decode_cmd_packet(data: &[u8]) -> Result<CDemoPacket, DecodeCmdError> {
        CDemoPacket::decode(data).map_err(DecodeCmdError::DecodeProtobufError)
    }
    fn decode_cmd_full_packet(data: &[u8]) -> Result<CDemoFullPacket, DecodeCmdError> {
        CDemoFullPacket::decode(data).map_err(DecodeCmdError::DecodeProtobufError)
    }
}
