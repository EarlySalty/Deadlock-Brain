#![forbid(unsafe_code)]

//! Source-Ingestion-Crate fuer Assets, Patchnotes, Sheets, Statlocker und Wiki.

pub use deadlock_brain_core as core;

pub mod assets_api;
pub mod deadlock_api;
pub mod deadlock_data;
pub mod error;
pub mod forum;
pub mod google_sheet;
pub mod patchnotes_db;
pub mod statlocker;
pub mod wiki;

mod store;
mod util;

pub use assets_api::{pull_assets, PullAssetsOptions};
pub use deadlock_api::{
    pull_demo_evidence, pull_match_metadata, pull_player_match_history, DemoJobState,
    DemoJobStatus, DemoQuery, PullDemoEvidenceOptions, PullMatchMetadataOptions,
    PullPlayerMatchHistoryOptions, DEMO_QUERY_VERSION,
};
pub use deadlock_data::{pull_deadlock_data, PullDeadlockDataOptions};
pub use error::{Result, SourcesError};
pub use forum::{pull_forum, PullForumOptions};
pub use google_sheet::{
    discover_sheet_tabs, pull_sheet, refresh_sheet, sheet_csv_url, sheet_pubhtml_url,
    PullSheetOptions, RefreshSheetOptions, SheetTab,
};
pub use patchnotes_db::{classify_source_kind, pull_patchnotes, PullPatchnotesOptions};
pub use statlocker::{pull_statlocker, PullStatlockerOptions};
pub use wiki::{pull_wiki_page, PullWikiPageOptions};
