use crate::Extractor;

pub const EXTRACTORS: &[&dyn Extractor] = &[&crate::docs::DocsExtractor];
