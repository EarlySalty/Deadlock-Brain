use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub const EXCEEDED: &str = "allocation_budget_exceeded_preserved_as_text";

pub(super) struct Selection<'a> {
    requested: &'a BTreeSet<&'a str>,
    pub hashes: BTreeMap<String, [u8; 32]>,
}
impl<'a> Selection<'a> {
    pub fn new(
        input: &str,
        requested: &'a BTreeSet<&'a str>,
        transient_copies: usize,
        budget: &mut Budget,
    ) -> Result<Self, &'static str> {
        budget.charge(
            input
                .len()
                .checked_mul(transient_copies)
                .and_then(|n| n.checked_add(65536))
                .ok_or(EXCEEDED)?,
        )?;
        for pointer in requested {
            budget.expanded(&[pointer.len()], 256)?;
        }
        Ok(Self {
            requested,
            hashes: BTreeMap::new(),
        })
    }
    pub fn contains(&self, pointer: &str) -> bool {
        pointer != "\0"
            && self.requested.iter().any(|requested| {
                *requested == pointer
                    || requested
                        .strip_prefix(pointer)
                        .is_some_and(|suffix| suffix.starts_with('/'))
            })
    }
    pub fn record(&mut self, pointer: &str, lexeme: &str) -> Result<(), &'static str> {
        if pointer != "\0"
            && self.requested.contains(pointer)
            && self
                .hashes
                .insert(pointer.to_owned(), Sha256::digest(lexeme.as_bytes()).into())
                .is_some()
        {
            return Err("duplicate_json_keys_preserved_as_text");
        }
        Ok(())
    }
}
const MAX_BYTES: usize = 512 * 1024 * 1024;

pub struct Budget {
    remaining: usize,
}
impl Budget {
    pub fn new() -> Self {
        Self {
            remaining: MAX_BYTES,
        }
    }
    pub fn with_limit(bytes: usize) -> Self {
        Self { remaining: bytes }
    }
    pub(super) fn checkpoint(&self) -> usize {
        self.remaining
    }
    pub(super) fn restore(&mut self, checkpoint: usize) {
        self.remaining = checkpoint;
    }
    pub fn charge(&mut self, bytes: usize) -> Result<(), &'static str> {
        self.remaining = self.remaining.checked_sub(bytes).ok_or(EXCEEDED)?;
        Ok(())
    }
    pub fn expanded(&mut self, lengths: &[usize], overhead: usize) -> Result<(), &'static str> {
        let bytes = lengths
            .iter()
            .try_fold(overhead, |sum, n| sum.checked_add(n.checked_mul(8)?))
            .ok_or(EXCEEDED)?;
        self.charge(bytes)
    }
}
pub fn escaped_len(key: &str) -> usize {
    key.len() + key.bytes().filter(|b| matches!(b, b'~' | b'/')).count()
}

impl Default for Budget {
    fn default() -> Self {
        Self::new()
    }
}
