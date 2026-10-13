use super::{limits, FactSet, FactValue, Receipt, Rejection};
#[path = "receipts_json.rs"]
mod json;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

fn invalid() -> Rejection {
    Rejection::new(
        "invalid_receipt",
        "Beleg verweist nicht auf den gespeicherten Rohwert",
    )
}

fn offset(text: &str) -> Result<usize, Rejection> {
    if text.is_empty()
        || (text.len() > 1 && text.starts_with('0'))
        || !text.bytes().all(|b| b.is_ascii_digit())
    {
        return Err(invalid());
    }
    text.parse().map_err(|_| invalid())
}

fn byte_ranges(bytes: &[u8], locator: &str) -> Result<Vec<u8>, Rejection> {
    let mut value = Vec::new();
    let mut previous_end = 0;
    for (index, range) in locator.split(',').enumerate() {
        let (start, end) = range.split_once('-').ok_or_else(invalid)?;
        let (start, end) = (offset(start)?, offset(end)?);
        if index >= limits::BYTE_RANGES
            || start >= end
            || (index > 0 && start < previous_end)
            || end > bytes.len()
            || value.len() + (end - start) > limits::BYTE_RANGE_BYTES
        {
            return Err(invalid());
        }
        value.extend_from_slice(&bytes[start..end]);
        previous_end = end;
    }
    Ok(value)
}

struct ValueIndex<'a> {
    bytes: &'a [u8],
    json: Option<json::Index<'a>>,
    kv: Option<BTreeMap<String, String>>,
}

impl<'a> ValueIndex<'a> {
    fn new(bytes: &'a [u8]) -> Result<Self, Rejection> {
        Ok(Self {
            bytes,
            json: json::Index::parse(bytes)?,
            kv: None,
        })
    }

    fn value(&mut self, receipt: &Receipt) -> Result<Vec<u8>, Rejection> {
        match receipt.locator_kind.as_str() {
            "raw_bytes" if receipt.locator.is_empty() => Ok(self.bytes.to_vec()),
            "json_pointer" => self
                .json
                .as_mut()
                .ok_or_else(invalid)?
                .value(&receipt.locator)
                .map(<[u8]>::to_vec),
            "byte_ranges" => byte_ranges(self.bytes, &receipt.locator),
            "kv_path" => {
                if self.kv.is_none() {
                    let input = std::str::from_utf8(self.bytes).map_err(|_| invalid())?;
                    self.kv = Some(
                        crate::entity_profile::derivation::game_file_facts::kv3_lexemes(input)
                            .map_err(|_| invalid())?,
                    );
                }
                self.kv
                    .as_ref()
                    .and_then(|kv| kv.get(&receipt.locator))
                    .map(|lexeme| lexeme.as_bytes().to_vec())
                    .ok_or_else(invalid)
            }
            _ => Err(invalid()),
        }
    }
}

pub fn value_bytes(bytes: &[u8], receipt: &Receipt) -> Result<Vec<u8>, Rejection> {
    ValueIndex::new(bytes)?.value(receipt)
}

pub fn verify(bytes: &[u8], set: &FactSet) -> Result<(), Rejection> {
    if bytes.len() > limits::RAW_BYTES {
        return Err(Rejection::new(
            "raw_too_large",
            "Rohobjekt überschreitet die Extraktionsgrenze",
        ));
    }
    limits::validate(set)?;
    let receipts = set
        .entities
        .iter()
        .map(|r| (&r.receipt, false))
        .chain(set.aliases.iter().map(|r| (&r.receipt, false)))
        .chain(
            set.facts
                .iter()
                .map(|r| (&r.receipt, matches!(r.value, FactValue::Number(_)))),
        )
        .chain(set.relations.iter().map(|r| (&r.receipt, false)));
    let mut values = ValueIndex::new(bytes)?;
    let mut hashes = BTreeMap::new();
    for (receipt, numeric) in receipts {
        let key = (&receipt.locator_kind, &receipt.locator);
        let (hash, number) = match hashes.entry(key) {
            std::collections::btree_map::Entry::Occupied(entry) => entry.into_mut(),
            std::collections::btree_map::Entry::Vacant(entry) => {
                let value = values.value(receipt)?;
                let hash: [u8; 32] = Sha256::digest(&value).into();
                let number = std::str::from_utf8(&value)
                    .map_err(|_| invalid())
                    .and_then(limits::validate_numeric_lexeme);
                entry.insert((hash, number))
            }
        };
        if *hash != receipt.value_sha256 {
            return Err(invalid());
        }
        if numeric
            && matches!(
                receipt.locator_kind.as_str(),
                "json_pointer" | "kv_path" | "byte_ranges"
            )
        {
            number.clone()?;
        }
    }
    Ok(())
}
