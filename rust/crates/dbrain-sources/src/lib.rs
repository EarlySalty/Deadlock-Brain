#![forbid(unsafe_code)]

//! Source-Ingestion-Crate fuer Assets, Patchnotes, Sheets, Statlocker und Wiki.

pub use deadlock_brain_core as core;

pub mod assets_api;
pub mod deadlock_api;
pub mod error;
pub mod google_sheet;
pub mod patchnotes_db;
pub mod statlocker;
pub mod wiki;

mod store;
mod util;

pub use assets_api::{pull_assets, PullAssetsOptions};
pub use deadlock_api::{pull_match_metadata, PullMatchMetadataOptions};
pub use error::{Result, SourcesError};
pub use google_sheet::{
    discover_sheet_tabs, pull_sheet, refresh_sheet, sheet_csv_url, sheet_pubhtml_url,
    PullSheetOptions, RefreshSheetOptions, SheetTab,
};
pub use patchnotes_db::{classify_source_kind, pull_patchnotes, PullPatchnotesOptions};
pub use statlocker::{pull_statlocker, PullStatlockerOptions};
pub use wiki::{pull_wiki_page, PullWikiPageOptions};
