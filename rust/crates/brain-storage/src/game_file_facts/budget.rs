// Conservative cumulative allocation accounting, including downstream fact expansion.
// Charges never get refunded, so transient copies and retained values share a bound.
pub const EXCEEDED: &str = "allocation_budget_exceeded_preserved_as_text";
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
