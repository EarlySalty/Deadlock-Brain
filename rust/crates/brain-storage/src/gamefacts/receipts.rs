use super::{FactSet, Receipt, Rejection};
use serde_json::value::RawValue;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

fn invalid() -> Rejection {
    Rejection::new(
        "invalid_receipt",
        "Beleg verweist nicht auf den gespeicherten Rohwert",
    )
}

fn json_value(bytes: &[u8], pointer: &str) -> Result<Vec<u8>, Rejection> {
    let mut value: Box<RawValue> = serde_json::from_slice(bytes).map_err(|_| invalid())?;
    if !pointer.is_empty() {
        let path = pointer.strip_prefix('/').ok_or_else(invalid)?;
        for token in path.split('/') {
            let key = token.replace("~1", "/").replace("~0", "~");
            value = match value.get().as_bytes().first() {
                Some(b'{') => {
                    let mut fields: BTreeMap<String, Box<RawValue>> =
                        serde_json::from_str(value.get()).map_err(|_| invalid())?;
                    fields.remove(&key).ok_or_else(invalid)?
                }
                Some(b'[') => {
                    if key.len() > 1 && key.starts_with('0') {
                        return Err(invalid());
                    }
                    let index = key.parse::<usize>().map_err(|_| invalid())?;
                    let values: Vec<Box<RawValue>> =
                        serde_json::from_str(value.get()).map_err(|_| invalid())?;
                    values.into_iter().nth(index).ok_or_else(invalid)?
                }
                _ => return Err(invalid()),
            };
        }
    }
    Ok(value.get().as_bytes().to_vec())
}

pub fn value_bytes(bytes: &[u8], receipt: &Receipt) -> Result<Vec<u8>, Rejection> {
    match receipt.locator_kind.as_str() {
        "raw_bytes" if receipt.locator.is_empty() => Ok(bytes.to_vec()),
        "json_pointer" => json_value(bytes, &receipt.locator),
        _ => Err(invalid()),
    }
}

pub fn verify(bytes: &[u8], set: &FactSet) -> Result<(), Rejection> {
    let receipts = set
        .entities
        .iter()
        .map(|r| &r.receipt)
        .chain(set.aliases.iter().map(|r| &r.receipt))
        .chain(set.facts.iter().map(|r| &r.receipt))
        .chain(set.relations.iter().map(|r| &r.receipt));
    let mut hashes = BTreeMap::new();
    for receipt in receipts {
        let key = (&receipt.locator_kind, &receipt.locator);
        let hash = match hashes.get(&key) {
            Some(hash) => *hash,
            None => {
                let hash: [u8; 32] = Sha256::digest(value_bytes(bytes, receipt)?).into();
                hashes.insert(key, hash);
                hash
            }
        };
        if hash != receipt.value_sha256 {
            return Err(invalid());
        }
    }
    Ok(())
}
