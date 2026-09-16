#![forbid(unsafe_code)]

pub mod aggregate;
pub mod api;
pub mod catalog;
pub mod clean;
pub mod cli;
pub mod db;
pub mod index;
pub mod metrics;

pub use catalog::Catalog;
pub use clean::{match_to_rows, player_won, PlayerMatchRow};
pub use cli::{run_population, PopulationArgs};
pub use index::PopulationIndex;
pub use metrics::{jaccard_at, kendall_tau};

pub const BUCKET_ALL: &str = "all";
pub const BUCKET_WEAPON: &str = "weapon";
pub const BUCKET_SPIRIT: &str = "spirit";
pub const BUCKET_VITALITY: &str = "vitality";

pub const STAPLE_THRESHOLD: f64 = 0.70;
pub const IMBUE_SPLIT_THRESHOLD: f64 = 0.50;
pub const IMBUE_THIN_OBSERVATIONS: i64 = 30;
pub const MIN_CELL_OBSERVATIONS: i64 = 300;
pub const NEIGUNG_MIN_SHARE: f64 = 0.25;
pub const ABILITY_ORDER_PREFIX: usize = 16;

pub const BADGE_CENTER: f64 = 80.0;
pub const BADGE_HALFWIDTH: f64 = 25.0;
pub const WIN_WEIGHT: f64 = 1.0;
