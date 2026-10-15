#[path = "docs_extract.rs"]
mod extract;
#[path = "docs_source.rs"]
pub mod source;

pub use extract::DocsExtractor;
