use crate::Extractor;

pub const EXTRACTORS: &[&dyn Extractor] =
    &[&crate::hero::Hero, &crate::patch_extract::PatchExtractor];
