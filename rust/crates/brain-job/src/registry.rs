use crate::Extractor;

pub const EXTRACTORS: &[&dyn Extractor] = &[&crate::patch_extract::PatchExtractor];
