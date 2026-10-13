use crate::Extractor;

pub const EXTRACTORS: &[&dyn Extractor] = &[&crate::hero::Hero, &crate::meta_stats::MetaStats];
