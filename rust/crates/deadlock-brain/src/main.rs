#![forbid(unsafe_code)]
#![allow(dead_code)]

use std::{
    collections::BTreeSet,
    fs,
    path::PathBuf,
    process,
};

use anyhow::{anyhow, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use deadlock_brain_core::{
    build_narration,
    config::{self, Settings},
    http::HttpClient,
    minimax::{extract_minimax_text, MiniMaxClient, MiniMaxConfig},
};
use serde::Serialize;
use serde_json::{json, Value};
use sqlx::PgPool;

mod pg_entities;
mod pg_patchnotes;

#[derive(Debug, Parser)]
#[command(name = "deadlock-brain")]
struct Cli {
    #[arg(long, global = true, value_name = "PATH", help = "Pfad zur Brain-SQLite-DB (Standard: data/deadlock_brain.sqlite3).")]
    db: Option<PathBuf>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Zeigt lokale DB- und Source-Counts.")]
    Status,
    #[command(about = "Baut einen kompakten Datenkontext fuer eine Entity-Frage.")]
    Context(EntityContextArgs),
    #[command(about = "Baut eine Patch-Timeline fuer eine Entity.")]
    Timeline(TimelineArgs),
    #[command(about = "Baut einen KI-tauglichen Review-Kontext ohne Modellaufruf.")]
    Review(ReviewArgs),
    #[command(name = "ask-context", about = "Baut ein vertrauenssortiertes Wissens-Buendel plus LLM-Prompt zu einer beliebigen Deadlock-Frage.")]
    AskContext(AskContextArgs),
    #[command(about = "Fuehrt lokale Datenqualitaetschecks aus.")]
    Quality(PrettyArgs),
    #[command(about = "Zeigt Rename-/Rework-Beziehungen aus Patchnotes.")]
    Lineage(LineageArgs),
    #[command(about = "Zeigt alte/entfernte Entities aus Patchnotes.")]
    Legacy(LegacyArgs),
    #[command(about = "Sucht Entities nach Typ/Name in der zentralen Postgres (brain.entities + brain.entity_aliases).")]
    Entities(EntitiesArgs),
    #[command(about = "Erzeugt einen erklaerbaren Hero-Build-Vorschlag aus API-/Sheet-/Patchdaten.")]
    Build(BuildArgs),
    #[command(name = "build-context")]
    BuildContext(BuildContextArgs),
    #[command(name = "build-eval")]
    BuildEval(BuildEvalArgs),
    #[command(about = "Zeigt strukturierte Item-Daten aus der Deadlock Assets API.")]
    Item(ItemArgs),
    #[command(about = "Importiert und analysiert Build-Trainingsdaten.")]
    Learn {
        #[command(subcommand)]
        target: LearnCommands,
    },
    #[command(about = "Analysiert Statlocker-Player-Matches als Entscheidungs-Training.")]
    Player {
        #[command(subcommand)]
        target: PlayerCommands,
    },
    #[command(about = "Entdeckt und verarbeitet YouTube-Lernfeeds.")]
    Youtube {
        #[command(subcommand)]
        target: YoutubeCommands,
    },
    #[command(about = "Speichert oder listet persistente Analyse-Kontexte.")]
    Analysis {
        #[command(subcommand)]
        target: AnalysisCommands,
    },
    #[command(about = "Zieht Daten aus einer Quelle.")]
    Pull {
        #[command(subcommand)]
        source: PullCommands,
    },
    #[command(about = "Zieht alle Sheet-Tabs frisch und normalisiert Hero-Stats und Tab-Tabellen.")]
    RefreshSheet,
    #[command(about = "Erzeugt normalisierte Daten aus Snapshots.")]
    Normalize {
        #[command(subcommand)]
        target: NormalizeCommands,
    },
    #[command(about = "Erzeugt strukturierte Daten aus importierten Quellen.")]
    Parse {
        #[command(subcommand)]
        target: ParseCommands,
    },
    #[command(about = "Reichert strukturierte Daten an.")]
    Enrich {
        #[command(subcommand)]
        target: EnrichCommands,
    },
    #[command(name = "pg", about = "Synchronisiert Daten direkt in die zentrale Postgres-DB.")]
    Pg {
        #[command(subcommand)]
        target: PgCommands,
    },
}

#[derive(Debug, Args)]
struct PrettyArgs {
    #[arg(long, help = "Kompakter menschenlesbarer Output statt JSON.")]
    pretty: bool,
}

#[derive(Debug, Args)]
struct EntitiesArgs {
    #[arg(long = "type", value_name = "TYPE", help = "Optionaler Entity-Typ-Filter (z.B. hero, item, ability).")]
    entity_type: Option<String>,
    #[arg(long, value_name = "QUERY", help = "Suchbegriff (Teilstring in canonical_name oder Alias).")]
    query: String,
}

#[derive(Debug, Args)]
struct EntityContextArgs {
    #[arg(help = "Hero, Item, Ability oder Alias.")]
    query: String,
    #[arg(long = "limit-events", default_value_t = 30)]
    limit_events: usize,
    #[arg(long, help = "Kompakter menschenlesbarer Output statt JSON.")]
    pretty: bool,
}

#[derive(Debug, Args)]
struct TimelineArgs {
    #[arg(help = "Hero, Item, Ability oder Alias.")]
    query: String,
    #[arg(long = "limit-events", default_value_t = 2000)]
    limit_events: usize,
    #[arg(long, help = "Neueste Patches zuerst.")]
    descending: bool,
    #[arg(long, help = "Kompakter menschenlesbarer Output statt JSON.")]
    pretty: bool,
}

#[derive(Debug, Args)]
struct ReviewArgs {
    #[arg(help = "Hero, Item, Ability oder Alias.")]
    query: String,
    #[arg(long = "limit-events", default_value_t = 80)]
    limit_events: usize,
    #[arg(long, help = "Kompakter menschenlesbarer Output statt JSON.")]
    pretty: bool,
    #[arg(long = "prompt-only", help = "Nur den deutschen Prompt-Entwurf ausgeben.")]
    prompt_only: bool,
}

#[derive(Debug, Args)]
struct AskContextArgs {
    query: String,
    #[arg(long = "limit-events", default_value_t = 80)]
    limit_events: usize,
    #[arg(long = "include-unverified")]
    include_unverified: bool,
    #[arg(long = "max-claims", default_value_t = 12)]
    max_claims: usize,
    #[arg(long = "prompt-only")]
    prompt_only: bool,
    #[arg(long)]
    pretty: bool,
}

#[derive(Debug, Args)]
struct LineageArgs {
    #[arg(help = "Optionaler Hero, Item, Ability oder alter Name.")]
    query: Option<String>,
    #[arg(long, help = "Kompakter menschenlesbarer Output statt JSON.")]
    pretty: bool,
    #[arg(long, default_value_t = 50)]
    limit: usize,
}

#[derive(Debug, Args)]
struct LegacyArgs {
    #[arg(help = "Optionaler alter Name.")]
    query: Option<String>,
    #[arg(long, help = "Kompakter menschenlesbarer Output statt JSON.")]
    pretty: bool,
    #[arg(long, default_value_t = 50)]
    limit: usize,
}

#[derive(Debug, Args)]
struct BuildArgs {
    #[arg(help = "Hero-Name.")]
    query: String,
    #[arg(long = "limit-events", default_value_t = 80)]
    limit_events: usize,
    #[arg(long, help = "Kompakter menschenlesbarer Output statt JSON.")]
    pretty: bool,
}

#[derive(Debug, Args)]
struct BuildContextArgs {
    hero: String,
    #[arg(long, value_enum)]
    playstyle: Option<BuildPlaystyle>,
}

#[derive(Debug, Args)]
struct BuildEvalArgs {
    hero: String,
    #[arg(long, value_enum)]
    playstyle: Option<BuildPlaystyle>,
}

#[derive(Debug, Args)]
struct ItemArgs {
    #[arg(help = "Item-Name.")]
    query: String,
    #[arg(long, help = "Kompakter menschenlesbarer Output statt JSON.")]
    pretty: bool,
}

#[derive(Debug, Subcommand)]
enum LearnCommands {
    #[command(name = "import-steam-builds", about = "Importiert GC/Steam Hero-Builds aus der Steam-Bot-DB.")]
    ImportSteamBuilds(ImportSteamBuildsArgs),
    #[command(name = "list-builds", about = "Listet importierte Trainings-Builds.")]
    ListBuilds(ListBuildsArgs),
    #[command(name = "analyze-build", about = "Laesst MiniMax einen importierten Build als Trainingsbeispiel analysieren.")]
    AnalyzeBuild(AnalyzeBuildArgs),
    #[command(name = "analyze-next", about = "Analysiert automatisch die naechsten noch offenen Trainings-Builds.")]
    AnalyzeNext(AnalyzeNextBuildsArgs),
}

#[derive(Debug, Args)]
struct ImportSteamBuildsArgs {
    #[arg(long = "db-path", help = "Pfad zur Steam-Bot SQLite DB mit hero_build_sources.")]
    db_path: Option<PathBuf>,
    #[arg(long, help = "Optional nur ein Hero.")]
    hero: Option<String>,
    #[arg(long, default_value_t = 0, help = "Steam Build-Sprache, default Englisch=0.")]
    language: i64,
    #[arg(long = "limit-per-hero", default_value_t = 10)]
    limit_per_hero: usize,
    #[arg(long)]
    pretty: bool,
}

#[derive(Debug, Args)]
struct ListBuildsArgs {
    #[arg(long)]
    hero: Option<String>,
    #[arg(long, default_value_t = 25)]
    limit: usize,
    #[arg(long)]
    pretty: bool,
}

#[derive(Debug, Args)]
struct AnalyzeBuildArgs {
    #[arg(help = "ID aus learned_builds.")]
    build_id: i64,
    #[arg(long)]
    model: Option<String>,
    #[arg(long = "max-completion-tokens")]
    max_completion_tokens: Option<u64>,
    #[arg(long)]
    temperature: Option<f64>,
    #[arg(long = "top-p")]
    top_p: Option<f64>,
    #[arg(long = "dry-run")]
    dry_run: bool,
    #[arg(long)]
    pretty: bool,
}

#[derive(Debug, Args)]
struct AnalyzeNextBuildsArgs {
    #[arg(long, help = "Optional nur ein Hero.")]
    hero: Option<String>,
    #[arg(long, default_value_t = 5)]
    limit: usize,
    #[arg(long)]
    model: Option<String>,
    #[arg(long = "max-completion-tokens")]
    max_completion_tokens: Option<u64>,
    #[arg(long)]
    temperature: Option<f64>,
    #[arg(long = "top-p")]
    top_p: Option<f64>,
    #[arg(long = "dry-run")]
    dry_run: bool,
    #[arg(long = "delay-seconds", default_value_t = 2.0)]
    delay_seconds: f64,
    #[arg(long)]
    pretty: bool,
}

#[derive(Debug, Subcommand)]
enum PlayerCommands {
    #[command(name = "list-matches", about = "Listet importierte Statlocker Player-Matches.")]
    ListMatches(PlayerListMatchesArgs),
    #[command(name = "match-context", about = "Baut einen MiniMax-tauglichen Player-Match-Kontext ohne Modellaufruf.")]
    MatchContext(PlayerMatchContextArgs),
    #[command(name = "analyze-match", about = "Laesst MiniMax ein Player-Match als Entscheidungsbeispiel analysieren.")]
    AnalyzeMatch(PlayerAnalyzeMatchArgs),
    #[command(name = "analyze-next", about = "Analysiert automatisch die naechsten offenen Player-Matches.")]
    AnalyzeNext(PlayerAnalyzeNextArgs),
}

#[derive(Debug, Args)]
struct PlayerListMatchesArgs {
    #[arg(long = "account-id")]
    account_id: Option<String>,
    #[arg(long, default_value_t = 25)]
    limit: usize,
    #[arg(long)]
    pretty: bool,
}

#[derive(Debug, Args)]
struct PlayerMatchContextArgs {
    account_id: String,
    match_id: String,
    #[arg(long)]
    pretty: bool,
}

#[derive(Debug, Args)]
struct PlayerAnalyzeMatchArgs {
    account_id: String,
    match_id: String,
    #[arg(long)]
    model: Option<String>,
    #[arg(long = "max-completion-tokens")]
    max_completion_tokens: Option<u64>,
    #[arg(long)]
    temperature: Option<f64>,
    #[arg(long = "top-p")]
    top_p: Option<f64>,
    #[arg(long = "dry-run")]
    dry_run: bool,
    #[arg(long)]
    pretty: bool,
}

#[derive(Debug, Args)]
struct PlayerAnalyzeNextArgs {
    #[arg(long = "account-id")]
    account_id: Option<String>,
    #[arg(long, default_value_t = 5)]
    limit: usize,
    #[arg(long)]
    model: Option<String>,
    #[arg(long = "max-completion-tokens")]
    max_completion_tokens: Option<u64>,
    #[arg(long)]
    temperature: Option<f64>,
    #[arg(long = "top-p")]
    top_p: Option<f64>,
    #[arg(long = "dry-run")]
    dry_run: bool,
    #[arg(long = "delay-seconds", default_value_t = 2.0)]
    delay_seconds: f64,
    #[arg(long)]
    pretty: bool,
}

#[derive(Debug, Subcommand)]
enum YoutubeCommands {
    #[command(about = "Holt alle Videos aus der festen YouTube-Feedliste in die Queue.")]
    Discover(YoutubeDiscoverArgs),
    #[command(name = "import-transcripts", about = "Importiert lokale YouTube-Transkripte fuer die Queue.")]
    ImportTranscripts(YoutubeImportTranscriptsArgs),
    #[command(name = "fetch-transcripts", about = "Holt fehlende Untertitel/Auto-Captions mit yt-dlp.")]
    FetchTranscripts(YoutubeFetchTranscriptsArgs),
    #[command(name = "transcribe-local", about = "Laedt Audio und transkribiert fehlende Videos lokal mit faster-whisper.")]
    TranscribeLocal(YoutubeTranscribeLocalArgs),
    #[command(name = "analyze-next", about = "Laesst MiniMax die naechsten Videos bewerten und prueft Claims lokal.")]
    AnalyzeNext(YoutubeAnalyzeNextArgs),
    #[command(name = "auto-learn", about = "Discover -> Transcript-Import -> MiniMax -> lokale Claim-Pruefung.")]
    AutoLearn(YoutubeAutoLearnArgs),
    #[command(about = "Listet YouTube-Videos in der Lernqueue.")]
    Queue(YoutubeQueueArgs),
}

#[derive(Debug, Args)]
struct YoutubeDiscoverArgs {
    #[arg(long, default_value = "config/youtube_feeds.json")]
    config: PathBuf,
    #[arg(long = "max-videos-per-feed", default_value_t = 50)]
    max_videos_per_feed: usize,
    #[arg(long = "cache-ttl-seconds", default_value_t = 21600)]
    cache_ttl_seconds: u64,
    #[arg(long)]
    pretty: bool,
}

#[derive(Debug, Args)]
struct YoutubeImportTranscriptsArgs {
    #[arg(long = "transcript-dir", default_value = "data/youtube_transcripts")]
    transcript_dir: PathBuf,
    #[arg(long)]
    limit: Option<usize>,
    #[arg(long)]
    pretty: bool,
}

#[derive(Debug, Args)]
struct YoutubeFetchTranscriptsArgs {
    #[arg(long = "transcript-dir", default_value = "data/youtube_transcripts")]
    transcript_dir: PathBuf,
    #[arg(long, default_value_t = 20)]
    limit: usize,
    #[arg(long, default_value = "original")]
    languages: String,
    #[arg(long, value_enum, default_value_t = Order::Oldest)]
    order: Order,
    #[arg(long)]
    pretty: bool,
}

#[derive(Debug, Args)]
struct YoutubeTranscribeLocalArgs {
    #[arg(long = "audio-dir", default_value = "data/youtube_audio")]
    audio_dir: PathBuf,
    #[arg(long, default_value_t = 3)]
    limit: usize,
    #[arg(long = "model-size", default_value = "base")]
    model_size: String,
    #[arg(long, default_value = "auto")]
    device: String,
    #[arg(long = "compute-type", default_value = "int8")]
    compute_type: String,
    #[arg(long, value_enum, default_value_t = Order::Oldest)]
    order: Order,
    #[arg(long = "keep-audio")]
    keep_audio: bool,
    #[arg(long = "download-timeout-seconds", default_value_t = 900)]
    download_timeout_seconds: u64,
    #[arg(long)]
    pretty: bool,
}

#[derive(Debug, Args)]
struct YoutubeAnalyzeNextArgs {
    #[arg(long, default_value_t = 5)]
    limit: usize,
    #[arg(long)]
    model: Option<String>,
    #[arg(long = "max-completion-tokens")]
    max_completion_tokens: Option<u64>,
    #[arg(long)]
    temperature: Option<f64>,
    #[arg(long = "top-p")]
    top_p: Option<f64>,
    #[arg(long = "dry-run")]
    dry_run: bool,
    #[arg(long = "delay-seconds", default_value_t = 1.0)]
    delay_seconds: f64,
    #[arg(long)]
    pretty: bool,
}

#[derive(Debug, Args)]
struct YoutubeAutoLearnArgs {
    #[arg(long, default_value = "config/youtube_feeds.json")]
    config: PathBuf,
    #[arg(long = "transcript-dir", default_value = "data/youtube_transcripts")]
    transcript_dir: PathBuf,
    #[arg(long = "discover-limit", default_value_t = 50)]
    discover_limit: usize,
    #[arg(long = "analyze-limit", default_value_t = 5)]
    analyze_limit: usize,
    #[arg(long = "transcript-fetch-limit", default_value_t = 20)]
    transcript_fetch_limit: usize,
    #[arg(long, default_value = "original")]
    languages: String,
    #[arg(long = "local-transcribe")]
    local_transcribe: bool,
    #[arg(long = "local-transcribe-limit", default_value_t = 2)]
    local_transcribe_limit: usize,
    #[arg(long = "audio-dir", default_value = "data/youtube_audio")]
    audio_dir: PathBuf,
    #[arg(long = "asr-model-size", default_value = "base")]
    asr_model_size: String,
    #[arg(long = "asr-device", default_value = "auto")]
    asr_device: String,
    #[arg(long = "asr-compute-type", default_value = "int8")]
    asr_compute_type: String,
    #[arg(long = "keep-audio")]
    keep_audio: bool,
    #[arg(long = "cache-ttl-seconds", default_value_t = 21600)]
    cache_ttl_seconds: u64,
    #[arg(long)]
    model: Option<String>,
    #[arg(long = "max-completion-tokens")]
    max_completion_tokens: Option<u64>,
    #[arg(long)]
    temperature: Option<f64>,
    #[arg(long = "top-p")]
    top_p: Option<f64>,
    #[arg(long = "no-fetch-transcripts")]
    no_fetch_transcripts: bool,
    #[arg(long = "dry-run")]
    dry_run: bool,
    #[arg(long = "delay-seconds", default_value_t = 1.0)]
    delay_seconds: f64,
    #[arg(long)]
    pretty: bool,
}

#[derive(Debug, Args)]
struct YoutubeQueueArgs {
    #[arg(long)]
    status: Option<String>,
    #[arg(long, default_value_t = 25)]
    limit: usize,
    #[arg(long)]
    pretty: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Order {
    Oldest,
    Newest,
}

#[derive(Debug, Subcommand)]
enum AnalysisCommands {
    #[command(name = "save-review", about = "Speichert einen Review-Kontext ohne Modellaufruf.")]
    SaveReview(AnalysisSaveReviewArgs),
    #[command(name = "run-minimax", about = "Ruft MiniMax fuer einen Review-Kontext auf und speichert das Ergebnis.")]
    RunMinimax(AnalysisRunMinimaxArgs),
    #[command(name = "list", about = "Listet gespeicherte Analyse-Notizen.")]
    List(AnalysisListArgs),
}

#[derive(Debug, Args)]
struct AnalysisSaveReviewArgs {
    #[arg(help = "Hero, Item, Ability oder Alias.")]
    query: String,
    #[arg(long = "limit-events", default_value_t = 80)]
    limit_events: usize,
    #[arg(long = "result-text", help = "Optionaler fertiger Analyse-Text.")]
    result_text: Option<String>,
    #[arg(long, help = "Optionaler Modellname fuer fertige Analyse.")]
    model: Option<String>,
    #[arg(long)]
    confidence: Option<f64>,
    #[arg(long)]
    pretty: bool,
}

#[derive(Debug, Args)]
struct AnalysisRunMinimaxArgs {
    #[arg(help = "Hero, Item, Ability oder Alias.")]
    query: String,
    #[arg(long = "limit-events", default_value_t = 80)]
    limit_events: usize,
    #[arg(long)]
    model: Option<String>,
    #[arg(long = "max-completion-tokens")]
    max_completion_tokens: Option<u64>,
    #[arg(long)]
    temperature: Option<f64>,
    #[arg(long = "top-p")]
    top_p: Option<f64>,
    #[arg(long = "dry-run", help = "Baut nur den MiniMax-Request ohne API-Aufruf.")]
    dry_run: bool,
    #[arg(long)]
    pretty: bool,
}

#[derive(Debug, Args)]
struct AnalysisListArgs {
    #[arg(long)]
    query: Option<String>,
    #[arg(long, default_value_t = 25)]
    limit: usize,
    #[arg(long)]
    pretty: bool,
}

#[derive(Debug, Subcommand)]
enum PullCommands {
    #[command(about = "Zieht Deadlock Assets API Daten.")]
    Assets(PullAssetsArgs),
    #[command(name = "deadlock-data", about = "Zieht trusted deadlock-data aus GitHub.")]
    DeadlockData(PullDeadlockDataArgs),
    #[command(name = "build-data")]
    BuildData(PullBuildDataArgs),
    #[command(about = "Importiert bestehende Patchnotes aus der zentralen Bot-DB.")]
    Patchnotes(PullPatchnotesArgs),
    #[command(about = "Zieht gezielt Statlocker WPA-/Leaderboard-Daten.")]
    Statlocker(Box<PullStatlockerArgs>),
}

#[derive(Debug, Args)]
struct PullAssetsArgs {
    #[arg(
        long,
        action = clap::ArgAction::Append,
        value_parser = [
            "items",
            "heroes",
            "heroes_all",
            "raw_items",
            "raw_heroes",
            "ranks",
            "colors",
            "build_tags",
            "npc_units",
        ],
        help = "Endpoint auswaehlen. Mehrfach nutzbar. Default: items/heroes/raw_items/raw_heroes."
    )]
    kind: Vec<String>,
}

#[derive(Debug, Args)]
struct PullDeadlockDataArgs {
    #[arg(long = "repo-dir", help = "Lokaler Cache. Standard: data/external/deadlock-data.")]
    repo_dir: Option<PathBuf>,
    #[arg(long = "no-git-update", help = "Nutze vorhandenen Cache ohne git pull.")]
    no_git_update: bool,
}

#[derive(Debug, Args)]
struct PullBuildDataArgs {
    #[arg(long)]
    hero: String,
}

#[derive(Debug, Args)]
struct PullPatchnotesArgs {
    #[arg(long = "db-path")]
    db_path: Option<PathBuf>,
}

#[derive(Debug, Args)]
struct PullStatlockerArgs {
    #[arg(
        long,
        action = clap::ArgAction::Append,
        value_parser = [
            "wpa-patches",
            "wpa-items",
            "leaderboard",
            "player-profile",
            "player-matches",
            "match-detail",
            "player-build-analysis",
            "leaderboard-player-matches",
        ],
        help = "Mehrfach nutzbar. Default: wpa-patches, wpa-items, leaderboard."
    )]
    kind: Vec<String>,
    #[arg(long, help = "z.B. patch_129989 oder 129989. Default: neuester Statlocker-WPA-Patch.")]
    patch: Option<String>,
    #[arg(long, default_value = "all", help = "all oder Hero-Name, z.B. Mo & Krill.")]
    hero: String,
    #[arg(long = "min-sample-size", default_value_t = 500)]
    min_sample_size: u64,
    #[arg(long, default_value = "rank_10,rank_11")]
    rank: String,
    #[arg(long = "leaderboard-page", default_value_t = 1)]
    leaderboard_page: u64,
    #[arg(long = "leaderboard-page-size", default_value_t = 100)]
    leaderboard_page_size: u64,
    #[arg(long = "account-id", help = "Statlocker/Steam accountId fuer Player-Endpunkte.")]
    account_id: Option<String>,
    #[arg(long = "match-id", help = "Match-ID fuer Matchdetail-Endpunkt.")]
    match_id: Option<String>,
    #[arg(long = "hero-id", help = "Hero-ID fuer Player-Build-Analysis.")]
    hero_id: Option<String>,
    #[arg(long = "players-from-leaderboard", default_value_t = 5)]
    players_from_leaderboard: u64,
    #[arg(long = "matches-per-player", default_value_t = 6)]
    matches_per_player: u64,
    #[arg(long = "include-match-details")]
    include_match_details: bool,
    #[arg(long = "include-build-analysis")]
    include_build_analysis: bool,
    #[arg(long = "game-mode", value_enum, default_value_t = GameMode::All)]
    game_mode: GameMode,
    #[arg(long = "delay-seconds", default_value_t = 1.0)]
    delay_seconds: f64,
    #[arg(long = "cache-ttl-seconds", default_value_t = 21600)]
    cache_ttl_seconds: u64,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum GameMode {
    All,
    Standard,
    Brawl,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum BuildPlaystyle {
    Weapon,
    Spirit,
    Tank,
}

impl BuildPlaystyle {
    fn as_str(self) -> &'static str {
        match self {
            Self::Weapon => "weapon",
            Self::Spirit => "spirit",
            Self::Tank => "tank",
        }
    }
}

#[derive(Debug, Subcommand)]
enum NormalizeCommands {
    #[command(about = "Baut entities und entity_aliases.")]
    Entities(NormalizeEntitiesArgs),
    #[command(name = "sheet-stats", about = "Baut normalisierte Hero-Stats aus dem Google Sheet.")]
    SheetStats(NormalizeSheetStatsArgs),
    #[command(name = "sheet-tabs", about = "Baut normalisierte Tabellen fuer Hero-Rankings, Boons/AP und Raw-Heroes.")]
    SheetTabs(NormalizeSheetTabsArgs),
    #[command(name = "resolve-gaps", about = "Loest offene Entity-Luecken konservativ neu auf.")]
    ResolveGaps(ResolveGapsArgs),
}

#[derive(Debug, Args)]
struct NormalizeEntitiesArgs {
    #[arg(long, help = "Loescht normalisierte Entities vorher.")]
    rebuild: bool,
}

#[derive(Debug, Args)]
struct NormalizeSheetStatsArgs {
    #[arg(long, help = "Loescht Sheet-Stat-Tabellen vorher.")]
    rebuild: bool,
}

#[derive(Debug, Args)]
struct NormalizeSheetTabsArgs {
    #[arg(long, help = "Loescht Tab-Tabellen vorher.")]
    rebuild: bool,
}

#[derive(Debug, Args)]
struct ResolveGapsArgs {
    #[arg(long = "dry-run", help = "Nur Report erzeugen, keine DB-Aenderungen schreiben.")]
    dry_run: bool,
}

#[derive(Debug, Subcommand)]
enum ParseCommands {
    #[command(about = "Parst Patchnotes zu patch_events.")]
    Patchnotes(ParsePatchnotesArgs),
}

#[derive(Debug, Args)]
struct ParsePatchnotesArgs {
    #[arg(long, help = "Loescht patch_events vorher neu.")]
    rebuild: bool,
}

#[derive(Debug, Subcommand)]
enum EnrichCommands {
    #[command(name = "patch-events", about = "Baut Details aus patch_events.")]
    PatchEvents(EnrichPatchEventsArgs),
    #[command(about = "Baut Rename-/Rework-Lineage aus patch_events.")]
    Lineage(EnrichLineageArgs),
    #[command(name = "legacy-entities", about = "Baut alte/entfernte Entities aus patch_events.")]
    LegacyEntities(EnrichLegacyEntitiesArgs),
    #[command(name = "patch-impact", about = "Analysiert die Patch-Entwicklung von Entities.")]
    PatchImpact(PatchImpactArgs),
    #[command(name = "meta-trends", about = "Analysiert aktuelle Meta-Trends.")]
    MetaTrends,
}

#[derive(Debug, Args)]
struct EnrichPatchEventsArgs {
    #[arg(long, help = "Loescht Enrichments vorher.")]
    rebuild: bool,
}

#[derive(Debug, Args)]
struct EnrichLineageArgs {
    #[arg(long, help = "Loescht Lineage vorher.")]
    rebuild: bool,
}

#[derive(Debug, Args)]
struct EnrichLegacyEntitiesArgs {
    #[arg(long, help = "Loescht Legacy-Entities vorher.")]
    rebuild: bool,
}

#[derive(Debug, Args)]
struct PatchImpactArgs {
    #[arg(long, help = "Optional: Beschraenken auf diesen Hero.")]
    hero: Option<String>,
    #[arg(long, default_value_t = 10, help = "Anzahl der abzuarbeitenden Entities.")]
    limit: usize,
    #[arg(long = "dry-run", help = "Nur Prompt/Request bauen (benoetigt --hero), kein Modellaufruf.")]
    dry_run: bool,
}

#[derive(Debug, Subcommand)]
enum PgCommands {
    #[command(name = "import-patchnote", about = "Importiert exakt einen Patchnote-Eintrag direkt nach brain.* in Postgres.")]
    ImportPatchnote(PgImportPatchnoteArgs),
}

#[derive(Debug, Args)]
struct PgImportPatchnoteArgs {
    #[arg(long = "patch-id", help = "changelog_posts.id")]
    patch_id: i64,
    #[arg(long = "dsn-env", default_value = "DEADLOCK_CENTRAL_DSN")]
    dsn_env: String,
    #[arg(long = "dry-run", help = "Nur Datensatz lesen/parsen; keine Schreibzugriffe in brain-Tabellen.")]
    dry_run: bool,
}

fn main() {
    if let Err(error) = run_from_cli() {
        eprintln!("{error:#}");
        process::exit(1);
    }
}

fn run_from_cli() -> Result<()> {
    let cli = Cli::parse();
    // Ein Tokio-Runtime am Top-Level nach dem PG-Cutover: `run` ist async und
    // teilt sich einen `PgPool` fuer alle Befehle, statt pro Befehl einen eigenen
    // Runtime oder eine eigene Verbindung hochzuziehen. Der synchrone
    // `pg import-patchnote`-Pfad wird bewusst auf einen Blocking-Thread ausgelagert.
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    runtime.block_on(run(cli))
}

async fn run(cli: Cli) -> Result<()> {
    let Cli { db: db_path, command } = cli;
    let mut settings = config::load_settings()?;
    if let Some(path) = db_path {
        // --db/SQLite obsolet nach PG-Cutover, Entfernung in Phase 6.
        settings.db_path = path;
    }
    let command = match command {
        Commands::Pg { target } => {
            // `pg import-patchnote` nutzt den synchronen `postgres`-Crate, der
            // intern selbst einen Tokio-Runtime startet. Direkt im async-Kontext
            // aufgerufen wuerde das "runtime within a runtime" paniken -> daher
            // auf einen Blocking-Thread auslagern.
            return tokio::task::spawn_blocking(move || run_pg(target)).await?;
        }
        Commands::Entities(args) => {
            return pg_entities::run(args).await;
        }
        other => other,
    };
    prepare_dirs(&settings)?;
    // Ein PgPool fuer die gesamte Befehlsausfuehrung (DSN aus DEADLOCK_CENTRAL_DSN).
    let pool = deadlock_brain_core::pg::pg_pool().await?;

    match command {
        Commands::Status => print_status(&pool, &settings).await,
        Commands::Context(args) => {
            let result = dbrain_retrieval::build_entity_context(
                &pool,
                &args.query,
                usize_to_i64(args.limit_events),
            )
            .await?;
            if args.pretty {
                print_context(&result);
                Ok(())
            } else {
                print_json(&result)
            }
        }
        Commands::Timeline(args) => {
            let result = dbrain_retrieval::build_entity_timeline(
                &pool,
                &args.query,
                usize_to_i64(args.limit_events),
                !args.descending,
            )
            .await?;
            if args.pretty {
                print_timeline(&result);
                Ok(())
            } else {
                print_json(&result)
            }
        }
        Commands::Review(args) => {
            let result = dbrain_retrieval::build_review_context(
                &pool,
                &args.query,
                usize_to_i64(args.limit_events),
            )
            .await?;
            if args.prompt_only {
                println!("{}", str_value(get(&result, "prompt_de")).unwrap_or_default());
                Ok(())
            } else if args.pretty {
                print_review_context(&result);
                Ok(())
            } else {
                print_json(&result)
            }
        }
        Commands::AskContext(args) => {
            let result = dbrain_retrieval::ask_context(
                &pool,
                &args.query,
                &dbrain_retrieval::AskContextOptions {
                    limit_events: usize_to_i64(args.limit_events),
                    include_unverified: args.include_unverified,
                    max_claims: args.max_claims,
                },
            )
            .await?;
            if args.prompt_only {
                println!("{}", str_value(get(&result, "prompt")).unwrap_or_default());
                Ok(())
            } else {
                print_json(&result)
            }
        }
        Commands::Quality(args) => {
            let result = dbrain_retrieval::run_quality_checks(&pool).await?;
            if args.pretty {
                print_quality_report(&result);
                Ok(())
            } else {
                print_json(&result)
            }
        }
        Commands::Lineage(args) => {
            let rows = load_lineage(&pool, args.query.as_deref(), args.limit).await?;
            if args.pretty {
                print_lineage(&rows, args.query.as_deref());
                Ok(())
            } else {
                print_json(&json!({"query": args.query, "lineage": rows}))
            }
        }
        Commands::Legacy(args) => {
            let rows = load_legacy(&pool, args.query.as_deref(), args.limit).await?;
            if args.pretty {
                print_legacy(&rows, args.query.as_deref());
                Ok(())
            } else {
                print_json(&json!({"query": args.query, "legacy_entities": rows}))
            }
        }
        Commands::Build(args) => {
            let result = dbrain_learn::build_hero_build_context(
                &pool,
                &args.query,
                &[],
                args.limit_events,
            )
            .await?;
            if args.pretty {
                print_build_context(&result);
                Ok(())
            } else {
                print_json(&result)
            }
        }
        Commands::BuildContext(args) => {
            let playstyle = args.playstyle.map(BuildPlaystyle::as_str);
            let result = dbrain_builds::build_context(&pool, &args.hero, playstyle).await?;
            print_json(&result)
        }
        Commands::BuildEval(args) => run_build_eval(&pool, &settings, args).await,
        Commands::Item(args) => {
            let result = dbrain_retrieval::build_item_context(&pool, &args.query).await?;
            if args.pretty {
                print_item_context(&result);
                Ok(())
            } else {
                print_json(&result)
            }
        }
        Commands::Learn { target } => run_learn(&settings, target).await,
        Commands::Player { target } => run_player(&settings, target).await,
        Commands::Youtube { target } => print_json(&youtube_deferral(&target)),
        Commands::Analysis { target } => run_analysis(&pool, &settings, target).await,
        Commands::Pull { source } => run_pull(&pool, &settings, source).await,
        Commands::RefreshSheet => run_refresh_sheet(&pool, &settings).await,
        Commands::Normalize { target } => run_normalize(&pool, target).await,
        Commands::Parse { target } => run_parse(&pool, target).await,
        Commands::Enrich { target } => run_enrich(&pool, &settings, target).await,
        Commands::Entities(_) => {
            unreachable!("PG-Entities werden vor SQLite-Einrichtung ausgefuehrt.")
        }
        Commands::Pg { target: _ } => {
            unreachable!("PG-Commands werden vor SQLite-Einrichtung ausgefuehrt.")
        }
    }
}

fn run_pg(target: PgCommands) -> Result<()> {
    match target {
        PgCommands::ImportPatchnote(args) => {
            print_json(&pg_patchnotes::import_patchnote(
                &pg_patchnotes::ImportPatchnoteOptions {
                    patch_id: args.patch_id,
                    dsn_env: args.dsn_env,
                    dry_run: args.dry_run,
                },
            )?)
        }
    }
}

async fn run_build_eval(pool: &PgPool, settings: &Settings, args: BuildEvalArgs) -> Result<()> {
    let playstyle = args.playstyle.map(BuildPlaystyle::as_str);
    let build_context = dbrain_builds::build_context(pool, &args.hero, playstyle).await?;
    let config = MiniMaxConfig::from_settings(settings);
    if !config.api_key_present() {
        return print_json(&json!({
            "build_context": build_context,
            "notice": "MiniMax environment is not configured; skipping narration."
        }));
    }

    let client = MiniMaxClient::new(config)?;
    let request = build_narration::build_narration_request(&build_context, client.config())?;
    let response = client.chat(&request)?;
    let narration = extract_minimax_text(&response);
    let known_item_names = load_known_item_names(pool).await?;
    let validation =
        build_narration::validate_narration(&narration, &build_context, &known_item_names);
    print_json(&json!({
        "build_context": build_context,
        "narration": narration,
        "validation": validation
    }))
}

async fn load_known_item_names(pool: &PgPool) -> Result<BTreeSet<String>> {
    let names = sqlx::query_scalar::<_, String>(
        "SELECT name FROM brain.item_catalog WHERE name IS NOT NULL AND name <> ''",
    )
    .fetch_all(pool)
    .await?;
    let mut set = BTreeSet::new();
    for name in names {
        let trimmed = name.trim();
        if !trimmed.is_empty() {
            set.insert(trimmed.to_string());
        }
    }
    Ok(set)
}

async fn run_learn(settings: &Settings, target: LearnCommands) -> Result<()> {
    let pool = deadlock_brain_core::pg::pg_pool().await?;
    match target {
        LearnCommands::ImportSteamBuilds(args) => {
            let result = dbrain_learn::learn_import_steam_builds(
                &pool,
                dbrain_learn::LearnImportSteamBuildsOptions {
                    steam_db_path: Some(args.db_path.unwrap_or_else(|| settings.central_deadlock_db_path.clone())),
                    hero: args.hero,
                    language: args.language,
                    limit_per_hero: usize_to_i64(args.limit_per_hero),
                },
            )
            .await?;
            if args.pretty {
                print_learn_result("import-steam-builds", &result);
                Ok(())
            } else {
                print_json(&result)
            }
        }
        LearnCommands::ListBuilds(args) => {
            let result = Value::Array(dbrain_learn::learn_list_builds(
                &pool,
                args.hero.as_deref(),
                usize_to_i64(args.limit),
            )
            .await?);
            if args.pretty {
                print_learn_result("list-builds", &result);
                Ok(())
            } else {
                print_json(&result)
            }
        }
        LearnCommands::AnalyzeBuild(args) => {
            let config = minimax_config(
                settings,
                args.model,
                args.max_completion_tokens,
                args.temperature,
                args.top_p,
            );
            let result = dbrain_learn::learn_analyze_build(
                &pool,
                dbrain_learn::LearnAnalyzeBuildOptions {
                    build_id: args.build_id,
                    config,
                    dry_run: args.dry_run,
                    include_request: true,
                },
            )
            .await?;
            if args.pretty {
                print_learn_result("analyze-build", &result);
                Ok(())
            } else {
                print_json(&result)
            }
        }
        LearnCommands::AnalyzeNext(args) => {
            let config = minimax_config(
                settings,
                args.model,
                args.max_completion_tokens,
                args.temperature,
                args.top_p,
            );
            let result = dbrain_learn::learn_analyze_next(
                &pool,
                dbrain_learn::LearnAnalyzeNextOptions {
                    hero: args.hero,
                    limit: usize_to_i64(args.limit),
                    config,
                    dry_run: args.dry_run,
                    delay_seconds: args.delay_seconds,
                },
            )
            .await?;
            if args.pretty {
                print_learn_result("analyze-next", &result);
                Ok(())
            } else {
                print_json(&result)
            }
        }
    }
}

async fn run_player(settings: &Settings, target: PlayerCommands) -> Result<()> {
    let pool = deadlock_brain_core::pg::pg_pool().await?;
    match target {
        PlayerCommands::ListMatches(args) => {
            let result = Value::Array(dbrain_learn::player_list_matches(
                &pool,
                args.account_id.as_deref(),
                usize_to_i64(args.limit),
            )
            .await?);
            if args.pretty {
                print_player_result("list-matches", &result);
                Ok(())
            } else {
                print_json(&result)
            }
        }
        PlayerCommands::MatchContext(args) => {
            let result = dbrain_learn::player_match_context(&pool, &args.account_id, &args.match_id).await?;
            if args.pretty {
                print_player_result("match-context", &result);
                Ok(())
            } else {
                print_json(&result)
            }
        }
        PlayerCommands::AnalyzeMatch(args) => {
            let config = minimax_config(
                settings,
                args.model,
                args.max_completion_tokens,
                args.temperature,
                args.top_p,
            );
            let result = dbrain_learn::player_analyze_match(
                &pool,
                dbrain_learn::PlayerAnalyzeMatchOptions {
                    account_id: args.account_id,
                    match_id: args.match_id,
                    config,
                    dry_run: args.dry_run,
                    include_request: true,
                },
            )
            .await?;
            if args.pretty {
                print_player_result("analyze-match", &result);
                Ok(())
            } else {
                print_json(&result)
            }
        }
        PlayerCommands::AnalyzeNext(args) => {
            let config = minimax_config(
                settings,
                args.model,
                args.max_completion_tokens,
                args.temperature,
                args.top_p,
            );
            let result = dbrain_learn::player_analyze_next(
                &pool,
                dbrain_learn::PlayerAnalyzeNextOptions {
                    account_id: args.account_id,
                    limit: usize_to_i64(args.limit),
                    config,
                    dry_run: args.dry_run,
                    delay_seconds: args.delay_seconds,
                },
            )
            .await?;
            if args.pretty {
                print_player_result("analyze-next", &result);
                Ok(())
            } else {
                print_json(&result)
            }
        }
    }
}

async fn run_analysis(pool: &PgPool, settings: &Settings, target: AnalysisCommands) -> Result<()> {
    match target {
        AnalysisCommands::SaveReview(args) => {
            let result = dbrain_retrieval::analysis_save_review_for_query(
                pool,
                &args.query,
                usize_to_i64(args.limit_events),
                args.result_text.as_deref(),
                args.model.as_deref(),
                args.confidence,
            )
            .await?;
            if args.pretty {
                print_analysis_result("save-review", &result);
                Ok(())
            } else {
                print_json(&result)
            }
        }
        AnalysisCommands::RunMinimax(args) => {
            let config = minimax_config(
                settings,
                args.model,
                args.max_completion_tokens,
                args.temperature,
                args.top_p,
            );
            let result = dbrain_retrieval::analysis_run_minimax(
                pool,
                &args.query,
                dbrain_retrieval::AnalysisRunMinimaxOptions {
                    limit_events: usize_to_i64(args.limit_events),
                    config,
                    dry_run: args.dry_run,
                },
            )
            .await?;
            if args.pretty {
                print_analysis_result("run-minimax", &result);
                Ok(())
            } else {
                print_json(&result)
            }
        }
        AnalysisCommands::List(args) => {
            let result = dbrain_retrieval::analysis_list(
                pool,
                args.query.as_deref(),
                usize_to_i64(args.limit),
            )
            .await?;
            if args.pretty {
                print_analysis_result("list", &result);
                Ok(())
            } else {
                print_json(&result)
            }
        }
    }
}

async fn run_pull(pool: &PgPool, settings: &Settings, source: PullCommands) -> Result<()> {
    let http = http_client(settings)?;
    match source {
        PullCommands::Assets(args) => {
            let result = dbrain_sources::pull_assets(
                &settings.raw_dir,
                &http,
                dbrain_sources::PullAssetsOptions { kinds: args.kind },
            )
            .await?;
            print_json(&result)
        }
        PullCommands::DeadlockData(args) => {
            let repo_dir = args
                .repo_dir
                .unwrap_or_else(|| settings.data_dir.join("external/deadlock-data"));
            let pull = dbrain_sources::pull_deadlock_data(
                &settings.raw_dir,
                dbrain_sources::PullDeadlockDataOptions {
                    repo_dir,
                    update_repo: !args.no_git_update,
                },
            )
            .await?;
            let patch_events = dbrain_normalize::parse_patchnotes(pool, false).await?;
            print_json(&json!({
                "pull": pull,
                "patch_events": patch_events,
            }))
        }
        PullCommands::BuildData(args) => {
            let result = dbrain_builds::sync_build_data(
                pool,
                dbrain_builds::BuildDataSyncOptions::new(args.hero, settings.user_agent.clone()),
            )
            .await?;
            print_json(&result)
        }
        PullCommands::Patchnotes(_) => {
            // Patchnotes werden seit dem PG-Cutover direkt aus der zentralen
            // Postgres (`patchnotes`-Schema) gelesen; das alte `--db-path` (SQLite)
            // ist obsolet und wird in Phase 6 aus der CLI entfernt.
            let result = dbrain_sources::pull_patchnotes(
                &settings.raw_dir,
                dbrain_sources::PullPatchnotesOptions,
            )
            .await?;
            print_json(&result)
        }
        PullCommands::Statlocker(args) => {
            let result = dbrain_sources::pull_statlocker(
                &settings.raw_dir,
                &http,
                dbrain_sources::PullStatlockerOptions {
                    kinds: args.kind,
                    patch: args.patch,
                    hero: args.hero,
                    min_sample_size: args.min_sample_size,
                    rank: args.rank,
                    leaderboard_page: args.leaderboard_page,
                    leaderboard_page_size: args.leaderboard_page_size,
                    account_id: args.account_id,
                    match_id: args.match_id,
                    hero_id: args.hero_id,
                    players_from_leaderboard: args.players_from_leaderboard,
                    matches_per_player: args.matches_per_player,
                    include_match_details: args.include_match_details,
                    include_build_analysis: args.include_build_analysis,
                    game_mode: game_mode_value(args.game_mode),
                    delay_seconds: args.delay_seconds,
                    cache_ttl_seconds: args.cache_ttl_seconds,
                },
            )
            .await?;
            print_json(&result)
        }
    }
}

async fn run_refresh_sheet(pool: &PgPool, settings: &Settings) -> Result<()> {
    let http = http_client(settings)?;
    let pull_wrapper = dbrain_sources::refresh_sheet(
        &settings.raw_dir,
        &http,
        dbrain_sources::RefreshSheetOptions {
            sheet_id: settings.sheet_id.clone(),
            gid: settings.sheet_gid.clone(),
        },
    )
    .await?;
    let pull = get(&pull_wrapper, "pull").cloned().unwrap_or(pull_wrapper);
    let stats = dbrain_normalize::normalize_sheet_stats(pool, true).await?;
    let tabs = dbrain_normalize::normalize_sheet_tabs(pool, true).await?;
    print_json(&json!({
        "pull": pull,
        "normalize_sheet_stats": stats,
        "normalize_sheet_tabs": tabs,
    }))
}

async fn run_normalize(pool: &PgPool, target: NormalizeCommands) -> Result<()> {
    match target {
        NormalizeCommands::Entities(args) => {
            print_json(&dbrain_normalize::normalize_entities(pool, args.rebuild).await?)
        }
        NormalizeCommands::SheetStats(args) => {
            print_json(&dbrain_normalize::normalize_sheet_stats(pool, args.rebuild).await?)
        }
        NormalizeCommands::SheetTabs(args) => {
            print_json(&dbrain_normalize::normalize_sheet_tabs(pool, args.rebuild).await?)
        }
        NormalizeCommands::ResolveGaps(args) => {
            print_json(&dbrain_normalize::resolve_gaps(pool, args.dry_run).await?)
        }
    }
}

async fn run_parse(pool: &PgPool, target: ParseCommands) -> Result<()> {
    match target {
        ParseCommands::Patchnotes(args) => {
            print_json(&dbrain_normalize::parse_patchnotes(pool, args.rebuild).await?)
        }
    }
}

async fn run_enrich(pool: &PgPool, settings: &Settings, target: EnrichCommands) -> Result<()> {
    match target {
        EnrichCommands::PatchEvents(args) => {
            print_json(
                &dbrain_enrich::build_patch_event_enrichments(pool, args.rebuild).await?,
            )
        }
        EnrichCommands::Lineage(args) => {
            print_json(&dbrain_normalize::enrich_lineage(pool, args.rebuild).await?)
        }
        EnrichCommands::LegacyEntities(args) => {
            print_json(&dbrain_normalize::enrich_legacy_entities(pool, args.rebuild).await?)
        }
        EnrichCommands::PatchImpact(args) => {
            print_json(&run_patch_impact(pool, settings, args).await?)
        }
        EnrichCommands::MetaTrends => {
            let config = minimax_config(settings, None, None, None, None);
            print_json(&dbrain_enrich::run_meta_trend_analysis(pool, &config).await?)
        }
    }
}

async fn run_patch_impact(
    pool: &PgPool,
    settings: &Settings,
    args: PatchImpactArgs,
) -> Result<Value> {
    let config = minimax_config(settings, None, None, None, None);
    if args.dry_run {
        let Some(hero) = args.hero.as_deref() else {
            return Err(anyhow!("Fuer --dry-run muss --hero angegeben werden."));
        };
        let entity_type = entity_type_for_name(pool, hero).await?;
        let context = dbrain_enrich::build_patch_impact_context(pool, hero, &entity_type).await?;
        let request_info = dbrain_enrich::build_patch_impact_request(&context, &config)?;
        return Ok(json!({
            "dry_run": true,
            "prompt": request_info.prompt_text,
            "request": serde_json::to_value(&request_info.request)?,
        }));
    }
    if let Some(hero) = args.hero.as_deref() {
        let entity_type = entity_type_for_name(pool, hero).await?;
        let context = dbrain_enrich::build_patch_impact_context(pool, hero, &entity_type).await?;
        let request_info = dbrain_enrich::build_patch_impact_request(&context, &config)?;
        let client = MiniMaxClient::new(config.clone())?;
        let response = client.chat(&request_info.request)?;
        let result_text = extract_minimax_text(&response);
        dbrain_enrich::save_patch_impact_note(
            pool,
            &context,
            &request_info.prompt_text,
            Some(&result_text),
            &config.model,
            "analysis_ready",
        )
        .await?;
        Ok(json!({"processed": 1, "success": 1, "failed": 0, "hero": hero}))
    } else {
        Ok(serde_json::to_value(
            dbrain_enrich::run_patch_impact_batch(pool, &config, args.limit).await?,
        )?)
    }
}

async fn entity_type_for_name(pool: &PgPool, name: &str) -> Result<String> {
    let value = sqlx::query_scalar::<_, String>(
        "SELECT entity_type FROM brain.entities WHERE canonical_name = $1 LIMIT 1",
    )
    .bind(name)
    .fetch_optional(pool)
    .await?;
    Ok(value.unwrap_or_else(|| "hero".to_string()))
}

fn youtube_deferral(target: &YoutubeCommands) -> Value {
    json!({
        "status": "deferred",
        "target": youtube_target_name(target),
        "message": "YouTube-Lernen läuft im Rust-Port über das eigene Binary `deadlock-brain-yt` (z. B. `deadlock-brain-yt ingest`).",
        "binary": "deadlock-brain-yt",
    })
}

fn youtube_target_name(target: &YoutubeCommands) -> &'static str {
    match target {
        YoutubeCommands::Discover(_) => "discover",
        YoutubeCommands::ImportTranscripts(_) => "import-transcripts",
        YoutubeCommands::FetchTranscripts(_) => "fetch-transcripts",
        YoutubeCommands::TranscribeLocal(_) => "transcribe-local",
        YoutubeCommands::AnalyzeNext(_) => "analyze-next",
        YoutubeCommands::AutoLearn(_) => "auto-learn",
        YoutubeCommands::Queue(_) => "queue",
    }
}

fn prepare_dirs(settings: &Settings) -> Result<()> {
    for dir in [&settings.data_dir, &settings.raw_dir, &settings.cache_dir] {
        fs::create_dir_all(dir)?;
    }
    Ok(())
}

fn http_client(settings: &Settings) -> Result<HttpClient> {
    HttpClient::new(settings.user_agent.clone(), settings.cache_dir.clone()).map_err(Into::into)
}

fn minimax_config(
    settings: &Settings,
    model: Option<String>,
    max_completion_tokens: Option<u64>,
    temperature: Option<f64>,
    top_p: Option<f64>,
) -> MiniMaxConfig {
    let mut config = MiniMaxConfig::from_settings(settings);
    if let Some(model) = model {
        config.model = model;
    }
    if let Some(max_completion_tokens) = max_completion_tokens {
        config.max_completion_tokens = max_completion_tokens;
    }
    if let Some(temperature) = temperature {
        config.temperature = temperature;
    }
    if let Some(top_p) = top_p {
        config.top_p = top_p;
    }
    config
}

fn game_mode_value(mode: GameMode) -> Option<String> {
    match mode {
        GameMode::All => None,
        GameMode::Standard => Some("standard".to_string()),
        GameMode::Brawl => Some("brawl".to_string()),
    }
}

fn usize_to_i64(value: usize) -> i64 {
    i64::try_from(value).unwrap_or(i64::MAX)
}

fn bounded_limit(value: usize) -> i64 {
    usize_to_i64(value).clamp(1, 500)
}

fn print_json<T: Serialize + ?Sized>(value: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

async fn print_status(pool: &PgPool, settings: &Settings) -> Result<()> {
    let status = dbrain_retrieval::status(pool).await?;
    println!("Project: {}", settings.project_root.display());
    println!("DB:      {}", settings.db_path.display());
    println!("Raw:     {}", settings.raw_dir.display());
    println!("Cache:   {}", settings.cache_dir.display());
    println!("Patch DB:{}", settings.central_deadlock_db_path.display());
    println!();
    println!("Source documents:");
    print_status_rows(get(&status, "source_documents"), |row| {
        format!(
            "  {}: {}",
            display_value(get(row, "source")),
            display_value(get(row, "documents"))
        )
    });
    println!();
    println!("Entity snapshots:");
    print_status_rows(get(&status, "entity_snapshots"), |row| {
        format!(
            "  {} / {}: {}",
            display_value(get(row, "source")),
            display_value(get(row, "entity_type")),
            display_value(get(row, "snapshots"))
        )
    });
    println!();
    println!("Patch events:");
    print_status_rows(get(&status, "patch_events"), |row| {
        format!(
            "  {} / {}: {}",
            display_value(get(row, "source_kind")),
            display_value(get(row, "entity_type")),
            display_value(get(row, "events"))
        )
    });
    println!();
    println!("Entities:");
    print_status_rows(get(&status, "entities"), |row| {
        format!(
            "  {}: {}",
            display_value(get(row, "entity_type")),
            display_value(get(row, "entities"))
        )
    });
    println!();
    println!("Derived data:");
    print_status_rows(get(&status, "derived_data"), |row| {
        format!(
            "  {}: {}",
            display_value(get(row, "table")),
            display_value(get(row, "count"))
        )
    });
    Ok(())
}

fn print_status_rows<F>(rows: Option<&Value>, render: F)
where
    F: Fn(&Value) -> String,
{
    let Some(rows) = rows.and_then(Value::as_array) else {
        println!("  none");
        return;
    };
    if rows.is_empty() {
        println!("  none");
        return;
    }
    for row in rows {
        println!("{}", render(row));
    }
}

async fn load_lineage(pool: &PgPool, query: Option<&str>, limit: usize) -> Result<Vec<Value>> {
    let max_rows = bounded_limit(limit);
    let rows = if let Some(query) = query.filter(|value| !value.trim().is_empty()) {
        let norm = dbrain_normalize::normalize_alias(query);
        sqlx::query_scalar::<_, Value>(
            r#"
            SELECT to_jsonb(t) FROM (
                SELECT *
                FROM brain.entity_lineage
                WHERE source_name_norm = $1
                   OR target_name_norm = $1
                   OR owner_name_norm = $1
                ORDER BY confidence DESC, id ASC
                LIMIT $2
            ) AS t
            "#,
        )
        .bind(norm)
        .bind(max_rows)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_scalar::<_, Value>(
            r#"
            SELECT to_jsonb(t) FROM (
                SELECT *
                FROM brain.entity_lineage
                ORDER BY patch_event_id DESC, id DESC
                LIMIT $1
            ) AS t
            "#,
        )
        .bind(max_rows)
        .fetch_all(pool)
        .await?
    };
    Ok(rows)
}

async fn load_legacy(pool: &PgPool, query: Option<&str>, limit: usize) -> Result<Vec<Value>> {
    let max_rows = bounded_limit(limit);
    let rows = if let Some(query) = query.filter(|value| !value.trim().is_empty()) {
        sqlx::query_scalar::<_, Value>(
            r#"
            SELECT to_jsonb(t) FROM (
                SELECT *
                FROM brain.legacy_entities
                WHERE canonical_name ILIKE $1 OR name_norm LIKE $2
                ORDER BY confidence DESC, event_count DESC, canonical_name
                LIMIT $3
            ) AS t
            "#,
        )
        .bind(format!("%{query}%"))
        .bind(format!("%{}%", query.to_lowercase()))
        .bind(max_rows)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_scalar::<_, Value>(
            r#"
            SELECT to_jsonb(t) FROM (
                SELECT *
                FROM brain.legacy_entities
                ORDER BY confidence DESC, event_count DESC, canonical_name
                LIMIT $1
            ) AS t
            "#,
        )
        .bind(max_rows)
        .fetch_all(pool)
        .await?
    };
    Ok(rows)
}

fn print_context(ctx: &Value) {
    if let Some(best_match) = get(ctx, "best_match").filter(|value| !value.is_null()) {
        println!(
            "Best match: {} / {} ({})",
            display_value(get(best_match, "entity_type")),
            display_value(get(best_match, "canonical_name")),
            display_value(get(best_match, "primary_external_id"))
        );
    } else {
        println!("Best match: none");
    }
    println!("Aliases: {}", array_len(get(ctx, "aliases")));
    let sheet = get(ctx, "sheet_stats").unwrap_or(&Value::Null);
    println!("Sheet stats: {}", if bool_value(get(sheet, "available")) { "yes" } else { "no" });
    let enrichments = get(ctx, "enrichments").unwrap_or(&Value::Null);
    println!(
        "Enrichments: {} / {} rows",
        if bool_value(get(enrichments, "available")) { "yes" } else { "no" },
        array_len(get(enrichments, "rows"))
    );
    println!("Patch events: {}", array_len(get(ctx, "patch_events")));
    if let Some(events) = get(ctx, "patch_events").and_then(Value::as_array) {
        for event in events.iter().take(10) {
            let date = str_value(get(event, "posted_at")).unwrap_or("unknown-date");
            let section = str_value(get(event, "section")).unwrap_or("Unsectioned");
            let entity = str_value(get(event, "entity_name"))
                .or_else(|| str_value(get(event, "entity_type")))
                .unwrap_or_default();
            println!(
                "- {} [{}] {} / {} / {}",
                date,
                display_value(get(event, "source_kind")),
                section,
                entity,
                display_value(get(event, "change_type"))
            );
            println!("  {}", display_value(get(event, "normalized_line")));
        }
    }
}

fn print_timeline(timeline: &Value) {
    if let Some(best_match) = get(timeline, "best_match").filter(|value| !value.is_null()) {
        println!(
            "Timeline: {} / {} ({})",
            display_value(get(best_match, "entity_type")),
            display_value(get(best_match, "canonical_name")),
            display_value(get(best_match, "primary_external_id"))
        );
    } else {
        println!("Timeline: {} (fallback)", display_value(get(timeline, "query")));
    }
    println!(
        "Patches: {} / Events: {}",
        display_value(get(timeline, "patch_count")),
        display_value(get(timeline, "event_count"))
    );
    if let Some(by_kind) = get(timeline, "impact_summary")
        .and_then(|impact| get(impact, "by_kind"))
        .and_then(Value::as_object)
    {
        if !by_kind.is_empty() {
            let parts = by_kind
                .iter()
                .map(|(key, value)| format!("{key}={}", display_value(Some(value))))
                .collect::<Vec<_>>();
            println!("Impact kinds: {}", parts.join(", "));
        }
    }
    if let Some(patches) = get(timeline, "patches").and_then(Value::as_array) {
        for patch in patches.iter().take(12) {
            let date = str_value(get(patch, "date")).unwrap_or("unknown-date");
            println!(
                "- {} [{}] {}",
                date,
                display_value(get(patch, "source")),
                display_value(get(patch, "title"))
            );
            if let Some(events) = get(patch, "events").and_then(Value::as_array) {
                for event in events.iter().take(8) {
                    let kind = str_value(get(event, "impact_kind")).unwrap_or("unknown");
                    let level = str_value(get(event, "impact_level")).unwrap_or("unknown");
                    let line = str_value(get(event, "normalized_line"))
                        .or_else(|| str_value(get(event, "raw_line")))
                        .unwrap_or_default();
                    println!("  - {kind}/{level}: {line}");
                }
            }
        }
    }
}

fn print_review_context(review_context: &Value) {
    let summary = get(review_context, "entity_summary").unwrap_or(&Value::Null);
    let name = str_value(get(summary, "name"))
        .or_else(|| str_value(get(review_context, "query")))
        .unwrap_or_default();
    let entity_type = str_value(get(summary, "entity_type")).unwrap_or("unknown");
    println!("Review context: {entity_type} / {name}");
    println!("Matched: {}", if bool_value(get(summary, "matched")) { "yes" } else { "no" });
    let timeline = get(review_context, "timeline_signals").unwrap_or(&Value::Null);
    println!("Events: {}", display_or(get(timeline, "event_count"), "0"));
    let latest = get(timeline, "latest_patch").unwrap_or(&Value::Null);
    if latest.as_object().map(|object| !object.is_empty()).unwrap_or(false) {
        println!(
            "Latest patch: {} / {}",
            str_value(get(latest, "posted_at")).unwrap_or("unknown-date"),
            display_value(get(latest, "patch_title"))
        );
    }
    let stats = get(review_context, "current_stat_hints").unwrap_or(&Value::Null);
    println!(
        "Sheet stats: {} / {} hints",
        if bool_value(get(stats, "available")) { "yes" } else { "no" },
        array_len(get(stats, "hints"))
    );
    if let Some(questions) = get(review_context, "open_questions").and_then(Value::as_array) {
        if !questions.is_empty() {
            println!("Open questions:");
            for question in questions.iter().take(10) {
                println!("- {}", display_value(Some(question)));
            }
        }
    }
}

fn print_quality_report(report: &Value) {
    let summary = get(report, "summary").unwrap_or(&Value::Null);
    println!(
        "Quality: {}",
        if bool_value(get(report, "ok")) {
            "ok"
        } else {
            "needs attention"
        }
    );
    println!(
        "Checks: {} / warnings={} / errors={}",
        display_or(get(summary, "checks"), "0"),
        display_or(get(summary, "warnings"), "0"),
        display_or(get(summary, "errors"), "0")
    );
    if let Some(checks) = get(report, "checks").and_then(Value::as_array) {
        for check in checks {
            if str_value(get(check, "severity")) == Some("ok") {
                continue;
            }
            println!(
                "- {} {}: {}",
                display_value(get(check, "severity")),
                display_value(get(check, "check")),
                display_or(get(check, "rows"), "0")
            );
            println!("  {}", display_value(get(check, "message")));
            if let Some(samples) = get(check, "samples").and_then(Value::as_array) {
                for sample in samples.iter().take(3) {
                    println!("  sample: {}", display_value(Some(sample)));
                }
            }
        }
    }
}

fn print_lineage(rows: &[Value], query: Option<&str>) {
    let label = query.unwrap_or("latest");
    println!("Lineage: {label} / rows={}", rows.len());
    for row in rows {
        let relation = display_value(get(row, "relation_type"));
        let source = display_value(get(row, "source_name"));
        let target = str_value(get(row, "target_name"));
        let owner = str_value(get(row, "owner_name"));
        let metadata = get(row, "metadata").unwrap_or(&Value::Null);
        let owner_text = owner.map(|value| format!(" / owner={value}")).unwrap_or_default();
        let target_text = target.map(|value| format!(" -> {value}")).unwrap_or_default();
        println!("- {relation}: {source}{target_text}{owner_text}");
        println!(
            "  {} / {}",
            str_value(get(metadata, "posted_at")).unwrap_or("unknown-date"),
            display_value(get(metadata, "patch_title"))
        );
        if let Some(line) = str_value(get(metadata, "line")).filter(|value| !value.is_empty()) {
            println!("  {line}");
        }
    }
}

fn print_legacy(rows: &[Value], query: Option<&str>) {
    let label = query.unwrap_or("top");
    println!("Legacy entities: {label} / rows={}", rows.len());
    for row in rows {
        println!(
            "- {}: {} events={} confidence={} status={}",
            display_value(get(row, "legacy_type")),
            display_value(get(row, "canonical_name")),
            display_value(get(row, "event_count")),
            display_value(get(row, "confidence")),
            display_value(get(row, "status"))
        );
        if let Some(sample) = get(row, "samples")
            .and_then(Value::as_array)
            .and_then(|samples| samples.first())
        {
            println!(
                "  {} / {}",
                str_value(get(sample, "posted_at")).unwrap_or("unknown-date"),
                display_value(get(sample, "patch_title"))
            );
            println!("  {}", display_value(get(sample, "line")));
        }
    }
}

fn print_build_context(ctx: &Value) {
    let hero = get(ctx, "hero").unwrap_or(&Value::Null);
    let economy = get(ctx, "economy").unwrap_or(&Value::Null);
    let build = get(ctx, "build").unwrap_or(&Value::Null);
    println!(
        "Build: {} / {}",
        display_value(get(hero, "name")),
        display_value(get(hero, "hero_type"))
    );
    if let Some(role) = str_value(get(hero, "role")).filter(|value| !value.is_empty()) {
        println!("Role: {role}");
    }
    if let Some(playstyle) = str_value(get(hero, "playstyle")).filter(|value| !value.is_empty()) {
        println!("Playstyle: {playstyle}");
    }
    let gameplan = get(hero, "inferred_gameplan").unwrap_or(&Value::Null);
    if gameplan.as_object().map(|object| !object.is_empty()).unwrap_or(false) {
        println!("Understood gameplan: {}", display_value(get(gameplan, "summary")));
        let needs = join_array(get(gameplan, "needs"), usize::MAX);
        if !needs.is_empty() {
            println!("Needs: {needs}");
        }
        let damage_profile = get(gameplan, "damage_profile").unwrap_or(&Value::Null);
        if damage_profile.as_object().map(|object| !object.is_empty()).unwrap_or(false) {
            println!(
                "Damage profile: {} (ability_damage={}, spirit_scaling={}, weapon_hooks={})",
                display_value(get(damage_profile, "damage_plan")),
                display_value(get(damage_profile, "ability_damage_sources")),
                display_value(get(damage_profile, "spirit_scaling_damage_sources")),
                display_value(get(damage_profile, "weapon_stat_sources"))
            );
        }
        let priority_stats = join_array(get(gameplan, "priority_scaling_stats"), usize::MAX);
        if !priority_stats.is_empty() {
            println!("Priority scaling: {priority_stats}");
        }
        if let Some(abilities) = get(hero, "abilities").and_then(Value::as_array) {
            if !abilities.is_empty() {
                println!("Abilities:");
                for ability in abilities {
                    let tags = join_array(get(ability, "role_tags"), usize::MAX);
                    let key_props = get(ability, "key_properties")
                        .and_then(Value::as_array)
                        .map(|properties| {
                            properties
                                .iter()
                                .take(4)
                                .map(|prop| format!("{}: {}", display_value(get(prop, "label")), format_prop_value(prop)))
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    println!("- {}: {}", display_value(get(ability, "name")), tags);
                    if !key_props.is_empty() {
                        println!("  {}", key_props.join("; "));
                    }
                }
            }
        }
        if let Some(lane_priorities) = get(gameplan, "lane_priorities").and_then(Value::as_array) {
            if !lane_priorities.is_empty() {
                println!("Lane priorities:");
                for priority in lane_priorities {
                    println!("- {}", display_value(Some(priority)));
                }
            }
        }
    }
    println!(
        "Shop spike: {} souls per category",
        display_value(get(economy, "important_shop_spike"))
    );
    let wiki_rules = get(economy, "wiki_rules").unwrap_or(&Value::Null);
    if wiki_rules.as_object().map(|object| !object.is_empty()).unwrap_or(false) {
        let tiers = join_array(get(wiki_rules, "item_tiers"), usize::MAX);
        println!(
            "Shop rules: tiers={}; active limit={}; bonus cap={}",
            tiers,
            display_value(get(wiki_rules, "active_item_limit")),
            display_value(get(wiki_rules, "shop_bonus_cap"))
        );
    }
    println!("Plan: {}", display_value(get(build, "plan")));
    if let Some(lane_model) = get(build, "lane_decision_model").and_then(Value::as_array) {
        if !lane_model.is_empty() {
            println!("Lane decision model:");
            for line in lane_model {
                println!("- {}", display_value(Some(line)));
            }
        }
    }
    if let Some(signals) = get(ctx, "statlocker_signals").and_then(Value::as_array) {
        if !signals.is_empty() {
            println!("Statlocker WPA signals:");
            for signal in signals.iter().take(8) {
                let wpa = get(signal, "cost_relative_wpa").filter(|value| !value.is_null()).or_else(|| get(signal, "wpa"));
                println!(
                    "- {}: wpa={} n={} time={}",
                    display_value(get(signal, "item")),
                    display_value(wpa),
                    display_value(get(signal, "sample_size")),
                    display_value(get(signal, "mean_purchase_time_min"))
                );
            }
        }
    }
    println!();
    for section in ["early", "core", "late", "situational"] {
        let Some(rows) = get(build, section).and_then(Value::as_array) else {
            continue;
        };
        if rows.is_empty() {
            continue;
        }
        println!("{}:", capitalize(section));
        for row in rows {
            println!(
                "- {} [{} T{} / {}] score={}",
                display_value(get(row, "name")),
                display_value(get(row, "slot")),
                display_value(get(row, "tier")),
                display_value(get(row, "cost")),
                display_value(get(row, "score"))
            );
            let archetypes = join_array(get(row, "archetypes"), usize::MAX);
            if !archetypes.is_empty() {
                println!("  archetypes: {archetypes}");
            }
            let why = join_array(get(row, "why"), usize::MAX).replace(", ", "; ");
            if !why.is_empty() {
                println!("  why: {why}");
            }
            let warnings = join_array(get(row, "warnings"), usize::MAX).replace(", ", "; ");
            if !warnings.is_empty() {
                println!("  warn: {warnings}");
            }
        }
        println!();
    }
    let targets = get(build, "shop_routes_to_4800").and_then(Value::as_object);
    if let Some(targets) = targets.filter(|value| !value.is_empty()) {
        println!("Shop routes to 4800:");
        for (slot, target) in targets {
            let route_names = get(target, "items")
                .and_then(Value::as_array)
                .map(|items| {
                    items
                        .iter()
                        .take(5)
                        .map(|row| display_value(get(row, "name")))
                        .collect::<Vec<_>>()
                        .join(", ")
                })
                .unwrap_or_default();
            println!(
                "- {}: spend={} / reached={} / {}",
                str_value(get(target, "label")).unwrap_or(slot),
                display_value(get(target, "spend")),
                if bool_value(get(target, "target_reached")) { "yes" } else { "no" },
                route_names
            );
        }
    }
    if let Some(rejected) = get(ctx, "rejected_expensive_items").and_then(Value::as_array) {
        if !rejected.is_empty() {
            println!();
            println!("Expensive items to justify:");
            for row in rejected.iter().take(6) {
                let warnings = join_array(get(row, "warnings"), usize::MAX).replace(", ", "; ");
                println!(
                    "- {} cost={} score={} bucket={} {}",
                    display_value(get(row, "name")),
                    display_value(get(row, "cost")),
                    display_value(get(row, "score")),
                    display_value(get(row, "bucket")),
                    warnings
                );
            }
        }
    }
}

fn print_item_context(ctx: &Value) {
    println!(
        "Item: {} [{} T{} / {}]",
        display_value(get(ctx, "name")),
        display_value(get(ctx, "slot")),
        display_value(get(ctx, "tier")),
        display_value(get(ctx, "cost"))
    );
    println!(
        "Active: {} / activation={}",
        if bool_value(get(ctx, "is_active")) { "yes" } else { "no" },
        display_value(get(ctx, "activation"))
    );
    let archetypes = join_array(get(ctx, "archetypes"), usize::MAX);
    if !archetypes.is_empty() {
        println!("Archetypes: {archetypes}");
    }
    if let Some(description) = str_value(get(ctx, "description")).filter(|value| !value.is_empty()) {
        println!("Desc: {description}");
    }
    if let Some(properties) = get(ctx, "properties").and_then(Value::as_array) {
        if !properties.is_empty() {
            println!("Properties:");
            for prop in properties.iter().take(20) {
                println!("- {}: {}", display_value(get(prop, "label")), format_prop_value(prop));
                let scales = join_array(get(prop, "scales_with"), usize::MAX);
                if !scales.is_empty() {
                    println!("  scales: {scales}");
                }
            }
        }
    }
}

fn print_analysis_result(target: &str, result: &Value) {
    match target {
        "save-review" => {
            println!(
                "Analysis note: {} / {} / {}",
                display_value(get(result, "id")),
                display_value(get(result, "query")),
                display_value(get(result, "status"))
            );
            println!(
                "Entity: {} / {}",
                display_value(get(result, "entity_type")),
                display_value(get(result, "entity_name"))
            );
            println!("Context hash: {}", display_value(get(result, "context_hash")));
        }
        "run-minimax" => {
            if bool_value(get(result, "dry_run")) {
                let messages = get(result, "request")
                    .and_then(|request| get(request, "messages"))
                    .and_then(Value::as_array)
                    .map(Vec::len)
                    .unwrap_or(0);
                println!("MiniMax dry-run: {}", display_value(get(result, "model")));
                println!("Endpoint: {}", display_value(get(result, "endpoint")));
                println!("API key present: {}", if bool_value(get(result, "api_key_present")) { "yes" } else { "no" });
                println!("Messages: {messages}");
                return;
            }
            let note = get(result, "note").unwrap_or(&Value::Null);
            println!(
                "MiniMax analysis: note={} model={}",
                display_value(get(note, "id")),
                display_value(get(result, "model"))
            );
            println!("{}", display_value(get(result, "result_text")));
        }
        _ => {
            let rows = result.as_array().map(Vec::as_slice).unwrap_or(&[]);
            println!("Analysis notes: rows={}", rows.len());
            for row in rows {
                println!(
                    "- {}: {} / {} / {} / {}",
                    display_value(get(row, "id")),
                    display_value(get(row, "query")),
                    display_value(get(row, "entity_type")),
                    display_value(get(row, "entity_name")),
                    display_value(get(row, "status"))
                );
                println!(
                    "  {} / {} / {}",
                    display_value(get(row, "prompt_version")),
                    str_value(get(row, "model")).unwrap_or("no-model"),
                    display_value(get(row, "context_hash"))
                );
            }
        }
    }
}

fn print_learn_result(target: &str, result: &Value) {
    match target {
        "import-steam-builds" => {
            println!("Steam build import: {}", display_value(get(result, "steam_db_path")));
            println!(
                "rows={} imported={} updated={} skipped={}",
                display_or(get(result, "rows_seen"), "0"),
                display_or(get(result, "imported"), "0"),
                display_or(get(result, "updated"), "0"),
                display_or(get(result, "skipped"), "0")
            );
            if get(result, "error").is_some_and(|value| !value.is_null()) {
                println!("error={}", display_value(get(result, "error")));
            }
        }
        "list-builds" => {
            let rows = result.as_array().map(Vec::as_slice).unwrap_or(&[]);
            println!("Learned builds: rows={}", rows.len());
            for row in rows {
                let items = join_array(get(row, "item_names"), 8);
                println!(
                    "- {}: {} rank={} quality={} score={} / {}",
                    display_value(get(row, "id")),
                    display_value(get(row, "hero_name")),
                    display_value(get(row, "source_rank")),
                    display_value(get(row, "quality_tier")),
                    display_value(get(row, "quality_score")),
                    display_value(get(row, "name"))
                );
                if !items.is_empty() {
                    println!("  items: {items}");
                }
            }
        }
        "analyze-build" => {
            if bool_value(get(result, "dry_run")) {
                let messages = request_message_count(result);
                println!(
                    "Build learning dry-run: build={} model={}",
                    display_value(get(result, "build_id")),
                    display_value(get(result, "model"))
                );
                println!("Endpoint: {}", display_value(get(result, "endpoint")));
                println!("API key present: {}", if bool_value(get(result, "api_key_present")) { "yes" } else { "no" });
                println!("Messages: {messages}");
                let note = get(result, "note").unwrap_or(&Value::Null);
                if note.as_object().map(|object| !object.is_empty()).unwrap_or(false) {
                    println!(
                        "Stored context note: {} / {}",
                        display_value(get(note, "id")),
                        display_value(get(note, "status"))
                    );
                }
                return;
            }
            println!(
                "Build learning analysis: build={} model={}",
                display_value(get(result, "build_id")),
                display_value(get(result, "model"))
            );
            println!("{}", display_value(get(result, "result_text")));
        }
        "analyze-next" => {
            println!(
                "Build learning batch: selected={} model={}",
                display_or(get(result, "pending_selected"), "0"),
                display_value(get(result, "model"))
            );
            println!("Dry run: {}", if bool_value(get(result, "dry_run")) { "yes" } else { "no" });
            println!("API key present: {}", if bool_value(get(result, "api_key_present")) { "yes" } else { "no" });
            if let Some(rows) = get(result, "results").and_then(Value::as_array) {
                for row in rows {
                    println!(
                        "- {}: {} rank={} {} note={} / {}",
                        display_value(get(row, "build_id")),
                        display_value(get(row, "hero_name")),
                        display_value(get(row, "source_rank")),
                        display_value(get(row, "status")),
                        str_value(get(row, "note_id")).unwrap_or("-"),
                        display_value(get(row, "name"))
                    );
                    if let Some(error) = str_value(get(row, "error")).filter(|value| !value.is_empty()) {
                        println!("  error: {error}");
                    }
                }
            }
        }
        _ => {}
    }
}

fn print_player_result(target: &str, result: &Value) {
    match target {
        "list-matches" => {
            let rows = result.as_array().map(Vec::as_slice).unwrap_or(&[]);
            println!("Player matches: rows={}", rows.len());
            for row in rows {
                let summary = get(row, "summary").unwrap_or(&Value::Null);
                let hero_id = get(row, "hero_id")
                    .filter(|value| !value.is_null())
                    .or_else(|| get(summary, "heroId"))
                    .or_else(|| get(summary, "hero_id"));
                let won = get(summary, "won").and_then(Value::as_bool);
                let result_text = match won {
                    Some(true) => "win",
                    Some(false) => "loss",
                    None => "unknown",
                };
                println!(
                    "- account={} match={} hero_id={} result={}",
                    display_value(get(row, "account_id")),
                    display_value(get(row, "match_id")),
                    display_or(hero_id, "-"),
                    result_text
                );
            }
        }
        "match-context" => {
            println!(
                "Player match context: account={} match={}",
                display_value(get(result, "account_id")),
                display_value(get(result, "match_id"))
            );
            println!(
                "Hero: {} ({})",
                display_or(get(result, "hero_name"), "-"),
                display_or(get(result, "hero_id"), "-")
            );
            let player_match = get(result, "player_match").and_then(Value::as_object);
            let keys = player_match
                .map(|object| object.keys().cloned().collect::<Vec<_>>().join(", "))
                .unwrap_or_else(|| "-".to_string());
            println!("Match row keys: {keys}");
            let detail = get(result, "match_detail").and_then(Value::as_object);
            println!("Match detail: {}", if detail.is_some_and(|object| !object.is_empty()) { "yes" } else { "no" });
            let api_match = get(result, "deadlock_api_match").unwrap_or(&Value::Null);
            let actual_items = get(api_match, "player")
                .and_then(|player| get(player, "actual_item_timeline"))
                .and_then(Value::as_array)
                .map(Vec::len)
                .unwrap_or(0);
            println!(
                "Deadlock API match: {}",
                if api_match.as_object().map(|object| !object.is_empty()).unwrap_or(false) {
                    "yes"
                } else {
                    "no"
                }
            );
            if actual_items > 0 {
                println!("Actual item events: {actual_items}");
            }
            let build_analysis = get(result, "player_build_analysis").and_then(Value::as_object);
            println!(
                "Player build analysis: {}",
                if build_analysis.is_some_and(|object| !object.is_empty()) {
                    "yes"
                } else {
                    "no"
                }
            );
        }
        "analyze-match" => {
            if bool_value(get(result, "dry_run")) {
                println!(
                    "Player match dry-run: account={} match={} model={}",
                    display_value(get(result, "account_id")),
                    display_value(get(result, "match_id")),
                    display_value(get(result, "model"))
                );
                println!(
                    "Hero: {} ({})",
                    display_or(get(result, "hero_name"), "-"),
                    display_or(get(result, "hero_id"), "-")
                );
                println!("Endpoint: {}", display_value(get(result, "endpoint")));
                println!("API key present: {}", if bool_value(get(result, "api_key_present")) { "yes" } else { "no" });
                println!("Messages: {}", request_message_count(result));
                let note = get(result, "note").unwrap_or(&Value::Null);
                if note.as_object().map(|object| !object.is_empty()).unwrap_or(false) {
                    println!(
                        "Stored context note: {} / {}",
                        display_value(get(note, "id")),
                        display_value(get(note, "status"))
                    );
                }
                return;
            }
            println!(
                "Player match analysis: account={} match={} model={}",
                display_value(get(result, "account_id")),
                display_value(get(result, "match_id")),
                display_value(get(result, "model"))
            );
            println!(
                "Hero: {} ({})",
                display_or(get(result, "hero_name"), "-"),
                display_or(get(result, "hero_id"), "-")
            );
            println!("{}", display_value(get(result, "result_text")));
        }
        "analyze-next" => {
            println!(
                "Player match batch: selected={} model={}",
                display_or(get(result, "pending_selected"), "0"),
                display_value(get(result, "model"))
            );
            println!("Dry run: {}", if bool_value(get(result, "dry_run")) { "yes" } else { "no" });
            println!("API key present: {}", if bool_value(get(result, "api_key_present")) { "yes" } else { "no" });
            if let Some(rows) = get(result, "results").and_then(Value::as_array) {
                for row in rows {
                    let hero = get(row, "hero_name")
                        .filter(|value| !value.is_null())
                        .or_else(|| get(row, "hero_id"));
                    println!(
                        "- account={} match={} hero={} {} note={}",
                        display_value(get(row, "account_id")),
                        display_value(get(row, "match_id")),
                        display_or(hero, "-"),
                        display_value(get(row, "status")),
                        display_or(get(row, "note_id"), "-")
                    );
                    if let Some(error) = str_value(get(row, "error")).filter(|value| !value.is_empty()) {
                        println!("  error: {error}");
                    }
                }
            }
        }
        _ => {}
    }
}

fn request_message_count(result: &Value) -> usize {
    get(result, "request")
        .and_then(|request| get(request, "messages"))
        .and_then(Value::as_array)
        .map(Vec::len)
        .unwrap_or(0)
}

fn format_prop_value(prop: &Value) -> String {
    let prefix = str_value(get(prop, "prefix")).unwrap_or_default();
    let value = get(prop, "value")
        .filter(|value| !value.is_null())
        .map(|value| display_value(Some(value)))
        .unwrap_or_default();
    let mut postfix = str_value(get(prop, "postfix")).unwrap_or_default().to_string();
    if !postfix.is_empty() && value.ends_with(postfix.trim()) {
        postfix.clear();
    }
    render_sign_token(prefix, &value, &postfix)
}

fn render_sign_token(prefix: &str, value: &str, postfix: &str) -> String {
    let mut rendered_prefix = prefix.to_string();
    let mut rendered_postfix = postfix.to_string();
    if !rendered_prefix.contains("{s:sign}") && !rendered_postfix.contains("{s:sign}") {
        return format!("{rendered_prefix}{value}{rendered_postfix}");
    }
    let sign = if value.trim_start().starts_with('-') { "-" } else { "+" };
    let unsigned_value = value.trim_start_matches(['+', '-']);
    rendered_prefix = rendered_prefix.replace("{s:sign}", sign);
    rendered_postfix = rendered_postfix.replace("{s:sign}", sign);
    format!("{rendered_prefix}{unsigned_value}{rendered_postfix}")
}

fn get<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
    value.as_object().and_then(|object| object.get(key))
}

fn str_value(value: Option<&Value>) -> Option<&str> {
    value.and_then(Value::as_str)
}

fn bool_value(value: Option<&Value>) -> bool {
    value.and_then(Value::as_bool).unwrap_or(false)
}

fn array_len(value: Option<&Value>) -> usize {
    value.and_then(Value::as_array).map(Vec::len).unwrap_or(0)
}

fn display_or(value: Option<&Value>, fallback: &str) -> String {
    let rendered = display_value(value);
    if rendered.is_empty() || rendered == "null" {
        fallback.to_string()
    } else {
        rendered
    }
}

fn display_value(value: Option<&Value>) -> String {
    match value {
        Some(Value::String(value)) => value.clone(),
        Some(Value::Null) | None => String::new(),
        Some(other) => other.to_string(),
    }
}

fn join_array(value: Option<&Value>, limit: usize) -> String {
    value
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .take(limit)
                .map(|item| display_value(Some(item)))
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_default()
}

fn capitalize(value: &str) -> String {
    let mut chars = value.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}
