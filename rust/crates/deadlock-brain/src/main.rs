#![forbid(unsafe_code)]
#![allow(dead_code)]

use std::{
    collections::BTreeSet,
    fs,
    io::{self, ErrorKind, Read},
    path::PathBuf,
    process,
};

use anyhow::{anyhow, Context, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use deadlock_brain_core::{
    ai::{extract_ai_text, AiClient, AiConfig, ChatCompletionRequest, ChatMessage},
    build_narration,
    config::{self, Settings},
    http::HttpClient,
};
use serde::Serialize;
use serde_json::{json, Value};
use sqlx::PgPool;
use std::os::unix::fs::OpenOptionsExt;

mod hero_compare_cli;
mod pg_entities;
mod pg_insights;
mod pg_patchnotes;
mod pg_steam_news;
#[path = "bin/site/mod.rs"]
mod site;
mod steam_web_api;
#[derive(Debug, Args)]
struct SiteMigrationArgs {
    #[command(flatten)]
    database: site::database::ConnectionArgs,
}

#[derive(Debug, Parser)]
#[command(name = "deadlock-brain")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Zeigt öffentliche Spielvergleiche und Steckbriefe auf der Brain-Site.")]
    Site(site::Args),
    #[command(about = "Richtet den begrenzten öffentlichen Steckbriefzugriff ein.")]
    SiteMigrate(SiteMigrationArgs),
    #[command(
        name = "compare-heroes",
        about = "Berechnet einen belegten HP-Vergleich zweier Helden als Vorschau oder veröffentlicht ihn nach Bestätigung."
    )]
    CompareHeroes(hero_compare_cli::CompareHeroesArgs),
    #[command(
        about = "Stellt eine Frage ausschließlich über den typisierten brain-serve/BrainClient-Pfad."
    )]
    Answer(BrainAnswerArgs),
    #[command(about = "Zeigt das zentral geprüfte KI-Modell ohne Modellaufruf.")]
    AiModel,
    #[command(about = "Zeigt lokale DB- und Source-Counts.")]
    Status,
    #[command(about = "Baut einen kompakten Datenkontext fuer eine Entity-Frage.")]
    Context(EntityContextArgs),
    #[command(about = "Baut eine Patch-Timeline fuer eine Entity.")]
    Timeline(TimelineArgs),
    #[command(about = "Baut einen KI-tauglichen Review-Kontext ohne Modellaufruf.")]
    Review(ReviewArgs),
    #[command(
        name = "ask-context",
        about = "Baut ein vertrauenssortiertes Wissens-Buendel plus LLM-Prompt zu einer beliebigen Deadlock-Frage."
    )]
    AskContext(AskContextArgs),
    #[command(
        name = "review-build",
        about = "Berechnet eine Build-Anfrage und veröffentlicht sie als gekennzeichnetes Review-Build."
    )]
    ReviewBuild(ReviewBuildArgs),
    #[command(
        name = "publish-build-query",
        about = "Veröffentlicht eine ausdrücklich angeforderte Build-Variante nach der bestehenden Daten- und Mechanikprüfung."
    )]
    PublishBuildQuery(ReviewBuildArgs),
    #[command(about = "Fuehrt lokale Datenqualitaetschecks aus.")]
    Quality(PrettyArgs),
    #[command(about = "Erzeugt mechanisch begründete Reasoner-Builds und Backtests.")]
    Reason {
        #[command(subcommand)]
        target: ReasonCommands,
    },
    #[command(about = "Zeigt Rename-/Rework-Beziehungen aus Patchnotes.")]
    Lineage(LineageArgs),
    #[command(about = "Zeigt alte/entfernte Entities aus Patchnotes.")]
    Legacy(LegacyArgs),
    #[command(
        about = "Sucht Entities nach Typ/Name in der zentralen Postgres (brain.entities + brain.entity_aliases)."
    )]
    Entities(EntitiesArgs),
    #[command(name = "build-context")]
    BuildContext(BuildContextArgs),
    #[command(name = "build-eval")]
    BuildEval(BuildEvalArgs),
    #[command(
        name = "build-spec",
        about = "Synthesize publishable build payload JSON from corpus and live data."
    )]
    BuildSpec(BuildSpecArgs),
    #[command(about = "Zeigt strukturierte Item-Daten aus der Deadlock Assets API.")]
    Item(ItemArgs),
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
    #[command(
        about = "Zieht alle Sheet-Tabs frisch und normalisiert Hero-Stats und Tab-Tabellen."
    )]
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
    #[command(
        name = "pg",
        about = "Synchronisiert Daten direkt in die zentrale Postgres-DB."
    )]
    Pg {
        #[command(subcommand)]
        target: PgCommands,
    },
    #[command(about = "Importiert kuratierte Insights in die zentrale Postgres-DB.")]
    Insights {
        #[command(subcommand)]
        target: InsightCommands,
    },
}

#[derive(Debug, Args)]
struct BrainAnswerArgs {
    #[arg(
        long,
        value_name = "JSON",
        help = "Normale Client-Konfiguration mit Endpunkt und Infisical-Referenz."
    )]
    client_config: PathBuf,
    #[arg(
        long,
        value_name = "LOOPBACK_URL",
        help = "Expliziter brain-serve-Endpunkt; sonst aus der Client-Konfiguration."
    )]
    endpoint: Option<String>,
    #[arg(long, value_parser = clap::value_parser!(u64).range(1..), help = "Discord-Autor-ID für eine lokale Probe des echten Bot-Antwortpfads.")]
    discord_user_id: Option<u64>,
    #[arg(
        long = "scope",
        value_name = "SCOPE",
        required = true,
        help = "Trusted scope binding; may be repeated."
    )]
    scopes: Vec<String>,
    #[arg(long, default_value_t = 8_000)]
    timeout_ms: u64,
    #[arg(long)]
    request_id: Option<String>,
    #[arg(long)]
    conversation_id: Option<String>,
    #[arg(help = "Question sent to brain-serve.")]
    question: String,
}

#[derive(Debug, Args)]
struct PrettyArgs {
    #[arg(long, help = "Kompakter menschenlesbarer Output statt JSON.")]
    pretty: bool,
}

#[derive(Debug, Args)]
struct EntitiesArgs {
    #[arg(
        long = "type",
        value_name = "TYPE",
        help = "Optionaler Entity-Typ-Filter (z.B. hero, item, ability)."
    )]
    entity_type: Option<String>,
    #[arg(
        long,
        value_name = "QUERY",
        help = "Suchbegriff (Teilstring in canonical_name oder Alias)."
    )]
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
    #[arg(
        long = "prompt-only",
        help = "Nur den deutschen Prompt-Entwurf ausgeben."
    )]
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
    #[arg(long = "game-wiki-dir", value_name = "PATH")]
    game_wiki_dir: Option<PathBuf>,
    #[arg(long)]
    pretty: bool,
}

#[derive(Debug, Args)]
struct ReviewBuildArgs {
    #[arg(help = "Natürliche Build-Anfrage, zum Beispiel 'Warden Gun Build'.")]
    query: String,
    #[arg(long = "wait-seconds", default_value_t = 30)]
    wait_seconds: u64,
    #[command(flatten)]
    publish_connection: PublishConnectionArgs,
}

#[derive(Debug, Args)]
struct PublishConnectionArgs {
    #[arg(long = "publish-endpoint", default_value = "http://127.0.0.1:8783")]
    endpoint: String,
    #[arg(long = "infisical-config")]
    infisical_config: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
enum ReasonCommands {
    #[command(name = "build", about = "Baut einen Reasoner-Build für einen Helden.")]
    Build(ReasonBuildArgs),
    #[command(
        name = "resume-publish",
        about = "Setzt eine gespeicherte Veröffentlichung ohne neue Build-Berechnung oder KI-Aufruf fort."
    )]
    ResumePublish(PublishResumeArgs),
    #[command(
        name = "patch-impact",
        about = "Zeigt die Wirkung des letzten Patches."
    )]
    PatchImpact(ReasonPatchImpactArgs),
    #[command(
        name = "backtest",
        about = "Vergleicht Reasoner-Builds mit Autoren-Builds."
    )]
    Backtest(ReasonBacktestArgs),
}

#[derive(Debug, Args)]
struct ReasonBuildArgs {
    #[arg(help = "Heldname oder Hero-ID.")]
    hero: String,
    #[arg(long = "no-ai")]
    no_ai: bool,
    #[arg(
        long,
        conflicts_with = "publish",
        help = "Trockenlauf ohne Reasoner-DB-Schreibzugriffe, auch für Read-only-Verbindungen."
    )]
    no_persist: bool,
    #[arg(long)]
    patch: Option<String>,
    #[arg(long)]
    publish: bool,
    #[arg(long = "wait-seconds", default_value_t = 30)]
    wait_seconds: u64,
    #[command(flatten)]
    publish_connection: PublishConnectionArgs,
    #[arg(long)]
    json: bool,
    #[arg(long = "seed-path", value_name = "PATH")]
    seed_path: Option<PathBuf>,
}

#[derive(Debug, Args)]
struct PublishResumeArgs {
    #[arg(
        value_name = "DATEI",
        help = "Gespeicherte Veröffentlichungsanfrage im JSON-Format."
    )]
    request_path: PathBuf,
    #[arg(long = "wait-seconds", default_value_t = 30)]
    wait_seconds: u64,
    #[command(flatten)]
    publish_connection: PublishConnectionArgs,
    #[arg(long, help = "Ergebnis als JSON ausgeben.")]
    json: bool,
}

#[derive(Debug, Args)]
struct ReasonPatchImpactArgs {
    #[arg(help = "Heldname oder Hero-ID.")]
    hero: String,
    #[arg(
        long,
        help = "Trockenlauf ohne Reasoner-DB-Schreibzugriffe, auch für Read-only-Verbindungen."
    )]
    no_persist: bool,
    #[arg(long)]
    patch: Option<String>,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Args)]
struct ReasonBacktestArgs {
    #[arg(
        long,
        help = "Trockenlauf ohne Reasoner-DB-Schreibzugriffe, auch für Read-only-Verbindungen."
    )]
    no_persist: bool,
    #[arg(long)]
    hero: Option<String>,
    #[arg(long)]
    patch: Option<String>,
    #[arg(long)]
    json: bool,
    #[arg(long = "seed-path", value_name = "PATH")]
    seed_path: Option<PathBuf>,
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
struct BuildSpecArgs {
    hero: String,
    #[arg(
        long = "corpus-dir",
        value_name = "PATH",
        default_value = dbrain_builds::spec::DEFAULT_CORPUS_DIR
    )]
    corpus_dir: PathBuf,
}

#[derive(Debug, Args)]
struct ItemArgs {
    #[arg(help = "Item-Name.")]
    query: String,
    #[arg(long, help = "Kompakter menschenlesbarer Output statt JSON.")]
    pretty: bool,
}

#[derive(Debug, Subcommand)]
enum AnalysisCommands {
    #[command(
        name = "save-review",
        about = "Speichert einen Review-Kontext ohne Modellaufruf."
    )]
    SaveReview(AnalysisSaveReviewArgs),
    #[command(
        name = "run-fireworks",
        aliases = ["run-minimax", "run-ai"],
        about = "Ruft Fireworks/DeepSeek fuer einen Review-Kontext auf und speichert das Ergebnis."
    )]
    RunAi(AnalysisRunAiArgs),
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
struct AnalysisRunAiArgs {
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
    #[arg(
        long = "dry-run",
        help = "Baut nur den Fireworks-Request ohne API-Aufruf."
    )]
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
    #[command(name = "build-data")]
    BuildData(PullBuildDataArgs),
    #[command(about = "Importiert bestehende Patchnotes aus der zentralen Bot-DB.")]
    Patchnotes(PullPatchnotesArgs),
    #[command(about = "Liest öffentliche Threads aus dem Deadlock-Forum anhand der Sitemap.")]
    Forum(PullForumArgs),
    #[command(about = "Zieht öffentliche Threads und Kommentare aus Reddit-Subreddits.")]
    Reddit(PullRedditArgs),
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
            "ranks",
            "colors",
            "build_tags",
            "npc_units",
            "generic_data",
            "misc_entities",
            "modifiers",
        ],
        help = "Endpoints auswählen. Mehrfach nutzbar. Standard: vollständige Spieldaten je Clientversion, Englisch und Deutsch; Modifiers ohne Sprache."
    )]
    kind: Vec<String>,
    #[arg(
        long,
        value_name = "PFAD",
        help = "Datenverzeichnis; sonst gemeinsame Konfiguration, standardmäßig /home/nathanael/.local/share/deadlock-brain."
    )]
    data_dir: Option<PathBuf>,
}

#[derive(Debug, Args)]
struct PullBuildDataArgs {
    #[arg(long)]
    hero: String,
}

#[derive(Debug, Args)]
struct PullPatchnotesArgs {}

#[derive(Debug, Args)]
struct PullForumArgs {
    #[arg(long, default_value = "/home/nathanael/.local/share/deadlock-brain")]
    data_dir: PathBuf,
    #[arg(long, default_value = "/etc/deadlock-brain/infisical.json")]
    archive_infisical_config: PathBuf,
    #[arg(
        long,
        help = "Nutzt die vorhandene lokale Brave-Sitzung für öffentliche Seiten."
    )]
    browser: bool,
    #[arg(long, requires = "publish_release")]
    base_release: Option<String>,
    #[arg(long, requires = "base_release", requires = "kernel_config")]
    publish_release: Option<String>,
    #[arg(long)]
    kernel_config: Option<PathBuf>,
    #[arg(long, default_value = "/etc/deadlock-brain/infisical.json")]
    kernel_infisical_config: PathBuf,
    #[arg(
        long = "sitemap-url",
        default_value = "https://forums.playdeadlock.com/sitemap.xml"
    )]
    sitemap_url: String,
    #[arg(
        long,
        default_value_t = 25,
        help = "0 bedeutet ohne Limit; für Nachimporte können kleine Durchläufe verwendet werden."
    )]
    limit: usize,
    #[arg(long = "delay-seconds", default_value_t = 1.0)]
    delay_seconds: f64,
    #[arg(long = "cache-ttl-seconds", default_value_t = 86_400)]
    cache_ttl_seconds: u64,
    #[arg(
        long = "refresh-existing",
        help = "Bereits gespeicherte Thread-IDs erneut abrufen."
    )]
    refresh_existing: bool,
}

#[derive(Debug, Args)]
struct PullRedditArgs {
    #[arg(long = "subreddit", help = "Mehrfach nutzbar. Default: Deadlock.")]
    subreddit: Vec<String>,
    #[arg(
        long,
        default_value_t = 25,
        help = "Limit pro Subreddit; 0 bedeutet ohne Limit."
    )]
    limit: usize,
    #[arg(long = "delay-seconds", default_value_t = 2.0)]
    delay_seconds: f64,
    #[arg(long = "cache-ttl-seconds", default_value_t = 86_400)]
    cache_ttl_seconds: u64,
    #[arg(
        long = "refresh-existing",
        help = "Bereits gespeicherte Thread-IDs erneut abrufen."
    )]
    refresh_existing: bool,
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
    #[arg(
        long,
        help = "z.B. patch_129989 oder 129989. Default: neuester Statlocker-WPA-Patch."
    )]
    patch: Option<String>,
    #[arg(
        long,
        default_value = "all",
        help = "all oder Hero-Name, z.B. Mo & Krill."
    )]
    hero: String,
    #[arg(long = "min-sample-size", default_value_t = 500)]
    min_sample_size: u64,
    #[arg(long, default_value = "rank_10,rank_11")]
    rank: String,
    #[arg(long = "leaderboard-page", default_value_t = 1)]
    leaderboard_page: u64,
    #[arg(long = "leaderboard-page-size", default_value_t = 100)]
    leaderboard_page_size: u64,
    #[arg(
        long = "account-id",
        help = "Statlocker/Steam accountId fuer Player-Endpunkte."
    )]
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
    #[command(
        name = "sheet-stats",
        about = "Baut normalisierte Hero-Stats aus dem Google Sheet."
    )]
    SheetStats(NormalizeSheetStatsArgs),
    #[command(
        name = "sheet-tabs",
        about = "Baut normalisierte Tabellen fuer Hero-Rankings, Boons/AP und Raw-Heroes."
    )]
    SheetTabs(NormalizeSheetTabsArgs),
    #[command(
        name = "resolve-gaps",
        about = "Loest offene Entity-Luecken konservativ neu auf."
    )]
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
    #[arg(
        long = "dry-run",
        help = "Nur Report erzeugen, keine DB-Aenderungen schreiben."
    )]
    dry_run: bool,
}

#[derive(Debug, Subcommand)]
enum ParseCommands {
    #[command(about = "Parst Patchnotes zu patch_events.")]
    Patchnotes(ParsePatchnotesArgs),
    #[command(
        name = "forum-claims",
        about = "Parst Forum-Posts zu historischen, quarantined Claims."
    )]
    ForumClaims(ParseForumClaimsArgs),
    #[command(
        name = "reddit-claims",
        about = "Parst Reddit-Posts zu historischen, quarantined Claims."
    )]
    RedditClaims(ParseRedditClaimsArgs),
}

#[derive(Debug, Args)]
struct ParsePatchnotesArgs {
    #[arg(long, help = "Loescht patch_events vorher neu.")]
    rebuild: bool,
    #[arg(
        long = "dry-run",
        help = "Nur Plan und Zaehler ausgeben, keine DB-Aenderungen schreiben."
    )]
    dry_run: bool,
    #[arg(
        long,
        help = "Nur neue Posts aus dem Patchnotes-Feed übernehmen und noch ungeparste Patches ergänzen."
    )]
    feed: bool,
}

#[derive(Debug, Args)]
struct ParseForumClaimsArgs {
    #[arg(long, help = "Loescht forum_claims vorher neu.")]
    rebuild: bool,
}

#[derive(Debug, Args)]
struct ParseRedditClaimsArgs {
    #[arg(
        long,
        help = "Löscht nur Reddit-Claims (ingest_source=reddit) vorher neu."
    )]
    rebuild: bool,
}

#[derive(Debug, Subcommand)]
enum PgCommands {
    #[command(
        name = "import-steam-news",
        about = "Importiert offizielle Steam-News-Patches direkt nach brain.* in Postgres."
    )]
    ImportSteamNews(PgSteamNewsArgs),
    #[command(
        name = "import-patchnote",
        about = "Importiert exakt einen Patchnote-Eintrag direkt nach brain.* in Postgres."
    )]
    ImportPatchnote(PgImportPatchnoteArgs),
    #[command(
        name = "sync-patchnotes",
        about = "Übernimmt API-Patchfunde über den vorhandenen Originalquellen-Import."
    )]
    SyncPatchnotes(PgSyncPatchnotesArgs),
}

#[derive(Debug, Subcommand)]
enum InsightCommands {
    #[command(
        name = "import-json",
        about = "Importiert ein Insight-JSON-Array nach brain.insight_records."
    )]
    ImportJson(InsightImportJsonArgs),
}

#[derive(Debug, Args)]
struct PgSteamNewsArgs {
    #[command(flatten)]
    ledger: LedgerAccessArgs,
    #[arg(long = "dsn-env", default_value = "DEADLOCK_CENTRAL_DSN")]
    dsn_env: String,
    #[arg(long, default_value_t = 1_422_450)]
    appid: u32,
    #[arg(long = "start-date", default_value = "2024-01-01")]
    start_date: String,
    #[arg(long, default_value_t = 500)]
    count: u32,
    #[arg(long = "cache-ttl-seconds", default_value_t = 900)]
    cache_ttl_seconds: u64,
    #[arg(
        long = "include-non-official",
        help = "Auch externe Steam-News-Feeds importieren; Standard ist nur steam_community_announcements."
    )]
    include_non_official: bool,
    #[arg(long = "gid", action = clap::ArgAction::Append, help = "Optional auf einzelne Steam-News-GIDs begrenzen.")]
    gids: Vec<String>,
    #[arg(
        long = "dry-run",
        help = "Kein fachlicher Import; Datenbankzugriff und Steam-Abrufjournal bleiben aktiv."
    )]
    dry_run: bool,
}

#[derive(Debug, Args)]
struct PgImportPatchnoteArgs {
    #[command(flatten)]
    ledger: LedgerAccessArgs,
    #[arg(long = "patch-id", help = "changelog_posts.id")]
    patch_id: i64,
    #[arg(long = "dsn-env", default_value = "DEADLOCK_CENTRAL_DSN")]
    dsn_env: String,
    #[arg(
        long = "dry-run",
        help = "Kein fachlicher Import; Datenbankzugriff und Steam-Abrufjournal bleiben aktiv."
    )]
    dry_run: bool,
}

#[derive(Debug, Args)]
struct PgSyncPatchnotesArgs {
    #[command(flatten)]
    ledger: LedgerAccessArgs,
    #[arg(long = "dsn-env", default_value = "DEADLOCK_CENTRAL_DSN")]
    dsn_env: String,
    #[arg(
        long,
        help = "Kein fachlicher Import; Datenbankzugriff und HTTP-Abrufe bleiben aktiv, ohne Steam-Abrufjournal."
    )]
    dry_run: bool,
}

#[derive(Debug, Args)]
struct LedgerAccessArgs {
    #[arg(long, default_value = "config/steam-ledger.json")]
    ledger_config: PathBuf,
    #[arg(long, default_value = "config/infisical.json")]
    infisical_config: PathBuf,
}

async fn ledger_for_pg(target: &PgCommands) -> Result<Option<steam_web_api::SteamLedger>> {
    let (args, caller) = match target {
        PgCommands::ImportSteamNews(args) => (&args.ledger, pg_steam_news::STEAM_LEDGER_CALLER),
        PgCommands::ImportPatchnote(args) => (&args.ledger, pg_patchnotes::STEAM_LEDGER_CALLER),
        PgCommands::SyncPatchnotes(_) => return Ok(None),
    };
    let path = args.ledger_config.clone();
    let config: steam_web_api::SteamLedgerConfig = tokio::task::spawn_blocking(move || {
        const LIMIT: u64 = 16 * 1024;
        let file = fs::OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
            .open(path)
            .map_err(|_| anyhow!("Ledger-Konfiguration ist nicht lesbar"))?;
        if !file
            .metadata()
            .map_err(|_| anyhow!("Ledger-Konfiguration ist nicht prüfbar"))?
            .is_file()
        {
            anyhow::bail!("Ledger-Konfiguration muss eine normale Datei sein");
        }
        let mut bytes = Vec::new();
        file.take(LIMIT + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| anyhow!("Ledger-Konfiguration ist nicht lesbar"))?;
        if bytes.len() as u64 > LIMIT {
            anyhow::bail!("Ledger-Konfiguration ist zu groß");
        }
        serde_json::from_slice(&bytes).map_err(|_| anyhow!("Ledger-Konfiguration ist ungültig"))
    })
    .await
    .map_err(|_| anyhow!("Ledger-Konfiguration ist nicht verfügbar"))??;
    let values = tokio::time::timeout(
        std::time::Duration::from_secs(10),
        dl_token_secrets::values(&args.infisical_config),
    )
    .await
    .map_err(|_| anyhow!("Infisical antwortet nicht rechtzeitig"))?
    .map_err(|_| anyhow!("Infisical-Secretquelle ist nicht verfügbar"))?;
    steam_web_api::SteamLedger::from_snapshot(caller, &config, &values)
        .map(Some)
        .map_err(|_| anyhow!("Ledger-Konfiguration oder Infisical-Referenz ist ungültig"))
}

#[derive(Debug, Args)]
struct InsightImportJsonArgs {
    #[arg(
        long = "file",
        value_name = "PATH",
        help = "JSON-Datei; ohne Datei wird stdin gelesen."
    )]
    file: Option<PathBuf>,
    #[arg(long = "dsn-env", default_value = "DEADLOCK_CENTRAL_DSN")]
    dsn_env: String,
    #[arg(
        long = "dry-run",
        help = "Nur validieren und zaehlen, keine PG-Schreibzugriffe."
    )]
    dry_run: bool,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct AnswerClientConfig {
    endpoint: String,
    infisical_config: PathBuf,
    secret_reference: String,
}

impl AnswerClientConfig {
    fn load(path: &std::path::Path) -> Result<Self> {
        const LIMIT: u64 = 16 * 1024;
        let file =
            fs::File::open(path).map_err(|_| anyhow!("Client-Konfiguration ist nicht lesbar"))?;
        if !file
            .metadata()
            .map_err(|_| anyhow!("Client-Konfiguration ist nicht prüfbar"))?
            .is_file()
        {
            anyhow::bail!("Client-Konfiguration muss eine normale Datei sein");
        }
        let mut bytes = Vec::new();
        file.take(LIMIT + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| anyhow!("Client-Konfiguration ist nicht lesbar"))?;
        if bytes.len() as u64 > LIMIT {
            anyhow::bail!("Client-Konfiguration ist zu groß");
        }
        let mut config: Self = serde_json::from_slice(&bytes)
            .map_err(|_| anyhow!("Client-Konfiguration ist ungültig"))?;
        if config.endpoint.trim().is_empty()
            || config.endpoint.len() > 2048
            || config.infisical_config.as_os_str().is_empty()
        {
            anyhow::bail!("Client-Konfiguration ist ungültig");
        }
        let mut name = config.secret_reference.bytes();
        if !name
            .next()
            .is_some_and(|byte| byte.is_ascii_uppercase() || byte == b'_')
            || !name.all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_')
        {
            anyhow::bail!("Infisical-Secret-Referenz ist ungültig");
        }
        if !config.infisical_config.is_absolute() {
            config.infisical_config = path
                .parent()
                .unwrap_or_else(|| std::path::Path::new("."))
                .join(&config.infisical_config);
        }
        Ok(config)
    }

    fn token_until(&self, deadline: std::time::Instant) -> Result<zeroize::Zeroizing<String>> {
        if std::time::Instant::now() >= deadline {
            anyhow::bail!("Infisical antwortet nicht rechtzeitig");
        }
        let path = self.infisical_config.clone();
        let name = self.secret_reference.clone();
        let (sender, receiver) = std::sync::mpsc::sync_channel(1);
        std::thread::Builder::new()
            .name("brain-cli-secrets".into())
            .spawn(move || {
                let result = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .map_err(|_| anyhow!("Infisical-Loader ist nicht verfügbar"))
                    .and_then(|runtime| {
                        runtime
                            .block_on(dl_token_secrets::values(&path))
                            .map_err(|_| anyhow!("Infisical-Secretquelle ist nicht verfügbar"))
                    })
                    .and_then(|values| {
                        values
                            .into_iter()
                            .find(|(key, _)| key == &name)
                            .map(|(_, token)| token)
                            .ok_or_else(|| anyhow!("Konfiguriertes Secret fehlt in Infisical"))
                    })
                    .and_then(|token| {
                        if token.is_empty()
                            || token.len() > 4096
                            || !token.bytes().all(|byte| byte.is_ascii_graphic())
                        {
                            Err(anyhow!("Konfiguriertes Secret ist ungültig"))
                        } else {
                            Ok(token)
                        }
                    });
                let _ = sender.send(result);
            })
            .map_err(|_| anyhow!("Infisical-Loader ist nicht verfügbar"))?;
        let result = receiver
            .recv_timeout(deadline.saturating_duration_since(std::time::Instant::now()))
            .map_err(|_| anyhow!("Infisical antwortet nicht rechtzeitig"))?;
        if std::time::Instant::now() >= deadline {
            anyhow::bail!("Infisical antwortet nicht rechtzeitig");
        }
        result
    }
}

fn brain_answer_deadline(timeout_ms: u64) -> Result<std::time::Instant> {
    anyhow::ensure!(
        (1..=600_000).contains(&timeout_ms),
        "Client-Timeout ist ungültig"
    );
    Ok(std::time::Instant::now() + std::time::Duration::from_millis(timeout_ms))
}

async fn run_brain_answer(args: &BrainAnswerArgs) -> Result<()> {
    let deadline = brain_answer_deadline(args.timeout_ms)?;
    let config = AnswerClientConfig::load(&args.client_config)?;
    let endpoint = args.endpoint.as_deref().unwrap_or(&config.endpoint);
    // Validate the transport before retrieving a credential; this does not send a request.
    brain_client::AsyncBrainClient::new_local(
        endpoint,
        "endpoint-validation",
        std::time::Duration::from_millis(args.timeout_ms),
    )
    .map_err(|_| anyhow!("Brain-Endpunkt ist ungültig"))?;
    let scopes: BTreeSet<String> = args
        .scopes
        .iter()
        .map(|scope| scope.trim())
        .filter(|scope| !scope.is_empty())
        .map(ToOwned::to_owned)
        .collect();
    if scopes.is_empty() || scopes.len() != args.scopes.len() {
        return Err(anyhow!(
            "Scopes müssen explizit, nichtleer und eindeutig sein"
        ));
    }
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .context("Systemzeit liegt vor UNIX_EPOCH")?
        .as_nanos();
    let default_id = format!("brain-cli-{}-{nonce}", process::id());
    let request_id = args
        .request_id
        .clone()
        .unwrap_or_else(|| default_id.clone());
    let conversation_id = args
        .conversation_id
        .clone()
        .unwrap_or_else(|| default_id.clone());
    let query = brain_client::Query {
        answer_context: None,
        request_id,
        conversation_id,
        text: args.question.clone(),
        domain: None,
        requested_scopes: scopes,
        profile: brain_client::AnswerProfile::Explain,
        patch: None,
        mode: None,
    };
    query.validate().context("ungültige Brain-Anfrage")?;
    let token = config.token_until(deadline)?;
    let remaining = deadline.saturating_duration_since(std::time::Instant::now());
    if remaining.is_zero() {
        anyhow::bail!("Brain-Anfrage überschreitet das Zeitbudget");
    }
    let client = brain_client::AsyncBrainClient::new_local(endpoint, &token, remaining)
        .context("BrainClient konnte nicht erstellt werden")?;
    let response = match args.discord_user_id {
        Some(user_id) => client.answer_for_discord(&query, user_id).await,
        None => client.answer(&query).await,
    }
    .context("brain-serve Anfrage fehlgeschlagen")?;
    print_json(&response)
}

#[cfg(test)]
mod answer_deadline_tests {
    use super::*;

    #[test]
    fn explicit_answer_timeout_accepts_ten_minutes_before_loading_config() {
        for (timeout_ms, valid) in [(1, true), (600_000, true), (600_001, false), (0, false)] {
            assert_eq!(brain_answer_deadline(timeout_ms).is_ok(), valid);
        }
    }
}

fn command_is_read_only(command: &Commands) -> bool {
    match command {
        Commands::Context(_)
        | Commands::Timeline(_)
        | Commands::Review(_)
        | Commands::AskContext(_)
        | Commands::Lineage(_)
        | Commands::Legacy(_)
        | Commands::Entities(_) => true,
        Commands::Reason { target } => match target {
            ReasonCommands::Build(args) => args.no_persist,
            ReasonCommands::ResumePublish(_) => true,
            ReasonCommands::PatchImpact(args) => args.no_persist,
            ReasonCommands::Backtest(args) => args.no_persist,
        },
        _ => false,
    }
}

async fn pg_pool_for_command(command: &Commands) -> Result<PgPool> {
    if command_is_read_only(command) {
        deadlock_brain_core::pg::pg_pool_read_only().await
    } else {
        deadlock_brain_core::pg::pg_pool().await
    }
}

fn main() {
    if let Err(error) = run_from_cli() {
        eprintln!("{error:#}");
        process::exit(1);
    }
}

fn run_from_cli() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::AiModel => print_json(&json!({
            "provider": "fireworks",
            "model": deadlock_brain_core::model_resolver::model_for_request()?,
        })),
        command => {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()?;
            runtime.block_on(run(Cli { command }))
        }
    }
}

fn assets_storage_path(path: &std::path::Path) -> Result<PathBuf> {
    anyhow::ensure!(
        path.is_absolute(),
        "Das Datenverzeichnis muss absolut sein. Wähle mit --data-dir oder DEADLOCK_BRAIN_DATA_DIR einen dauerhaften absoluten Pfad."
    );
    let mut resolved = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                resolved.pop();
            }
            other => {
                resolved.push(other.as_os_str());
                match fs::canonicalize(&resolved) {
                    Ok(canonical) => resolved = canonical,
                    Err(error) if error.kind() == ErrorKind::NotFound => {}
                    Err(error) => {
                        return Err(error).context("Das Datenverzeichnis ist nicht prüfbar.");
                    }
                }
            }
        }
    }
    Ok(resolved)
}

async fn run_site_migrate(args: SiteMigrationArgs) -> Result<()> {
    let pool = args.database.pool("brain_migrate").await?;
    let store = brain_storage::PgStore::new(pool);
    store.migrate_compare_artifacts().await?;
    Ok(())
}

async fn run(cli: Cli) -> Result<()> {
    let Cli { command } = cli;
    let command = match command {
        Commands::Site(args) => return site::serve(args).await,
        Commands::SiteMigrate(args) => return run_site_migrate(args).await,
        Commands::CompareHeroes(args) => return hero_compare_cli::run(args).await,
        Commands::Pull {
            source: PullCommands::Forum(args),
        } => return run_forum_pull(args).await,
        other => other,
    };
    if let Commands::Answer(args) = &command {
        return run_brain_answer(args).await;
    }
    if let Commands::Reason {
        target: ReasonCommands::ResumePublish(args),
    } = &command
    {
        return run_publish_resume(args).await;
    }
    let mut settings = config::load_settings()?;
    if let Commands::Pull {
        source: PullCommands::Assets(args),
    } = &command
    {
        if let Some(data_dir) = &args.data_dir {
            settings.data_dir = data_dir.clone();
            settings.raw_dir = data_dir.join("raw");
            settings.cache_dir = data_dir.join("cache");
        }
        let build_root = assets_storage_path(&settings.project_root)?;
        let source_dir = settings.raw_dir.join(dbrain_sources::assets_api::SOURCE);
        for path in [
            &settings.data_dir,
            &settings.raw_dir,
            &settings.cache_dir,
            &source_dir,
        ] {
            let storage_path = assets_storage_path(path)?;
            anyhow::ensure!(
                !storage_path.starts_with(&build_root),
                "Das Datenverzeichnis oder eines seiner Schreibziele liegt im löschbaren Build-Arbeitsbaum. Wähle mit --data-dir oder DEADLOCK_BRAIN_DATA_DIR einen dauerhaften absoluten Pfad außerhalb dieses Arbeitsbaums."
            );
        }
    }
    if let Commands::Entities(args) = &command {
        let pool = pg_pool_for_command(&command).await?;
        return pg_entities::run(args, &pool).await;
    }
    let command = match command {
        Commands::Pg { target } => {
            let ledger = ledger_for_pg(&target).await?;
            // Alle `pg`-Befehle nutzen synchrone Crates (`postgres` bzw. der
            // blocking-HttpClient), die intern teils selbst einen Tokio-Runtime
            // starten. Direkt im async-Kontext aufgerufen wuerde das
            // "runtime within a runtime" paniken -> daher komplett auf einen
            // Blocking-Thread auslagern. HttpClient/Settings werden dort OWNED
            // gebaut, damit die Closure `Send + 'static` bleibt.
            let settings = settings.clone();
            return tokio::task::spawn_blocking(move || {
                fs::create_dir_all(&settings.cache_dir)?;
                let http = http_client(&settings)?;
                run_pg(&http, ledger.as_ref(), target)
            })
            .await?;
        }
        Commands::Insights { target } => {
            // `insights import-json` schreibt ueber den synchronen `postgres`-Crate
            // und muss daher ebenfalls vom async-Runtime entkoppelt laufen.
            return tokio::task::spawn_blocking(move || run_insights(target)).await?;
        }
        other => other,
    };
    prepare_dirs(&settings)?;
    // Wissensabfragen und ausdrücklich persistenzfreie Reasoner-Läufe erzwingen
    // Read-only bereits beim Verbindungsaufbau.
    let pool = pg_pool_for_command(&command).await?;

    match command {
        Commands::Site(_) | Commands::SiteMigrate(_) | Commands::CompareHeroes(_) => {
            unreachable!("site is handled before local DB initialization")
        }
        Commands::Answer(_) => unreachable!("answer is handled before local DB initialization"),
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
                println!(
                    "{}",
                    str_value(get(&result, "prompt_de")).unwrap_or_default()
                );
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
                    game_wiki_dir: args.game_wiki_dir.clone(),
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
        Commands::ReviewBuild(args) => run_requested_build(&pool, args, true).await,
        Commands::PublishBuildQuery(args) => run_requested_build(&pool, args, false).await,
        Commands::Quality(args) => {
            let result = dbrain_retrieval::run_quality_checks(&pool).await?;
            if args.pretty {
                print_quality_report(&result);
                Ok(())
            } else {
                print_json(&result)
            }
        }
        Commands::Reason { target } => run_reason(&pool, &settings, target).await,
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
        Commands::BuildContext(args) => {
            let playstyle = args.playstyle.map(BuildPlaystyle::as_str);
            let result = dbrain_builds::build_context(&pool, &args.hero, playstyle).await?;
            print_json(&result)
        }
        Commands::BuildEval(args) => run_build_eval(&pool, &settings, args).await,
        Commands::BuildSpec(args) => run_build_spec(&pool, &settings, args).await,
        Commands::Item(args) => {
            let result = dbrain_retrieval::build_item_context(&pool, &args.query).await?;
            if args.pretty {
                print_item_context(&result);
                Ok(())
            } else {
                print_json(&result)
            }
        }
        Commands::Analysis { target } => run_analysis(&pool, &settings, target).await,
        Commands::Pull { source } => run_pull(&pool, &settings, source).await,
        Commands::RefreshSheet => run_refresh_sheet(&pool, &settings).await,
        Commands::Normalize { target } => run_normalize(&pool, target).await,
        Commands::Parse { target } => run_parse(&pool, target).await,
        Commands::AiModel => {
            unreachable!("AiModel wird vor allgemeiner Pool-Ausfuehrung ausgefuehrt.")
        }
        Commands::Entities(_) => {
            unreachable!("PG-Entities werden vor allgemeiner Pool-Ausfuehrung ausgefuehrt.")
        }
        Commands::Pg { target: _ } => {
            unreachable!("PG-Commands werden vor allgemeiner Pool-Ausfuehrung ausgefuehrt.")
        }
        Commands::Insights { target: _ } => {
            unreachable!("PG-Insights werden vor allgemeiner Pool-Ausfuehrung ausgefuehrt.")
        }
    }
}

fn run_pg(
    http: &HttpClient,
    ledger: Option<&steam_web_api::SteamLedger>,
    target: PgCommands,
) -> Result<()> {
    match target {
        PgCommands::ImportSteamNews(args) => print_json(&pg_steam_news::import_steam_news(
            http,
            ledger.context("Steam-Abrufjournal fehlt")?,
            &pg_steam_news::ImportSteamNewsOptions {
                dsn_env: args.dsn_env,
                appid: args.appid,
                start_date: args.start_date,
                count: args.count,
                cache_ttl_seconds: args.cache_ttl_seconds,
                include_non_official: args.include_non_official,
                gids: args.gids,
                dry_run: args.dry_run,
            },
        )?),
        PgCommands::ImportPatchnote(args) => {
            run_pg_patchnote(http, ledger.context("Steam-Abrufjournal fehlt")?, args)
        }
        PgCommands::SyncPatchnotes(args) => print_json(&pg_patchnotes::sync_patchnotes(
            http,
            &args.dsn_env,
            args.dry_run,
        )?),
    }
}

#[cfg(test)]
mod ingest_tests {
    use super::*;

    #[test]
    fn assets_cli_accepts_global_kinds_and_a_durable_data_directory() {
        for kind in [
            "items",
            "heroes",
            "heroes_all",
            "generic_data",
            "npc_units",
            "misc_entities",
            "modifiers",
        ] {
            let parsed = Cli::try_parse_from([
                "deadlock-brain",
                "pull",
                "assets",
                "--kind",
                kind,
                "--data-dir",
                "/durable/brain-data",
            ])
            .unwrap();
            let Commands::Pull {
                source: PullCommands::Assets(args),
            } = parsed.command
            else {
                panic!("wrong dispatch")
            };
            assert_eq!(args.kind, [kind]);
            assert_eq!(args.data_dir, Some(PathBuf::from("/durable/brain-data")));
        }
    }

    #[test]
    fn assets_cli_defers_to_shared_settings_with_a_durable_default() {
        let assets = Cli::try_parse_from(["deadlock-brain", "pull", "assets"]).unwrap();
        let Commands::Pull {
            source: PullCommands::Assets(assets),
        } = assets.command
        else {
            panic!("wrong dispatch")
        };
        assert_eq!(assets.data_dir, None);
        assert_eq!(
            config::default_data_dir(),
            PathBuf::from("/home/nathanael/.local/share/deadlock-brain")
        );
        assert!(config::default_data_dir().is_absolute());
    }

    #[test]
    fn patch_sync_cli_uses_existing_ledger_and_dry_run() {
        let parsed =
            Cli::try_parse_from(["deadlock-brain", "pg", "sync-patchnotes", "--dry-run"]).unwrap();
        let Commands::Pg {
            target: PgCommands::SyncPatchnotes(args),
        } = parsed.command
        else {
            panic!("wrong dispatch")
        };
        assert!(args.dry_run);
        assert!(!args.ledger.ledger_config.as_os_str().is_empty());
    }

    #[tokio::test]
    async fn patch_sync_does_not_reload_unneeded_steam_credentials() {
        let parsed = Cli::try_parse_from([
            "deadlock-brain",
            "pg",
            "sync-patchnotes",
            "--dry-run",
            "--ledger-config",
            "/nonexistent/steam-ledger.json",
            "--infisical-config",
            "/nonexistent/infisical.json",
        ])
        .unwrap();
        let Commands::Pg { target } = parsed.command else {
            panic!("wrong dispatch")
        };
        assert!(ledger_for_pg(&target).await.unwrap().is_none());
    }
}

fn run_pg_patchnote(
    http: &HttpClient,
    ledger: &steam_web_api::SteamLedger,
    args: PgImportPatchnoteArgs,
) -> Result<()> {
    print_json(&pg_patchnotes::import_patchnote(
        http,
        ledger,
        &pg_patchnotes::ImportPatchnoteOptions {
            patch_id: args.patch_id,
            dsn_env: args.dsn_env,
            dry_run: args.dry_run,
        },
    )?)
}

fn run_insights(target: InsightCommands) -> Result<()> {
    match target {
        InsightCommands::ImportJson(args) => {
            let raw = read_json_input(args.file.as_ref())?;
            print_json(&pg_insights::import_insights_json(
                &raw,
                &pg_insights::ImportInsightOptions {
                    dsn_env: args.dsn_env,
                    dry_run: args.dry_run,
                },
            )?)
        }
    }
}

fn read_json_input(path: Option<&PathBuf>) -> Result<String> {
    if let Some(path) = path {
        return fs::read_to_string(path).map_err(Into::into);
    }
    let mut raw = String::new();
    io::stdin().read_to_string(&mut raw)?;
    Ok(raw)
}

async fn run_build_eval(pool: &PgPool, settings: &Settings, args: BuildEvalArgs) -> Result<()> {
    let playstyle = args.playstyle.map(BuildPlaystyle::as_str);
    let build_context = dbrain_builds::build_context(pool, &args.hero, playstyle).await?;
    let config = AiConfig::from_settings(settings);
    if !config.api_key_present() {
        return print_json(&json!({
            "build_context": build_context,
            "notice": "Fireworks environment is not configured; skipping narration."
        }));
    }

    let client = AiClient::new(config)?;
    let request = build_narration::build_narration_request(&build_context, client.config())?;
    let response = client.chat(&request)?;
    let narration = extract_ai_text(&response);
    let known_item_names = load_known_item_names(pool, None).await?;
    let validation =
        build_narration::validate_narration(&narration, &build_context, &known_item_names);
    print_json(&json!({
        "build_context": build_context,
        "narration": narration,
        "validation": validation
    }))
}

async fn run_build_spec(pool: &PgPool, settings: &Settings, args: BuildSpecArgs) -> Result<()> {
    let build_context = dbrain_builds::build_context(pool, &args.hero, None).await?;
    let corpus =
        dbrain_builds::spec::load_corpus_build_by_hero_id(&args.corpus_dir, build_context.hero_id)?;
    let understanding_path = args
        .corpus_dir
        .join("understanding")
        .join(format!("{}.md", corpus.hero));
    let mut warnings = Vec::new();
    let (understanding, understanding_found) = match fs::read_to_string(&understanding_path) {
        Ok(text) => (Some(text), true),
        Err(error) if error.kind() == ErrorKind::NotFound => {
            warnings.push(format!(
                "understanding missing: {}",
                understanding_path.display()
            ));
            (None, false)
        }
        Err(error) => return Err(error.into()),
    };

    let known_item_names =
        load_known_item_names(pool, Some(brain_contracts::game_mode::GameMode::Normal)).await?;
    let candidates =
        dbrain_builds::spec::build_candidate_set(&build_context, &corpus, &known_item_names);
    let client = AiClient::from_settings(settings)?;
    if !client.config().api_key_present() {
        return Err(anyhow!(
            "Fireworks API-Key fehlt. Setze FIREWORK_API_KEY oder FIREWORKS_API_KEY; build-spec braucht das LLM."
        ));
    }

    let request = ChatCompletionRequest::new(
        vec![
            ChatMessage::system(dbrain_builds::spec::BUILD_SPEC_SYSTEM_PROMPT),
            ChatMessage::user(dbrain_builds::spec::build_spec_user_prompt(
                &build_context.hero_name,
                understanding.as_deref(),
                &candidates.items,
                &build_context,
            )),
        ],
        client.config(),
    );
    let response = client.chat(&request)?;
    let llm_text = extract_ai_text(&response);
    let llm_spec = dbrain_builds::spec::parse_llm_spec_text(&llm_text)?;
    let ability_order = dbrain_builds::spec::ability_order_from_corpus(&corpus);
    let skill_order_found = ability_order.is_some();
    if !skill_order_found {
        warnings.push("skill_order missing in corpus build".to_string());
    }

    let mut assembled = dbrain_builds::spec::assemble_payload(
        build_context.hero_id,
        llm_spec,
        &candidates.name_to_id,
        ability_order.as_deref(),
    );
    let payload = serde_json::to_value(&assembled.payload)?;
    dbrain_reasoner::publish::validate_publish_items(pool, &payload).await?;
    warnings.append(&mut assembled.warnings);
    eprintln!(
        "build-spec: item_count={} understanding_found={} skill_order_found={}",
        candidates.items.len(),
        understanding_found,
        skill_order_found
    );
    for warning in warnings {
        eprintln!("warning: {warning}");
    }
    print_json(&assembled.payload)
}

async fn load_known_item_names(
    pool: &PgPool,
    mode: Option<brain_contracts::game_mode::GameMode>,
) -> Result<BTreeSet<String>> {
    let version = brain_storage::asset_mirror::latest_mirrored_client_version(pool).await?;
    let catalog = brain_storage::asset_mirror::load_mirrored_assets_with_receipt(
        pool,
        version,
        "items",
        Some("english"),
    )
    .await?;
    let rows = catalog
        .payload
        .as_array()
        .context("Aktueller Item-Katalog fehlt.")?;
    Ok(rows
        .iter()
        .filter(|raw| raw["type"] == "upgrade" && mode.is_none_or(|mode| mode.allows_item(raw)))
        .filter_map(|raw| raw["name"].as_str())
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
        .collect())
}

#[cfg(test)]
mod requested_build_tests {
    use super::*;
    #[test]
    fn requested_build_commands_separate_review_from_guarded_publication() {
        let regular = Cli::try_parse_from([
            "deadlock-brain",
            "publish-build-query",
            "--",
            "Warden Gun Build",
        ])
        .unwrap();
        assert!(matches!(regular.command, Commands::PublishBuildQuery(_)));
        let review =
            Cli::try_parse_from(["deadlock-brain", "review-build", "--", "Warden Gun Build"])
                .unwrap();
        assert!(matches!(review.command, Commands::ReviewBuild(_)));
    }
    #[test]
    fn requested_build_receipt_requires_real_completion() {
        use brain_contracts::feeds::*;
        let request = brain_feeds::build_publish::deterministic_request(BuildPublishRequest {
            contract_version: BUILD_PUBLISH_VERSION.into(),
            request_id: String::new(),
            hero_id: 25,
            hero_name: "Warden".into(),
            build_name: "Fixture".into(),
            payload: json!({"mod_categories":[]}),
            caller: "brain-test".into(),
        })
        .unwrap();
        for state in [
            BuildPublishState::Queued,
            BuildPublishState::Running,
            BuildPublishState::Succeeded { hero_build_id: 0 },
            BuildPublishState::Failed {
                error_class: BuildPublishErrorClass::RateLimited,
            },
            BuildPublishState::Succeeded { hero_build_id: 123 },
        ] {
            let success = matches!(state, BuildPublishState::Succeeded { hero_build_id: 123 });
            let receipt = PublicationReceipt::from_result(
                &request,
                Ok(BuildPublishStatus {
                    contract_version: BUILD_PUBLISH_VERSION.into(),
                    request_id: request.request_id.clone(),
                    request_sha256: request.request_sha256().unwrap(),
                    state,
                    submitted_at: 1,
                    updated_at: 1,
                }),
            )
            .unwrap();
            assert_eq!(receipt.ensure_success().is_ok(), success);
            assert_eq!(receipt.hero_build_id, success.then_some(123));
        }
        let timeout = PublicationReceipt::from_result(
            &request,
            Err(brain_feeds::build_publish::PublishError::Timeout),
        )
        .unwrap();
        assert!(timeout.ensure_success().is_err());
        assert_eq!(timeout.error_class.as_deref(), Some("timeout"));
        let json = serde_json::to_value(timeout).unwrap();
        assert_eq!(json["request_sha256"], request.request_sha256().unwrap());
    }

    fn saved_request() -> brain_contracts::feeds::BuildPublishRequest {
        brain_feeds::build_publish::deterministic_request(brain_contracts::feeds::BuildPublishRequest {
            contract_version: brain_contracts::feeds::BUILD_PUBLISH_VERSION.into(),
            request_id: String::new(),
            hero_id: 25,
            hero_name: "Warden".into(),
            build_name: "Gespeicherter Testbuild".into(),
            payload: json!({"mod_categories": [], "description": "Unveränderte KI-Beschreibung"}),
            caller: "deadlock-brain-reasoner".into(),
        })
        .unwrap()
    }

    fn resume_args(path: &std::path::Path) -> PublishResumeArgs {
        let cli = Cli::try_parse_from([
            std::ffi::OsStr::new("deadlock-brain"),
            std::ffi::OsStr::new("reason"),
            std::ffi::OsStr::new("resume-publish"),
            path.as_os_str(),
            std::ffi::OsStr::new("--json"),
        ])
        .unwrap();
        let Commands::Reason {
            target: ReasonCommands::ResumePublish(args),
        } = cli.command
        else {
            panic!("resume command missing");
        };
        args
    }

    fn saved_request_files(dir: &std::path::Path) -> usize {
        fs::read_dir(dir)
            .unwrap()
            .filter(|entry| entry.as_ref().unwrap().file_type().unwrap().is_file())
            .count()
    }

    #[test]
    fn saved_regular_publication_requires_matching_build_provenance() {
        assert!(SavedPublication::new(saved_request())
            .unwrap()
            .regular_build()
            .is_err());
        let build: dbrain_reasoner::BuildObject = serde_json::from_value(json!({
            "hero_id":25,"hero_name":"Warden","patch_tag":"fixture-patch","name":"Mechanikprüfung",
            "core":[{"item_id":1,"name":"Item 1","tier":1,"buy_phase":"Core","why":"Mechanik",
                "confidence":"High","imbue_target":null,"sell_priority":null,
                "sources":[{"kind":"Mechanic","detail":"API-Spielwert"}]}],
            "situations":[],"ability_order":[{"ability_id":101,"currency_type":2,"delta":-1}],
            "confidence":"Low","rationale":"Mechanikprüfung","family":null,"variants":[],"family_discovery":null
        })).unwrap();
        assert!(publish_request(&build, false).is_err());
        let mut saved = regular_saved_observation(10, 6000, 20.0);
        assert!(saved.regular_build().unwrap().is_some());
        saved.build.as_mut().unwrap()["core"][0]["why"] = json!("Andere Beschreibung");
        assert!(saved.regular_build().is_err());
        let review = SavedPublication::new(publish_request(&build, true).unwrap()).unwrap();
        assert!(review.regular_build().unwrap().is_none());
    }

    fn regular_saved_observation(run: i64, version: i64, value: f64) -> SavedPublication {
        let origin = dbrain_reasoner::publish::BuildDataOrigin {
            client_version: version,
            source_run_id: run,
            mirrored_at: 2000 + run,
            parser_revision: "fixture-parser".into(),
            manifest_document_id: 100 + run,
            manifest_sha256: "1".repeat(64),
            heroes_document_id: 200 + run,
            heroes_sha256: "2".repeat(64),
            items_document_id: 300 + run,
            items_sha256: "3".repeat(64),
        };
        let hero = serde_json::from_value(json!({
            "hero_id":25,"name":"Warden","archetype":"weapon","base_health":600.0,
            "level_curve":[],"purchase_bonuses":{"weapon":[],"spirit":[],"vitality":[]},"scaling":[],
            "weapon":{"bullet_damage":value,"shots_per_second":4.0,"clip_size":16.0,"reload_duration":2.0,"range":20.0,"falloff_start_range":20.0,"falloff_end_range":50.0,"sustained_dps":50.0},
            "damage_plan":{"weapon_dps":50.0,"spirit_dps":0.0,"weapon_share":1.0,"primary_axis":"Weapon"},
            "abilities":[{"ability_id":101,"class_name":"ability_fixture","slot":1,"roles":[],"scaling":[],"channel_time":null,"charges":1,"cooldown":10.0,"scaling_step":null,"damage_type":"Spirit","base_effect":60.0,"properties":{"Damage":60.0,"AbilityCooldown":10.0}}]
        })).unwrap();
        let items = vec![serde_json::from_value(json!({
            "item_id":1,"name":"Item 1","class_name":"item_1","slot":"Weapon","tier":2,"cost":1600,
            "is_active":false,"shopable":true,"disabled":false,"damage_axis":"Weapon","defense_kind":[],
            "properties":{"BaseAttackDamagePercent":10.0},"passive_properties":{},"condition":"None","proc_cooldown":null,"imbueable":false
        })).unwrap()];
        let snapshots = [
            dbrain_reasoner::DeltaTarget::Hero(25),
            dbrain_reasoner::DeltaTarget::Ability(101),
            dbrain_reasoner::DeltaTarget::Item(1),
        ]
        .into_iter()
        .map(|target| {
            let source = if matches!(target, dbrain_reasoner::DeltaTarget::Hero(_)) {
                format!(
                    "deadlock_assets_api/hero#build-data={}",
                    serde_json::to_string(&origin).unwrap()
                )
            } else {
                "deadlock_assets_api/item_or_ability".into()
            };
            dbrain_reasoner::PatchSnapshot {
                target,
                name: "API".into(),
                fields: [(
                    "value".into(),
                    dbrain_reasoner::SnapshotField {
                        value: 10.0,
                        fetched_at: Some(origin.mirrored_at as f64),
                        source,
                        label: "Spielwert".into(),
                    },
                )]
                .into_iter()
                .collect(),
            }
        })
        .collect::<Vec<_>>();
        let meta = dbrain_reasoner::meta::MetaIndexWithSources {
            index: dbrain_reasoner::MetaIndex {
                by_item: Default::default(),
                sample_ok: Default::default(),
            },
            author_builds: vec![],
            hero_ability_orders: Default::default(),
            core_layouts: Default::default(),
            combinations: Default::default(),
            population: Default::default(),
            observations: vec![],
            family: None,
        };
        let cfg = dbrain_reasoner::ReasonerConfig {
            patch_tag: "fixture-patch".into(),
            use_ai: false,
            ..Default::default()
        };
        let build = dbrain_reasoner::plan_build(&hero, &items, &meta, &[], &snapshots, &cfg)
            .unwrap()
            .build;
        let mut saved = SavedPublication::new(publish_request(&build, false).unwrap()).unwrap();
        saved.build = Some(serde_json::to_value(build).unwrap());
        saved
    }

    #[tokio::test]
    async fn unchanged_observation_reuses_exact_saved_request_and_provenance_for_resume() {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("publish.json");
        let original = regular_saved_observation(10, 6000, 20.0);
        let observed = regular_saved_observation(20, 6000, 20.0);
        assert_eq!(observed.request, original.request);
        assert_ne!(observed.build, original.build);
        original.persist(&path).unwrap();
        let bytes = fs::read(&path).unwrap();
        let inode = fs::metadata(&path).unwrap().ino();
        let reused = observed.persist(&path).unwrap();
        assert_eq!(reused, original);
        assert_eq!(fs::read(&path).unwrap(), bytes);
        assert_eq!(fs::metadata(&path).unwrap().ino(), inode);
        let expected = &original;
        resume_publication_with(&resume_args(&path), |saved, _| async move {
            assert_eq!(&saved, expected);
            let restored = saved.regular_build()?.unwrap();
            dbrain_reasoner::publish::validate_build_provenance(&restored)?;
            PublicationReceipt::from_result(
                &saved.request,
                Err(brain_feeds::build_publish::PublishError::Timeout),
            )
        })
        .await
        .unwrap();
        for changed in [
            regular_saved_observation(30, 6001, 20.0),
            regular_saved_observation(30, 6000, 21.0),
        ] {
            assert_ne!(changed.request.request_id, original.request.request_id);
            assert_ne!(
                changed.default_path().unwrap(),
                original.default_path().unwrap()
            );
            assert!(changed.persist(&path).is_err());
            let new_path = dir
                .path()
                .join(format!("{}.json", changed.request.request_id));
            assert_eq!(changed.persist(&new_path).unwrap(), changed);
            assert_eq!(SavedPublication::read(&new_path).unwrap(), changed);
        }
        assert_eq!(fs::read(&path).unwrap(), bytes);
        assert_eq!(saved_request_files(dir.path()), 3);
        assert_eq!(saved_request_files(&dir.path().join("builds")), 3);
        let stored: Value = serde_json::from_slice(&bytes).unwrap();
        assert!(stored.get("build").is_none());
        let build_path = dir
            .path()
            .join("builds")
            .join(format!("{}.json", stored["build_sha256"].as_str().unwrap()));
        let mut tampered = fs::read(&build_path).unwrap();
        tampered[0] = b' ';
        fs::set_permissions(&build_path, fs::Permissions::from_mode(0o600)).unwrap();
        fs::write(&build_path, tampered).unwrap();
        assert!(SavedPublication::read(&path).is_err());
    }

    fn review_saved_observation(run: i64) -> SavedPublication {
        let regular = regular_saved_observation(run, 6000, 20.0);
        let build = regular.regular_build().unwrap().unwrap();
        let mut saved = SavedPublication::new(publish_request(&build, true).unwrap()).unwrap();
        saved.build = regular.build;
        saved
    }

    #[test]
    fn identical_review_reuses_legacy_saved_request_without_build() {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("publish.json");
        let observed = review_saved_observation(10);
        let mut legacy = observed.clone();
        legacy.build = None;
        legacy.persist(&path).unwrap();
        let bytes = fs::read(&path).unwrap();
        assert!(serde_json::from_slice::<Value>(&bytes)
            .unwrap()
            .get("build")
            .is_none());
        let inode = fs::metadata(&path).unwrap().ino();
        assert_eq!(observed.persist(&path).unwrap(), legacy);
        assert_eq!(SavedPublication::read(&path).unwrap(), legacy);
        assert_eq!(fs::read(&path).unwrap(), bytes);
        assert_eq!(fs::metadata(&path).unwrap().ino(), inode);
        assert_eq!(
            fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o400
        );
        assert_eq!(saved_request_files(dir.path()), 1);
    }

    #[test]
    fn identical_review_reuses_exact_saved_request_despite_provenance_only_difference() {
        use std::os::unix::fs::MetadataExt;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("publish.json");
        let original = review_saved_observation(10);
        let observed = review_saved_observation(20);
        assert_eq!(observed.request, original.request);
        assert_ne!(observed.build, original.build);
        assert!(original.regular_build().unwrap().is_none());
        assert!(observed.regular_build().unwrap().is_none());
        original.persist(&path).unwrap();
        let bytes = fs::read(&path).unwrap();
        let inode = fs::metadata(&path).unwrap().ino();
        assert_eq!(observed.persist(&path).unwrap(), original);
        let mut changed_request = observed.request.clone();
        changed_request.payload["description"] = json!("Andere Review-Beschreibung");
        let changed = SavedPublication::new(
            brain_feeds::build_publish::deterministic_request(changed_request).unwrap(),
        )
        .unwrap();
        assert!(changed.persist(&path).is_err());
        assert_eq!(SavedPublication::read(&path).unwrap(), original);
        assert_eq!(fs::read(&path).unwrap(), bytes);
        assert_eq!(fs::metadata(&path).unwrap().ino(), inode);
        assert_eq!(saved_request_files(dir.path()), 1);
        assert_eq!(saved_request_files(&dir.path().join("builds")), 1);
    }

    #[test]
    fn regular_publication_cannot_reuse_changed_game_values_with_mismatched_provenance() {
        use std::os::unix::fs::MetadataExt;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("publish.json");
        let original = regular_saved_observation(10, 6000, 20.0);
        original.persist(&path).unwrap();
        let bytes = fs::read(&path).unwrap();
        let inode = fs::metadata(&path).unwrap().ino();
        for mut changed in [
            regular_saved_observation(20, 6001, 20.0),
            regular_saved_observation(20, 6000, 21.0),
        ] {
            assert!(changed.regular_build().unwrap().is_some());
            assert_ne!(changed.request, original.request);
            assert!(changed.persist(&path).is_err());
            changed.request = original.request.clone();
            changed.request_sha256 = original.request_sha256.clone();
            changed.validate().unwrap();
            assert!(changed.regular_build().is_err());
            assert!(changed.persist(&path).is_err());
            let mismatched_path = dir.path().join("mismatched.json");
            changed.persist(&mismatched_path).unwrap();
            let mismatched_bytes = fs::read(&mismatched_path).unwrap();
            assert!(original.persist(&mismatched_path).is_err());
            assert_eq!(fs::read(&mismatched_path).unwrap(), mismatched_bytes);
            fs::remove_file(&mismatched_path).unwrap();
        }
        assert_eq!(SavedPublication::read(&path).unwrap(), original);
        assert_eq!(fs::read(&path).unwrap(), bytes);
        assert_eq!(fs::metadata(&path).unwrap().ino(), inode);
        assert_eq!(saved_request_files(dir.path()), 1);
    }

    #[test]
    fn saved_publication_is_atomic_read_only_and_never_overwrites_an_existing_request() {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("publish.json");
        let saved = SavedPublication::new(saved_request()).unwrap();
        saved.persist(&path).unwrap();
        let bytes = fs::read(&path).unwrap();
        let inode = fs::metadata(&path).unwrap().ino();
        assert_eq!(
            fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o400
        );
        saved.persist(&path).unwrap();
        assert_eq!(fs::metadata(&path).unwrap().ino(), inode);
        assert_eq!(SavedPublication::read(&path).unwrap(), saved);
        assert_eq!(fs::read(&path).unwrap(), bytes);
        assert_ne!(
            saved
                .request
                .request_id
                .strip_prefix("brain-publish-")
                .unwrap(),
            saved.request_sha256
        );
        let mut changed = saved.request.clone();
        changed.payload["description"] = json!("Neu berechnete KI-Beschreibung");
        let changed = SavedPublication::new(
            brain_feeds::build_publish::deterministic_request(changed).unwrap(),
        )
        .unwrap();
        assert!(changed.persist(&path).is_err());
        assert_eq!(fs::read(&path).unwrap(), bytes);
        assert_eq!(saved_request_files(dir.path()), 1);
    }

    #[test]
    fn concurrent_saves_only_accept_the_same_validated_request() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("publish.json");
        let saved = SavedPublication::new(saved_request()).unwrap();
        std::thread::scope(|scope| {
            let first = scope.spawn(|| saved.persist(&path));
            let second = scope.spawn(|| saved.persist(&path));
            first.join().unwrap().unwrap();
            second.join().unwrap().unwrap();
        });
        assert_eq!(SavedPublication::read(&path).unwrap(), saved);
        assert_eq!(saved_request_files(dir.path()), 1);
    }

    #[test]
    fn saved_publication_rejects_changed_hash_identity_version_and_payload() {
        let dir = tempfile::tempdir().unwrap();
        let saved = SavedPublication::new(saved_request()).unwrap();
        let mut variants = Vec::new();
        let mut changed = serde_json::to_value(&saved).unwrap();
        changed["request"]["payload"]["description"] = json!("synthetic-private-content");
        variants.push(changed.clone());
        let request: brain_contracts::feeds::BuildPublishRequest =
            serde_json::from_value(changed["request"].clone()).unwrap();
        changed["request_sha256"] = json!(request.request_sha256().unwrap());
        variants.push(changed);
        for (field, value) in [
            ("contract_version", json!("synthetic-private-content")),
            ("request_id", json!("different-valid-id")),
            ("hero_id", json!(0)),
            ("payload", json!([])),
        ] {
            let mut changed = serde_json::to_value(&saved).unwrap();
            changed["request"][field] = value;
            variants.push(changed);
        }
        let mut changed = serde_json::to_value(&saved).unwrap();
        changed["request_sha256"] = json!("0".repeat(64));
        variants.push(changed);
        let mut changed = serde_json::to_value(&saved).unwrap();
        changed["unknown"] = json!("synthetic-private-content");
        variants.push(changed);
        for (index, changed) in variants.iter().enumerate() {
            let path = dir.path().join(format!("changed-{index}.json"));
            fs::write(&path, serde_json::to_vec(changed).unwrap()).unwrap();
            let error = SavedPublication::read(&path).unwrap_err().to_string();
            assert!(!error.contains("synthetic-private-content"));
            assert!(!error.contains("line"));
            assert!(!error.contains("column"));
        }
    }

    #[test]
    fn saved_publication_rejects_oversized_non_regular_and_malformed_files() {
        use std::os::unix::fs::symlink;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("oversized.json");
        let file = fs::File::create(&path).unwrap();
        file.set_len(MAX_PUBLISH_STATE_BYTES + 1).unwrap();
        assert!(SavedPublication::read(&path).is_err());
        assert!(SavedPublication::read(dir.path()).is_err());
        let valid = dir.path().join("valid.json");
        SavedPublication::new(saved_request())
            .unwrap()
            .persist(&valid)
            .unwrap();
        let link = dir.path().join("link.json");
        symlink(&valid, &link).unwrap();
        assert!(SavedPublication::read(&link).is_err());
        let invalid = dir.path().join("invalid.json");
        fs::write(&invalid, "{synthetic-private-content").unwrap();
        let error = SavedPublication::read(&invalid).unwrap_err().to_string();
        assert!(!error.is_empty());
        assert!(!error.contains("synthetic-private-content"));
        assert!(SavedPublication::read(&dir.path().join("missing.json")).is_err());
        assert!(SavedPublication::new(saved_request())
            .unwrap()
            .persist(&invalid)
            .is_err());
        assert_eq!(
            fs::read_to_string(invalid).unwrap(),
            "{synthetic-private-content"
        );
    }

    #[tokio::test]
    async fn resume_uses_the_saved_request_without_build_or_ai_recalculation() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("publish.json");
        let request = saved_request();
        SavedPublication::new(request.clone())
            .unwrap()
            .persist(&path)
            .unwrap();
        let bytes = fs::read(&path).unwrap();
        let expected_request = &request;
        let expected_path = &path;
        let receipt =
            resume_publication_with(&resume_args(&path), |saved, saved_path| async move {
                assert_eq!(&saved.request, expected_request);
                assert_eq!(
                    saved.request_sha256,
                    expected_request.request_sha256().unwrap()
                );
                assert_eq!(&saved_path, expected_path);
                let mut receipt = PublicationReceipt::from_result(
                    &saved.request,
                    Ok(brain_contracts::feeds::BuildPublishStatus {
                        contract_version: expected_request.contract_version.clone(),
                        request_id: expected_request.request_id.clone(),
                        request_sha256: saved.request_sha256,
                        state: brain_contracts::feeds::BuildPublishState::Succeeded {
                            hero_build_id: 456,
                        },
                        submitted_at: 1,
                        updated_at: 2,
                    }),
                )?;
                receipt.request_path = Some(saved_path);
                Ok(receipt)
            })
            .await
            .unwrap();
        receipt.ensure_success().unwrap();
        assert_eq!(receipt.request_id, request.request_id);
        assert_eq!(receipt.request_sha256, request.request_sha256().unwrap());
        assert_eq!(receipt.request_path, Some(path.clone()));
        assert_eq!(fs::read(path).unwrap(), bytes);
    }

    #[tokio::test]
    async fn resume_rejects_a_modified_file_before_any_publish_call() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("publish.json");
        let saved = SavedPublication::new(saved_request()).unwrap();
        let mut changed = serde_json::to_value(saved).unwrap();
        changed["request"]["payload"]["description"] = json!("synthetic-private-content");
        fs::write(&path, serde_json::to_vec(&changed).unwrap()).unwrap();
        let called = std::cell::Cell::new(false);
        let publish_called = &called;
        let result = resume_publication_with(&resume_args(&path), |_, _| async move {
            publish_called.set(true);
            Err(anyhow!("unexpected publish call"))
        })
        .await;
        assert!(result.is_err());
        assert!(!called.get());
        assert!(!result
            .unwrap_err()
            .to_string()
            .contains("synthetic-private-content"));
    }

    #[tokio::test]
    async fn cli_resume_bypasses_settings_database_and_reasoner_initialization() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("publish.json");
        SavedPublication::new(saved_request())
            .unwrap()
            .persist(&path)
            .unwrap();
        let bytes = fs::read(&path).unwrap();
        let mut args = resume_args(&path);
        args.publish_connection.endpoint = "http://example.invalid".into();
        assert!(run(Cli {
            command: Commands::Reason {
                target: ReasonCommands::ResumePublish(args)
            }
        })
        .await
        .is_err());
        assert_eq!(fs::read(path).unwrap(), bytes);
    }

    #[test]
    fn blocked_publication_keeps_its_output_and_returns_an_error() {
        for variants in [0, 2] {
            let mut output = None;
            let mut published = false;
            let result = (|| {
                blocked_requested_build("Warden", variants, |value| {
                    output = Some(value.clone());
                    Ok(())
                })?;
                published = true;
                Ok::<_, anyhow::Error>(())
            })();
            assert!(result.is_err());
            assert!(!published);
            let output = output.unwrap();
            assert_eq!(output["status"], "BLOCKED");
            assert_eq!(output["hero_name"], "Warden");
            assert_eq!(output["review"], false);
            assert_eq!(output["hero_build_id"], Value::Null);
            assert_eq!(output["core"], json!([]));
            assert_eq!(output["situations"], json!([]));
            assert!(output["message"]
                .as_str()
                .is_some_and(|message| !message.is_empty()));
        }
    }

    #[test]
    fn unconfirmed_rate_limit_is_visible_with_the_saved_resume_path() {
        let request = saved_request();
        let mut receipt = PublicationReceipt::from_result(
            &request,
            Err(brain_feeds::build_publish::PublishError::RateLimited),
        )
        .unwrap();
        receipt.request_path = Some(PathBuf::from("/synthetic/publish.json"));
        assert!(receipt.ensure_success().is_err());
        let receipt = serde_json::to_value(receipt).unwrap();
        assert_eq!(receipt["status"], "UNCONFIRMED");
        assert_eq!(receipt["error_class"], "rate_limited");
        assert_eq!(receipt["request_path"], "/synthetic/publish.json");
        assert_eq!(receipt["request_sha256"], request.request_sha256().unwrap());
    }

    #[test]
    fn publish_credentials_follow_steam_precedence_and_reject_invalid_primary() {
        let values = [
            "INTERNAL_API_TOKEN",
            "STEAM_INTERNAL_API_TOKEN",
            "TWITCH_INTERNAL_API_TOKEN",
        ]
        .map(|name| (name.into(), zeroize::Zeroizing::new(name.into())));
        assert_eq!(
            select_publish_token(&values).unwrap().as_str(),
            "TWITCH_INTERNAL_API_TOKEN"
        );
        assert_eq!(
            select_publish_token(&values[..2]).unwrap().as_str(),
            "STEAM_INTERNAL_API_TOKEN"
        );
        assert_eq!(
            select_publish_token(&values[..1]).unwrap().as_str(),
            "INTERNAL_API_TOKEN"
        );
        assert!(select_publish_token(&[]).is_err());
        assert!(select_publish_token(&[
            (
                "TWITCH_INTERNAL_API_TOKEN".into(),
                zeroize::Zeroizing::new(String::new())
            ),
            values[0].clone()
        ])
        .is_err());
    }
}

const MAX_PUBLISH_STATE_BYTES: u64 = 300 * 1024;
const MAX_PUBLISH_BUILD_BYTES: u64 = 16 * 1024 * 1024;

#[derive(Debug, serde::Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct StoredPublication {
    request_sha256: String,
    request: brain_contracts::feeds::BuildPublishRequest,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    build: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    build_sha256: Option<String>,
}

fn temporary_publish_path(parent: &std::path::Path) -> Result<PathBuf> {
    static TEMP_SEQUENCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| anyhow!("Veröffentlichungsanfrage konnte nicht gespeichert werden."))?
        .as_nanos();
    let sequence = TEMP_SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Ok(parent.join(format!(
        ".brain-publish-{}-{nonce}-{sequence}.tmp",
        process::id()
    )))
}

fn saved_build_path(parent: &std::path::Path, sha256: &str) -> Result<PathBuf> {
    if sha256.len() != 64
        || !sha256
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        anyhow::bail!("Gespeicherte Build-Provenienz ist ungültig.");
    }
    Ok(parent.join("builds").join(format!("{sha256}.json")))
}

fn read_saved_build(parent: &std::path::Path, sha256: &str) -> Result<Value> {
    use sha2::Digest;
    let unreadable = || anyhow!("Gespeicherte Build-Provenienz ist nicht lesbar.");
    let file = fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(saved_build_path(parent, sha256)?)
        .map_err(|_| unreadable())?;
    let metadata = file.metadata().map_err(|_| unreadable())?;
    if !metadata.is_file() || metadata.len() > MAX_PUBLISH_BUILD_BYTES {
        anyhow::bail!("Gespeicherte Build-Provenienz muss eine begrenzte normale Datei sein.");
    }
    let mut bytes = Vec::new();
    file.take(MAX_PUBLISH_BUILD_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| unreadable())?;
    if bytes.len() as u64 > MAX_PUBLISH_BUILD_BYTES
        || hex::encode(sha2::Sha256::digest(&bytes)) != sha256
    {
        anyhow::bail!("Gespeicherte Build-Provenienz ist ungültig oder verändert.");
    }
    serde_json::from_slice(&bytes)
        .map_err(|_| anyhow!("Gespeicherte Build-Provenienz ist ungültig."))
}

fn persist_saved_build(
    parent: &std::path::Path,
    build: &Value,
) -> Result<(String, Option<PathBuf>)> {
    use sha2::Digest;
    use std::io::Write;
    let failed = || anyhow!("Build-Provenienz konnte nicht gespeichert werden.");
    let bytes = serde_json::to_vec(build).map_err(|_| failed())?;
    if bytes.len() as u64 > MAX_PUBLISH_BUILD_BYTES {
        anyhow::bail!("Build-Provenienz ist zu groß zum Speichern.");
    }
    let sha256 = hex::encode(sha2::Sha256::digest(&bytes));
    let path = saved_build_path(parent, &sha256)?;
    let directory = parent.join("builds");
    fs::create_dir_all(&directory).map_err(|_| failed())?;
    let temporary = temporary_publish_path(&directory)?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o400)
        .open(&temporary)
        .map_err(|_| failed())?;
    let mut created = None;
    let result = (|| {
        file.write_all(&bytes)
            .and_then(|_| file.sync_all())
            .map_err(|_| failed())?;
        match fs::hard_link(&temporary, &path) {
            Ok(()) => created = Some(path.clone()),
            Err(error) if error.kind() == ErrorKind::AlreadyExists => {
                read_saved_build(parent, &sha256)?;
            }
            Err(_) => return Err(failed()),
        }
        fs::File::open(&directory)
            .and_then(|directory| directory.sync_all())
            .map_err(|_| failed())
    })();
    drop(file);
    let removed = fs::remove_file(temporary).map_err(|_| failed());
    if let Err(error) = result.and(removed) {
        discard_saved_build(created.as_deref());
        return Err(error);
    }
    Ok((sha256, created))
}

fn discard_saved_build(created: Option<&std::path::Path>) {
    if let Some(path) = created {
        if fs::remove_file(path).is_ok() {
            if let Some(directory) = path.parent() {
                let _ = fs::File::open(directory).and_then(|directory| directory.sync_all());
            }
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct SavedPublication {
    request_sha256: String,
    request: brain_contracts::feeds::BuildPublishRequest,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    build: Option<Value>,
}

impl SavedPublication {
    fn new(request: brain_contracts::feeds::BuildPublishRequest) -> Result<Self> {
        let saved = Self {
            request_sha256: request
                .request_sha256()
                .map_err(|_| anyhow!("Veröffentlichungsanfrage ist ungültig."))?,
            request,
            build: None,
        };
        saved.validate()?;
        Ok(saved)
    }

    fn validate(&self) -> Result<()> {
        let invalid =
            || anyhow!("Gespeicherte Veröffentlichungsanfrage ist ungültig oder verändert.");
        self.request.validate().map_err(|_| invalid())?;
        if self.request.request_sha256().map_err(|_| invalid())? != self.request_sha256
            || brain_feeds::build_publish::deterministic_request(self.request.clone())
                .map_err(|_| invalid())?
                .request_id
                != self.request.request_id
        {
            return Err(invalid());
        }
        Ok(())
    }

    fn regular_build(&self) -> Result<Option<dbrain_reasoner::BuildObject>> {
        self.validate()?;
        if self.request.caller == "deadlock-brain-review"
            && self.request.build_name.starts_with("[REVIEW] ")
        {
            return Ok(None);
        }
        let build: dbrain_reasoner::BuildObject = serde_json::from_value(
            self.build.clone().context("Gespeicherte Anfrage enthält keine Build-Provenienz. Bitte einen aktuellen Build berechnen.")?,
        ).map_err(|_| anyhow!("Gespeicherte Build-Provenienz ist ungültig."))?;
        if publish_request(&build, false)? != self.request {
            anyhow::bail!(
                "Gespeicherter Build stimmt nicht mit der Veröffentlichungsanfrage überein."
            );
        }
        Ok(Some(build))
    }

    fn read(path: &std::path::Path) -> Result<Self> {
        let file = fs::OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
            .open(path)
            .map_err(|_| anyhow!("Gespeicherte Veröffentlichungsanfrage ist nicht lesbar."))?;
        let metadata = file
            .metadata()
            .map_err(|_| anyhow!("Gespeicherte Veröffentlichungsanfrage ist nicht prüfbar."))?;
        if !metadata.is_file() || metadata.len() > MAX_PUBLISH_STATE_BYTES {
            anyhow::bail!(
                "Gespeicherte Veröffentlichungsanfrage muss eine begrenzte normale Datei sein."
            );
        }
        let mut bytes = Vec::new();
        file.take(MAX_PUBLISH_STATE_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| anyhow!("Gespeicherte Veröffentlichungsanfrage ist nicht lesbar."))?;
        if bytes.len() as u64 > MAX_PUBLISH_STATE_BYTES {
            anyhow::bail!("Gespeicherte Veröffentlichungsanfrage ist zu groß.");
        }
        let stored: StoredPublication = serde_json::from_slice(&bytes)
            .map_err(|_| anyhow!("Gespeicherte Veröffentlichungsanfrage ist ungültig."))?;
        let build = match (stored.build, stored.build_sha256) {
            (build, None) => build,
            (None, Some(sha256)) => {
                let parent = path
                    .parent()
                    .filter(|parent| !parent.as_os_str().is_empty())
                    .unwrap_or_else(|| std::path::Path::new("."));
                Some(read_saved_build(parent, &sha256)?)
            }
            (Some(_), Some(_)) => {
                anyhow::bail!("Gespeicherte Veröffentlichungsanfrage ist ungültig.")
            }
        };
        let saved = Self {
            request_sha256: stored.request_sha256,
            request: stored.request,
            build,
        };
        saved.validate()?;
        Ok(saved)
    }

    fn persist(&self, path: &std::path::Path) -> Result<Self> {
        self.validate()?;
        let parent = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or_else(|| std::path::Path::new("."));
        let unavailable =
            || anyhow!("Verzeichnis für Veröffentlichungsanfragen ist nicht verfügbar.");
        fs::create_dir_all(parent).map_err(|_| unavailable())?;
        let lock = fs::File::open(parent).map_err(|_| unavailable())?;
        lock.lock().map_err(|_| unavailable())?;
        if fs::symlink_metadata(path).is_ok() {
            return self.reuse_existing(path);
        }
        let (build_sha256, created) = match self.build.as_ref() {
            Some(build) => {
                let (sha256, created) = persist_saved_build(parent, build)?;
                (Some(sha256), created)
            }
            None => (None, None),
        };
        let mut linked = false;
        let selected = self.write_request(path, parent, build_sha256, &mut linked);
        if !linked && !matches!(&selected, Ok(selected) if selected.build == self.build) {
            discard_saved_build(created.as_deref());
        }
        selected
    }

    fn reuse_existing(&self, path: &std::path::Path) -> Result<Self> {
        let existing = Self::read(path)?;
        if existing != *self {
            let equivalent = if existing.request == self.request {
                match (existing.regular_build()?, self.regular_build()?) {
                    (Some(first), Some(second)) => {
                        dbrain_reasoner::publish::equivalent_calculated_builds(&first, &second)?
                    }
                    (None, None) => true,
                    _ => false,
                }
            } else {
                false
            };
            if !equivalent {
                anyhow::bail!("Die vorhandene Veröffentlichungsanfrage stimmt nicht überein. Sie wurde nicht überschrieben.");
            }
        }
        Ok(existing)
    }

    fn write_request(
        &self,
        path: &std::path::Path,
        parent: &std::path::Path,
        build_sha256: Option<String>,
        linked: &mut bool,
    ) -> Result<Self> {
        use std::io::Write;
        let stored = StoredPublication {
            request_sha256: self.request_sha256.clone(),
            request: self.request.clone(),
            build: None,
            build_sha256,
        };
        let bytes = serde_json::to_vec(&stored)
            .map_err(|_| anyhow!("Veröffentlichungsanfrage konnte nicht gespeichert werden."))?;
        if bytes.len() as u64 > MAX_PUBLISH_STATE_BYTES {
            anyhow::bail!("Veröffentlichungsanfrage ist zu groß zum Speichern.");
        }
        let temporary = temporary_publish_path(parent)?;
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o400)
            .open(&temporary)
            .map_err(|_| anyhow!("Veröffentlichungsanfrage konnte nicht gespeichert werden."))?;
        let result = (|| {
            file.write_all(&bytes)
                .and_then(|_| file.sync_all())
                .map_err(|_| {
                    anyhow!("Veröffentlichungsanfrage konnte nicht gespeichert werden.")
                })?;
            let selected = match fs::hard_link(&temporary, path) {
                Ok(()) => {
                    *linked = true;
                    self.clone()
                }
                Err(error) if error.kind() == ErrorKind::AlreadyExists => {
                    self.reuse_existing(path)?
                }
                Err(_) => {
                    anyhow::bail!("Veröffentlichungsanfrage konnte nicht gespeichert werden.")
                }
            };
            if fs::File::open(parent)
                .and_then(|directory| directory.sync_all())
                .is_err()
            {
                if *linked
                    && fs::remove_file(path).is_ok()
                    && fs::File::open(parent)
                        .and_then(|directory| directory.sync_all())
                        .is_ok()
                {
                    *linked = false;
                }
                anyhow::bail!(
                    "Veröffentlichungsanfrage konnte nicht dauerhaft gespeichert werden."
                );
            }
            Ok(selected)
        })();
        drop(file);
        let removed = fs::remove_file(temporary)
            .map_err(|_| anyhow!("Temporäre Veröffentlichungsdatei konnte nicht entfernt werden."));
        let selected = result?;
        removed?;
        Ok(selected)
    }

    fn default_path(&self) -> Result<PathBuf> {
        let state_home = std::env::var_os("XDG_STATE_HOME")
            .filter(|path| !path.is_empty())
            .map(PathBuf::from)
            .map(Ok)
            .unwrap_or_else(|| {
                std::env::var_os("HOME")
                    .filter(|path| !path.is_empty())
                    .map(|home| PathBuf::from(home).join(".local/state"))
                    .context("Verzeichnis für Veröffentlichungsanfragen fehlt.")
            })?;
        if !state_home.is_absolute() {
            anyhow::bail!("Verzeichnis für Veröffentlichungsanfragen muss absolut sein.");
        }
        Ok(state_home
            .join("deadlock-brain/build-publish")
            .join(format!("{}.json", self.request.request_id)))
    }
}

#[derive(Debug, Serialize)]
struct PublicationReceipt {
    contract_version: String,
    request_id: String,
    request_sha256: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_path: Option<PathBuf>,
    status: &'static str,
    hero_build_id: Option<u64>,
    error_class: Option<String>,
}

impl PublicationReceipt {
    fn from_result(
        request: &brain_contracts::feeds::BuildPublishRequest,
        result: std::result::Result<
            brain_contracts::feeds::BuildPublishStatus,
            brain_feeds::build_publish::PublishError,
        >,
    ) -> Result<Self> {
        use brain_contracts::feeds::BuildPublishState;
        use brain_feeds::build_publish::PublishError;
        let hash = request
            .request_sha256()
            .map_err(|_| anyhow!("Veröffentlichungsanfrage ist ungültig."))?;
        let result = result.and_then(|status| {
            status
                .validate_for(&request.request_id, Some(&hash))
                .map_err(PublishError::InvalidResponse)?;
            Ok(status)
        });
        let (status, hero_build_id, error_class) = match result {
            Ok(status) => match status.state {
                BuildPublishState::Succeeded { hero_build_id } => {
                    ("DONE", Some(hero_build_id), None)
                }
                BuildPublishState::Failed { error_class } => (
                    "FAILED",
                    None,
                    Some(
                        serde_json::to_value(error_class)?
                            .as_str()
                            .unwrap_or("internal")
                            .to_string(),
                    ),
                ),
                _ => ("UNCONFIRMED", None, Some("timeout".into())),
            },
            Err(error) => {
                let class = match error {
                    PublishError::RateLimited => "rate_limited",
                    PublishError::Timeout => "timeout",
                    PublishError::Unauthorized => "unauthorized",
                    PublishError::Conflict => "conflict",
                    PublishError::NotFound => "not_found",
                    PublishError::InvalidRequest(_) => "rejected",
                    PublishError::InvalidResponse(_) => "invalid_response",
                    PublishError::Unavailable(_) => "steam_unavailable",
                };
                ("UNCONFIRMED", None, Some(class.into()))
            }
        };
        Ok(Self {
            contract_version: request.contract_version.clone(),
            request_id: request.request_id.clone(),
            request_sha256: hash,
            request_path: None,
            status,
            hero_build_id,
            error_class,
        })
    }

    fn ensure_success(&self) -> Result<()> {
        if self.status == "DONE" && self.hero_build_id.is_some_and(|id| id > 0) {
            Ok(())
        } else {
            Err(anyhow!("Veröffentlichung nicht bestätigt ({}). Anfrage {} bleibt unverändert; nicht als neuen Build erneut senden.", self.error_class.as_deref().unwrap_or("internal"), self.request_id))
        }
    }
}

fn publish_request(
    build: &dbrain_reasoner::BuildObject,
    review: bool,
) -> Result<brain_contracts::feeds::BuildPublishRequest> {
    let payload = if review {
        dbrain_reasoner::publish::review_publish_task_payload(build)?
    } else {
        dbrain_reasoner::publish::validate_publish_input(build)?;
        dbrain_reasoner::publish::publish_task_payload(build)
    };
    let request = brain_contracts::feeds::BuildPublishRequest {
        contract_version: brain_contracts::feeds::BUILD_PUBLISH_VERSION.into(),
        request_id: String::new(),
        hero_id: u32::try_from(build.hero_id).context("Ungültiger Held für Veröffentlichung.")?,
        hero_name: build.hero_name.clone(),
        build_name: payload
            .get("name")
            .and_then(Value::as_str)
            .context("Buildname fehlt.")?
            .into(),
        payload,
        caller: if review {
            "deadlock-brain-review".into()
        } else {
            dbrain_reasoner::publish::publication_caller(build)?
        },
    };
    brain_feeds::build_publish::deterministic_request(request)
        .map_err(|_| anyhow!("Veröffentlichungsanfrage ist ungültig."))
}

fn select_publish_token(
    values: &[(String, zeroize::Zeroizing<String>)],
) -> Result<zeroize::Zeroizing<String>> {
    for name in [
        "TWITCH_INTERNAL_API_TOKEN",
        "STEAM_INTERNAL_API_TOKEN",
        "INTERNAL_API_TOKEN",
    ] {
        if let Some((_, token)) = values.iter().find(|(key, _)| key == name) {
            if token.is_empty()
                || token.len() > 4096
                || !token.bytes().all(|byte| byte.is_ascii_graphic())
            {
                return Err(anyhow!("Interner Dienstzugang in Infisical ist ungültig."));
            }
            return Ok(token.clone());
        }
    }
    Err(anyhow!(
        "Interner Dienstzugang fehlt in Infisical. Es wurde nichts veröffentlicht."
    ))
}

async fn publish_build_http(
    pool: &PgPool,
    build: &dbrain_reasoner::BuildObject,
    review: bool,
    connection: &PublishConnectionArgs,
    wait_seconds: u64,
) -> Result<PublicationReceipt> {
    if !(1..=120).contains(&wait_seconds) {
        anyhow::bail!("Wartezeit muss zwischen 1 und 120 Sekunden liegen.");
    }
    if !review {
        dbrain_reasoner::publish::validate_publish_current(pool, build).await?;
    }
    let mut saved = SavedPublication::new(publish_request(build, review)?)?;
    saved.build = Some(serde_json::to_value(build)?);
    let request_path = saved.default_path()?;
    let saved = saved.persist(&request_path)?;
    publish_saved_http(Some(pool), saved, request_path, connection, wait_seconds).await
}

async fn publish_saved_http(
    pool: Option<&PgPool>,
    saved: SavedPublication,
    request_path: PathBuf,
    connection: &PublishConnectionArgs,
    wait_seconds: u64,
) -> Result<PublicationReceipt> {
    use brain_feeds::build_publish::HttpBuildPublishClient;
    use std::time::Duration;
    saved.validate()?;
    if !(1..=120).contains(&wait_seconds) {
        anyhow::bail!("Wartezeit muss zwischen 1 und 120 Sekunden liegen.");
    }
    eprintln!(
        "Gespeicherte Veröffentlichungsanfrage: {}",
        request_path.display()
    );
    let build = saved.regular_build()?;
    if let Some(build) = build {
        let owned_pool;
        let pool = match pool {
            Some(pool) => pool,
            None => {
                owned_pool = deadlock_brain_core::pg::pg_pool_read_only().await?;
                &owned_pool
            }
        };
        dbrain_reasoner::publish::validate_publish_current(pool, &build).await?;
    }
    let request = saved.request;
    let endpoint = connection.endpoint.clone();
    tokio::task::spawn_blocking(move || {
        HttpBuildPublishClient::new(&endpoint, "endpoint-validation", Duration::from_secs(5))
            .map(|_| ())
            .map_err(|_| anyhow!("Veröffentlichungsendpunkt ist ungültig."))
    })
    .await
    .map_err(|_| anyhow!("Veröffentlichungsendpunkt ist nicht prüfbar."))??;
    let path = connection
        .infisical_config
        .clone()
        .unwrap_or_else(|| config::repo_root().join("config/infisical.json"));
    let pool = deadlock_brain_core::pg::pg_pool_from_config(&path, true).await?;
    dbrain_reasoner::publish::validate_publish_items(&pool, &request.payload).await?;
    pool.close().await;
    let values = tokio::time::timeout(
        Duration::from_secs(10),
        deadlock_brain_core::pg::infisical_environment(&path),
    )
    .await
    .map_err(|_| anyhow!("Infisical antwortet nicht rechtzeitig."))?
    .map_err(|_| {
        anyhow!("Infisical-Dienstzugang ist nicht verfügbar. Es wurde nichts veröffentlicht.")
    })?;
    let token = select_publish_token(&values)?;
    drop(values);
    let endpoint = connection.endpoint.clone();
    tokio::task::spawn_blocking(move || {
        let result = HttpBuildPublishClient::new(&endpoint, &token, Duration::from_secs(5))
            .and_then(|client| {
                client.publish_and_wait(
                    &request,
                    Duration::from_secs(wait_seconds),
                    Duration::from_millis(500),
                )
            });
        let mut receipt = PublicationReceipt::from_result(&request, result)?;
        receipt.request_path = Some(request_path);
        Ok(receipt)
    })
    .await
    .map_err(|_| anyhow!("Veröffentlichung bleibt unbestätigt: Dienstaufruf wurde unterbrochen."))?
}

async fn resume_publication_with<F: std::future::Future<Output = Result<PublicationReceipt>>>(
    args: &PublishResumeArgs,
    publish: impl FnOnce(SavedPublication, PathBuf) -> F,
) -> Result<PublicationReceipt> {
    let saved = SavedPublication::read(&args.request_path)?;
    publish(saved, args.request_path.clone()).await
}

async fn run_publish_resume(args: &PublishResumeArgs) -> Result<()> {
    let publication = resume_publication_with(args, |saved, request_path| {
        publish_saved_http(
            None,
            saved,
            request_path,
            &args.publish_connection,
            args.wait_seconds,
        )
    })
    .await?;
    if args.json {
        print_json(&json!({"status": publication.status, "publication": publication}))?;
    } else {
        print_publication(&publication);
    }
    publication.ensure_success()
}

async fn run_requested_build(pool: &PgPool, args: ReviewBuildArgs, review: bool) -> Result<()> {
    if brain_contracts::game_mode::GameMode::from_question(&args.query)
        == brain_contracts::game_mode::GameMode::StreetBrawl
    {
        anyhow::bail!("Street-Brawl-Kaufpläne können derzeit nicht veröffentlicht werden. Es wird kein Standardmodus-Build als Ersatz erstellt.");
    }
    let ask = dbrain_retrieval::ask_context(
        pool,
        &args.query,
        &dbrain_retrieval::AskContextOptions {
            limit_events: 80,
            include_unverified: false,
            max_claims: 12,
            game_wiki_dir: None,
        },
    )
    .await?;
    if ask.pointer("/retrieval_meta/route").and_then(Value::as_str) != Some("build_reasoner") {
        return Err(anyhow!(
            "Die Anfrage wurde nicht als Build-Anfrage erkannt."
        ));
    }
    let mut build: dbrain_reasoner::BuildObject = serde_json::from_value(
        ask.get("build_context")
            .cloned()
            .ok_or_else(|| anyhow!("Der berechnete Build-Kontext fehlt."))?,
    )?;
    let alternative_variants = build.variants.len();
    if review {
        build.variants.clear();
        build.name = format!("{} Brain Review {}", build.hero_name, build.patch_tag);
    } else if dbrain_reasoner::publish::validate_publish_input(&build).is_err() {
        return blocked_requested_build(&build.hero_name, alternative_variants, print_json);
    }
    let selected_family = build.family.as_ref().map(|family| family.label.clone());
    let publication = publish_build_http(
        pool,
        &build,
        review,
        &args.publish_connection,
        args.wait_seconds,
    )
    .await?;
    print_json(&json!({
        "status": publication.status,
        "publication": publication,
        "review": review,
        "task_id": null,
        "hero_build_id": publication.hero_build_id,
        "version": null,
        "hero_id": build.hero_id,
        "hero_name": build.hero_name,
        "build_name": build.name,
        "selected_family": selected_family,
        "alternative_variants": alternative_variants,
        "core": build.core.iter().map(|item| json!({"item_id": item.item_id, "name": item.name})).collect::<Vec<_>>(),
        "situations": build.situations.iter().map(|block| json!({
            "label": block.label,
            "items": block.items.iter().map(|item| json!({"item_id": item.item_id, "name": item.name})).collect::<Vec<_>>()
        })).collect::<Vec<_>>()
    }))?;
    publication.ensure_success()
}

fn blocked_requested_build(
    hero_name: &str,
    alternative_variants: usize,
    emit: impl FnOnce(&Value) -> Result<()>,
) -> Result<()> {
    emit(&json!({
        "status":"BLOCKED", "task_id":null, "hero_build_id":null, "version":null,
        "hero_name":hero_name, "review":false, "core":[], "situations":[],
        "message": if alternative_variants > 0 {
            "Mehrere Buildvarianten passen. Bitte nenne den gewünschten Spielstil. Es wurde nichts veröffentlicht."
        } else {
            "Die aktuellen Daten reichen nicht für einen geprüften Build. Es wurde nichts veröffentlicht."
        }
    }))?;
    Err(anyhow!(
        "Veröffentlichung ist gesperrt (BLOCKED). Es wurde nichts veröffentlicht."
    ))
}

async fn run_reason(pool: &PgPool, settings: &Settings, target: ReasonCommands) -> Result<()> {
    match target {
        ReasonCommands::Build(args) => {
            let mut config = dbrain_reasoner::ReasonerConfig {
                use_ai: !args.no_ai,
                ..Default::default()
            };
            if let Some(patch) = args.patch {
                config.patch_tag = patch;
            }
            let ai = config
                .use_ai
                .then(|| AiClient::from_settings(settings))
                .transpose()?;
            let ctx = dbrain_reasoner::ReasonerCtx {
                pool: pool.clone(),
                ai,
                config,
            };
            let seed_path = args.seed_path.unwrap_or_else(|| {
                config::path_env(
                    "DEADLOCK_REASONER_SEED_PATH",
                    settings
                        .project_root
                        .join(".tasks/2026-09-12-build-reasoner/referenz"),
                )
            });
            let options = dbrain_reasoner::ReasonerOptions {
                seed_path: Some(&seed_path),
                persist: !args.no_persist,
            };
            let build = if args.publish {
                dbrain_reasoner::reason_build_for_publish_with_options(&ctx, &args.hero, options)
                    .await?
            } else {
                dbrain_reasoner::reason_build_with_options(&ctx, &args.hero, options).await?
            };
            let publication = if args.publish {
                Some(
                    publish_build_http(
                        pool,
                        &build,
                        false,
                        &args.publish_connection,
                        args.wait_seconds,
                    )
                    .await?,
                )
            } else {
                None
            };
            if args.json {
                if let Some(publication) = &publication {
                    print_json(
                        &json!({"build": build, "hero_build_id": publication.hero_build_id, "publication": publication}),
                    )?;
                } else {
                    print_json(&build)?;
                }
            } else {
                print_reason_build(&build, publication.as_ref());
            }
            publication
                .as_ref()
                .map_or(Ok(()), PublicationReceipt::ensure_success)
        }
        ReasonCommands::ResumePublish(args) => run_publish_resume(&args).await,
        ReasonCommands::PatchImpact(args) => {
            let mut config = dbrain_reasoner::ReasonerConfig {
                use_ai: true,
                ..Default::default()
            };
            if let Some(patch) = args.patch {
                config.patch_tag = patch;
            }
            let ctx = dbrain_reasoner::ReasonerCtx {
                pool: pool.clone(),
                ai: Some(AiClient::from_settings(settings)?),
                config,
            };
            let report = dbrain_reasoner::reason_patch_impact_with_options(
                &ctx,
                &args.hero,
                dbrain_reasoner::ReasonerOptions {
                    persist: !args.no_persist,
                    ..Default::default()
                },
            )
            .await?;
            if args.json {
                print_json(&report)
            } else {
                print_reason_patch_impact(&report);
                Ok(())
            }
        }
        ReasonCommands::Backtest(args) => {
            let mut config = dbrain_reasoner::ReasonerConfig {
                use_ai: false,
                ..Default::default()
            };
            if let Some(patch) = args.patch.clone() {
                config.patch_tag = patch;
            }
            let ctx = dbrain_reasoner::ReasonerCtx {
                pool: pool.clone(),
                ai: None,
                config,
            };
            let seed_path = args.seed_path.unwrap_or_else(|| {
                config::path_env(
                    "DEADLOCK_REASONER_SEED_PATH",
                    settings
                        .project_root
                        .join(".tasks/2026-09-12-build-reasoner/referenz"),
                )
            });
            let report = dbrain_reasoner::reason_backtest_with_options(
                &ctx,
                dbrain_reasoner::BacktestFilter {
                    hero: args.hero,
                    patch_tag: args.patch,
                },
                dbrain_reasoner::ReasonerOptions {
                    seed_path: Some(&seed_path),
                    persist: !args.no_persist,
                },
            )
            .await?;
            if args.json {
                print_json(&report)
            } else {
                println!("{report}");
                Ok(())
            }
        }
    }
}

fn print_reason_build(
    build: &dbrain_reasoner::BuildObject,
    publication: Option<&PublicationReceipt>,
) {
    println!("{} ({})", build.name, build.patch_tag);
    println!("Kern:");
    for item in &build.core {
        println!("- {} [{:?}]", item.name, item.confidence);
    }
    for block in &build.situations {
        println!("{}:", block.label);
        for item in &block.items {
            println!("- {} [{:?}]", item.name, item.confidence);
        }
    }
    if !build.ability_order.is_empty() {
        println!("Skill-Order: {} Schritte", build.ability_order.len());
    }
    if !build.rationale.trim().is_empty() {
        println!("Warum: {}", build.rationale);
    }
    if let Some(publication) = publication {
        print_publication(publication);
    }
}

fn print_publication(publication: &PublicationReceipt) {
    if let Some(id) = publication.hero_build_id {
        println!("Veröffentlicht. Build-ID: {id}");
    } else {
        println!(
            "Veröffentlichung nicht bestätigt. Fehlerklasse: {}",
            publication.error_class.as_deref().unwrap_or("internal")
        );
    }
    println!("Anfrage: {}", publication.request_id);
    println!("Prüfsumme: {}", publication.request_sha256);
    if let Some(path) = &publication.request_path {
        println!("Gespeicherte Veröffentlichungsanfrage: {}", path.display());
        println!("Wiederaufnahme: deadlock-brain reason resume-publish <DATEI>");
    }
}

fn print_reason_patch_impact(report: &dbrain_reasoner::PatchImpactReport) {
    println!("{}: {}", report.hero_name, report.summary);
    for (item_id, shift) in &report.shifted_items {
        println!("- Item {item_id}: {shift:+.4}");
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
        AnalysisCommands::RunAi(args) => {
            let config = ai_config(
                settings,
                args.model,
                args.max_completion_tokens,
                args.temperature,
                args.top_p,
            );
            let result = dbrain_retrieval::analysis_run_ai(
                pool,
                &args.query,
                dbrain_retrieval::AnalysisRunAiOptions {
                    limit_events: usize_to_i64(args.limit_events),
                    config,
                    dry_run: args.dry_run,
                },
            )
            .await?;
            if args.pretty {
                print_analysis_result("run-fireworks", &result);
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

async fn run_forum_pull(args: PullForumArgs) -> Result<()> {
    if !args.data_dir.is_absolute() || !args.archive_infisical_config.is_absolute() {
        return Err(anyhow!(
            "Forumdaten und Infisical-Konfiguration benötigen absolute Pfade."
        ));
    }
    let pool =
        deadlock_brain_core::pg::pg_pool_from_config(&args.archive_infisical_config, false).await?;
    let http = http_client_async(
        config::DEFAULT_USER_AGENT.to_string(),
        args.data_dir.join("cache"),
    )
    .await?;
    let result = dbrain_sources::forum::pull_forum_with_pool(
        &pool,
        &args.data_dir.join("raw"),
        &http,
        dbrain_sources::PullForumOptions {
            sitemap_url: args.sitemap_url,
            limit: args.limit,
            delay_seconds: args.delay_seconds,
            cache_ttl_seconds: args.cache_ttl_seconds,
            refresh_existing: args.refresh_existing,
            browser_config: args.browser.then(Default::default),
        },
    )
    .await?;
    let canonical = match (args.base_release, args.publish_release) {
        (Some(base), Some(release)) => {
            let path = args
                .kernel_config
                .as_ref()
                .ok_or_else(|| anyhow!("--kernel-config ist für Veröffentlichung erforderlich."))?;
            let kernel_pool = forum_kernel_pool(path, &args.kernel_infisical_config).await?;
            Some(
                dbrain_sources::forum_corpus::publish_archive(&pool, &kernel_pool, &base, &release)
                    .await?,
            )
        }
        _ => None,
    };
    print_json(&json!({"archive": result, "canonical": canonical}))
}

async fn run_pull(pool: &PgPool, settings: &Settings, source: PullCommands) -> Result<()> {
    let http = http_client_async(settings.user_agent.clone(), settings.cache_dir.clone()).await?;
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
        PullCommands::BuildData(args) => {
            let result = dbrain_builds::sync_build_data(
                pool,
                dbrain_builds::BuildDataSyncOptions::new(args.hero, settings.user_agent.clone()),
            )
            .await?;
            print_json(&result)
        }
        PullCommands::Patchnotes(_) => {
            // Patchnotes werden direkt aus der zentralen Postgres
            // (`patchnotes`-Schema) gelesen.
            let result = dbrain_sources::pull_patchnotes(
                &settings.raw_dir,
                dbrain_sources::PullPatchnotesOptions,
            )
            .await?;
            print_json(&result)
        }
        PullCommands::Forum(args) => run_forum_pull(args).await,
        PullCommands::Reddit(args) => {
            let result = dbrain_sources::pull_reddit(
                &settings.raw_dir,
                &http,
                dbrain_sources::PullRedditOptions {
                    subreddits: args.subreddit,
                    limit: args.limit,
                    delay_seconds: args.delay_seconds,
                    cache_ttl_seconds: args.cache_ttl_seconds,
                    refresh_existing: args.refresh_existing,
                },
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
    let http = http_client_async(settings.user_agent.clone(), settings.cache_dir.clone()).await?;
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
    let entities = dbrain_normalize::normalize_entities(pool, true).await?;
    let stats = dbrain_normalize::normalize_sheet_stats(pool, true).await?;
    let tabs = dbrain_normalize::normalize_sheet_tabs(pool, true).await?;
    print_json(&json!({
        "pull": pull,
        "normalize_entities": entities,
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
        ParseCommands::Patchnotes(args) if args.feed => {
            print_json(&dbrain_normalize::sync_patchnote_posts(pool, args.dry_run).await?)
        }
        ParseCommands::Patchnotes(args) => {
            print_json(&dbrain_normalize::parse_patchnotes(pool, args.rebuild, args.dry_run).await?)
        }
        ParseCommands::ForumClaims(args) => {
            print_json(&dbrain_normalize::parse_forum_claims(pool, args.rebuild).await?)
        }
        ParseCommands::RedditClaims(args) => {
            print_json(&dbrain_normalize::parse_reddit_claims(pool, args.rebuild).await?)
        }
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

fn prepare_dirs(settings: &Settings) -> Result<()> {
    for dir in [&settings.data_dir, &settings.raw_dir, &settings.cache_dir] {
        fs::create_dir_all(dir)?;
    }
    Ok(())
}

fn http_client(settings: &Settings) -> Result<HttpClient> {
    HttpClient::new(settings.user_agent.clone(), settings.cache_dir.clone()).map_err(Into::into)
}

async fn http_client_async(user_agent: String, cache_dir: PathBuf) -> Result<HttpClient> {
    Ok(tokio::task::spawn_blocking(move || HttpClient::new(user_agent, cache_dir)).await??)
}

fn ai_config(
    settings: &Settings,
    model: Option<String>,
    max_completion_tokens: Option<u64>,
    temperature: Option<f64>,
    top_p: Option<f64>,
) -> AiConfig {
    let mut config = AiConfig::from_settings(settings);
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

fn finish_batch_result(result: &Value, emit: impl FnOnce(&Value) -> Result<()>) -> Result<()> {
    emit(result)?;
    let failed = get(result, "failed").and_then(Value::as_u64).unwrap_or(0);
    let failed_rows = get(result, "results")
        .and_then(Value::as_array)
        .map(|rows| {
            rows.iter()
                .filter(|row| {
                    matches!(
                        str_value(get(row, "status")),
                        Some("error" | "analysis_failed")
                    )
                })
                .count()
        })
        .unwrap_or(0);
    if failed > 0 || failed_rows > 0 {
        anyhow::bail!(
            "Batch fehlgeschlagen: {} Schritte fehlgeschlagen. Teilergebnisse bleiben erhalten.",
            failed.max(failed_rows as u64)
        );
    }
    Ok(())
}

fn print_json<T: Serialize + ?Sized>(value: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

async fn print_status(pool: &PgPool, settings: &Settings) -> Result<()> {
    let status = dbrain_retrieval::status(pool).await?;
    println!("Project: {}", settings.project_root.display());
    println!("Raw:     {}", settings.raw_dir.display());
    println!("Cache:   {}", settings.cache_dir.display());
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
    println!(
        "Sheet stats: {}",
        if bool_value(get(sheet, "available")) {
            "yes"
        } else {
            "no"
        }
    );
    let enrichments = get(ctx, "enrichments").unwrap_or(&Value::Null);
    println!(
        "Enrichments: {} / {} rows",
        if bool_value(get(enrichments, "available")) {
            "yes"
        } else {
            "no"
        },
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
        println!(
            "Timeline: {} (fallback)",
            display_value(get(timeline, "query"))
        );
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
    println!(
        "Matched: {}",
        if bool_value(get(summary, "matched")) {
            "yes"
        } else {
            "no"
        }
    );
    let timeline = get(review_context, "timeline_signals").unwrap_or(&Value::Null);
    println!("Events: {}", display_or(get(timeline, "event_count"), "0"));
    let latest = get(timeline, "latest_patch").unwrap_or(&Value::Null);
    if latest
        .as_object()
        .map(|object| !object.is_empty())
        .unwrap_or(false)
    {
        println!(
            "Latest patch: {} / {}",
            str_value(get(latest, "posted_at")).unwrap_or("unknown-date"),
            display_value(get(latest, "patch_title"))
        );
    }
    let stats = get(review_context, "current_stat_hints").unwrap_or(&Value::Null);
    println!(
        "Sheet stats: {} / {} hints",
        if bool_value(get(stats, "available")) {
            "yes"
        } else {
            "no"
        },
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
        let owner_text = owner
            .map(|value| format!(" / owner={value}"))
            .unwrap_or_default();
        let target_text = target
            .map(|value| format!(" -> {value}"))
            .unwrap_or_default();
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
        if bool_value(get(ctx, "is_active")) {
            "yes"
        } else {
            "no"
        },
        display_value(get(ctx, "activation"))
    );
    let archetypes = join_array(get(ctx, "archetypes"), usize::MAX);
    if !archetypes.is_empty() {
        println!("Archetypes: {archetypes}");
    }
    if let Some(description) = str_value(get(ctx, "description")).filter(|value| !value.is_empty())
    {
        println!("Desc: {description}");
    }
    if let Some(properties) = get(ctx, "properties").and_then(Value::as_array) {
        if !properties.is_empty() {
            println!("Properties:");
            for prop in properties.iter().take(20) {
                println!(
                    "- {}: {}",
                    display_value(get(prop, "label")),
                    format_prop_value(prop)
                );
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
            println!(
                "Context hash: {}",
                display_value(get(result, "context_hash"))
            );
        }
        "run-fireworks" | "run-minimax" | "run-ai" => {
            if bool_value(get(result, "dry_run")) {
                let messages = get(result, "request")
                    .and_then(|request| get(request, "messages"))
                    .and_then(Value::as_array)
                    .map(Vec::len)
                    .unwrap_or(0);
                println!("Fireworks dry-run: {}", display_value(get(result, "model")));
                println!("Endpoint: {}", display_value(get(result, "endpoint")));
                println!(
                    "API key present: {}",
                    if bool_value(get(result, "api_key_present")) {
                        "yes"
                    } else {
                        "no"
                    }
                );
                println!("Messages: {messages}");
                return;
            }
            let note = get(result, "note").unwrap_or(&Value::Null);
            println!(
                "Fireworks analysis: note={} model={}",
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
    let mut postfix = str_value(get(prop, "postfix"))
        .unwrap_or_default()
        .to_string();
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
    let sign = if value.trim_start().starts_with('-') {
        "-"
    } else {
        "+"
    };
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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn batch_failure_keeps_the_complete_output_before_returning_an_error() {
        for result in [
            json!({"processed": 3, "success": 2, "failed": 1}),
            json!({"processed": 2, "success": 0, "failed": 2}),
            json!({"results": [
                {"status": "analysis_ready", "note_id": 42},
                {"status": "error", "error": "fixture"}
            ]}),
            json!({"results": [{"status": "analysis_failed"}]}),
        ] {
            let mut output = None;
            let outcome = finish_batch_result(&result, |value| {
                output = Some(value.clone());
                Ok(())
            });
            assert!(outcome.is_err());
            assert_eq!(output.as_ref(), Some(&result));
        }
    }

    #[test]
    fn successful_empty_and_dry_run_batches_remain_successful() {
        for result in [
            json!({"processed": 3, "success": 3, "failed": 0}),
            json!({"processed": 0, "success": 0, "failed": 0, "message": "no targets pending"}),
            json!({"results": [{"status": "analysis_ready"}]}),
            json!({"dry_run": true, "results": [{"status": "context_ready"}]}),
            json!({"dry_run": true, "request": {}}),
            json!({"results": []}),
        ] {
            let mut output = None;
            finish_batch_result(&result, |value| {
                output = Some(value.clone());
                Ok(())
            })
            .unwrap();
            assert_eq!(output.as_ref(), Some(&result));
        }
    }

    #[test]
    fn batch_output_errors_are_propagated() {
        assert!(finish_batch_result(&json!({"failed": 0}), |_| {
            Err(io::Error::from(ErrorKind::BrokenPipe).into())
        })
        .is_err());
    }

    #[test]
    fn wissensabfragen_erzwingen_read_only() {
        for argv in [
            vec!["deadlock-brain", "ask-context", "Warden"],
            vec!["deadlock-brain", "context", "Warden"],
            vec!["deadlock-brain", "timeline", "Warden"],
            vec!["deadlock-brain", "review", "Warden"],
            vec!["deadlock-brain", "lineage"],
            vec!["deadlock-brain", "legacy"],
            vec!["deadlock-brain", "entities", "--query", "Warden"],
            vec![
                "deadlock-brain",
                "reason",
                "build",
                "Warden",
                "--no-persist",
            ],
            vec![
                "deadlock-brain",
                "reason",
                "patch-impact",
                "Warden",
                "--no-persist",
            ],
            vec![
                "deadlock-brain",
                "reason",
                "backtest",
                "--hero",
                "Warden",
                "--no-persist",
            ],
        ] {
            let cli = Cli::try_parse_from(argv).expect("read-only command must parse");
            assert!(command_is_read_only(&cli.command));
        }
    }

    #[test]
    fn schreibende_befehle_bleiben_schreibfaehig() {
        for argv in [
            vec!["deadlock-brain", "reason", "build", "Warden"],
            vec!["deadlock-brain", "reason", "patch-impact", "Warden"],
            vec!["deadlock-brain", "reason", "backtest", "--hero", "Warden"],
        ] {
            let cli = Cli::try_parse_from(argv).expect("write-capable command must parse");
            assert!(!command_is_read_only(&cli.command));
        }
    }

    fn assert_no_persist_cli(command: &str, hero: &[&str]) {
        let mut argv = vec!["deadlock-brain", "reason", command];
        argv.extend_from_slice(hero);
        for no_persist in [false, true] {
            if no_persist {
                argv.push("--no-persist");
            }
            let cli = Cli::try_parse_from(&argv).unwrap();
            let Commands::Reason { target } = cli.command else {
                panic!("Reason-Befehl erwartet");
            };
            let actual = match target {
                ReasonCommands::Build(args) => args.no_persist,
                ReasonCommands::ResumePublish(_) => {
                    panic!("Unerwartete Veröffentlichungswiederaufnahme")
                }
                ReasonCommands::PatchImpact(args) => args.no_persist,
                ReasonCommands::Backtest(args) => args.no_persist,
            };
            assert_eq!(actual, no_persist);
        }
        let help =
            Cli::try_parse_from(["deadlock-brain", "reason", command, "--help"]).unwrap_err();
        assert_eq!(help.kind(), clap::error::ErrorKind::DisplayHelp);
        assert!(help.to_string().contains("--no-persist"));
    }

    #[test]
    fn fix2_build_no_persist_cli() {
        assert_no_persist_cli("build", &["Warden"]);
    }

    #[test]
    fn fix2_patch_impact_no_persist_cli() {
        assert_no_persist_cli("patch-impact", &["Warden"]);
    }

    #[test]
    fn fix2_backtest_no_persist_cli() {
        assert_no_persist_cli("backtest", &["--hero", "Warden"]);
    }

    #[test]
    fn fix2_no_persist_conflicts_with_publish() {
        let error = Cli::try_parse_from([
            "deadlock-brain",
            "reason",
            "build",
            "Warden",
            "--no-persist",
            "--publish",
        ])
        .unwrap_err();
        assert_eq!(error.kind(), clap::error::ErrorKind::ArgumentConflict);
    }
    #[test]
    fn analysis_run_ai_accepts_public_command_names() {
        for command_name in ["run-fireworks", "run-minimax", "run-ai"] {
            let cli =
                Cli::try_parse_from(["deadlock-brain", "analysis", command_name, "Lady Geist"])
                    .expect("parse analysis run command");

            let Commands::Analysis {
                target: AnalysisCommands::RunAi(args),
            } = cli.command
            else {
                panic!("expected analysis run ai for {command_name}");
            };
            assert_eq!(args.query, "Lady Geist");
        }
    }

    #[test]
    fn cli_json_preserves_current_wire_statuses() {
        for (status, expected) in [
            (brain_client::AnswerStatus::BuildRejected, "build_rejected"),
            (brain_client::AnswerStatus::Unavailable, "unavailable"),
        ] {
            let response = brain_client::PublicAnswerResponse {
                contract_version: brain_client::PUBLIC_API_VERSION.into(),
                request_id: "request-1".into(),
                knowledge_release: "release-1".into(),
                status,
                text: "fixture".into(),
                citations: vec![],
                artifacts: vec![],
            };
            let value = serde_json::to_value(response).expect("public response must serialize");
            assert_eq!(value["status"], expected);
        }
    }

    #[test]
    fn clap_command_definition_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn parses_pull_reddit_with_repeated_subreddits_and_gentle_defaults() {
        let cli = Cli::try_parse_from([
            "deadlock-brain",
            "pull",
            "reddit",
            "--subreddit",
            "DeadlockTheGame",
            "--subreddit",
            "DeadlockMemes",
            "--limit",
            "5",
        ])
        .expect("parse cli");

        let Commands::Pull {
            source: PullCommands::Reddit(args),
        } = cli.command
        else {
            panic!("expected pull reddit");
        };
        assert_eq!(
            args.subreddit,
            vec!["DeadlockTheGame".to_string(), "DeadlockMemes".to_string()]
        );
        assert_eq!(args.limit, 5);
        assert_eq!(args.delay_seconds, 2.0);
        assert_eq!(args.cache_ttl_seconds, 86_400);
        assert!(!args.refresh_existing);
    }

    #[test]
    fn parses_parse_reddit_claims_rebuild_flag() {
        let cli = Cli::try_parse_from(["deadlock-brain", "parse", "reddit-claims", "--rebuild"])
            .expect("parse cli");

        let Commands::Parse {
            target: ParseCommands::RedditClaims(args),
        } = cli.command
        else {
            panic!("expected parse reddit-claims");
        };
        assert!(args.rebuild);
    }

    #[test]
    fn blocking_http_client_is_built_off_the_async_runtime_thread() {
        use std::{io::Write, net::TcpListener, thread};

        let cache_dir =
            std::env::temp_dir().join(format!("deadlock-brain-http-client-test-{}", process::id()));
        let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
        let url = format!("http://{}/health", listener.local_addr().expect("address"));
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept");
            let mut request = [0_u8; 1024];
            let bytes_read = stream.read(&mut request).expect("read request");
            assert!(bytes_read > 0);
            stream
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nok")
                .expect("write response");
        });
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("runtime");

        runtime.block_on(async {
            let client = http_client_async("deadlock-brain-test".to_string(), cache_dir.clone())
                .await
                .expect("blocking http client");
            let response = client
                .get(&url, deadlock_brain_core::http::HttpGetOptions::default())
                .expect("GET inside async command");
            assert_eq!(response.text(), "ok");
            drop(client);
        });
        server.join().expect("server thread");
        fs::remove_dir_all(cache_dir).expect("remove cache dir");
    }

    #[test]
    fn pull_assets_rejects_unsupported_raw_kinds() {
        for kind in ["raw_items", "raw_heroes"] {
            let error = Cli::try_parse_from(["deadlock-brain", "pull", "assets", "--kind", kind])
                .unwrap_err();
            assert_eq!(error.kind(), clap::error::ErrorKind::InvalidValue);
        }
    }

    #[test]
    fn pull_assets_accepts_supported_kinds_and_default() {
        for kind in ["items", "heroes", "ranks"] {
            Cli::try_parse_from(["deadlock-brain", "pull", "assets", "--kind", kind])
                .expect("supported pull assets kind should parse");
        }
        Cli::try_parse_from(["deadlock-brain", "pull", "assets"])
            .expect("pull assets without --kind should parse");
    }

    #[test]
    fn pull_assets_help_lists_supported_kinds_only() {
        let help = Cli::try_parse_from(["deadlock-brain", "pull", "assets", "--help"]).unwrap_err();
        assert_eq!(help.kind(), clap::error::ErrorKind::DisplayHelp);
        let text = help.to_string();
        assert!(text.contains("items"));
        assert!(text.contains("heroes"));
        assert!(!text.contains("raw_items"));
        assert!(!text.contains("raw_heroes"));
    }
}

async fn forum_kernel_pool(
    path: &std::path::Path,
    infisical_path: &std::path::Path,
) -> Result<PgPool> {
    #[derive(serde::Deserialize)]
    struct Runtime {
        postgres: Database,
    }
    #[derive(serde::Deserialize)]
    struct Database {
        socket_dir: PathBuf,
        port: u16,
        username: String,
        database: String,
        auth: String,
        password_env: Option<String>,
        max_connections: u32,
    }
    let runtime: Runtime = serde_json::from_slice(
        &fs::read(path).map_err(|_| anyhow!("Kernelkonfiguration ist nicht lesbar."))?,
    )
    .map_err(|_| anyhow!("Kernelkonfiguration ist ungültig."))?;
    let database = runtime.postgres;
    if !database.socket_dir.is_absolute()
        || !database.socket_dir.is_dir()
        || database.port == 0
        || database.max_connections < 2
    {
        return Err(anyhow!("Ungültige Kernel-Datenbankkonfiguration."));
    }
    let mut options = sqlx::postgres::PgConnectOptions::new()
        .host(
            database
                .socket_dir
                .to_str()
                .ok_or_else(|| anyhow!("Ungültiger Socketpfad."))?,
        )
        .port(database.port)
        .username(&database.username)
        .database(&database.database)
        .ssl_mode(sqlx::postgres::PgSslMode::Disable);
    match database.auth.as_str() {
        "peer" => {}
        "password" => {
            let key = database
                .password_env
                .ok_or_else(|| anyhow!("Infisical-Verweis für Kernel-Datenbank fehlt."))?;
            let values = dl_token_secrets::values(infisical_path)
                .await
                .map_err(|_| anyhow!("Kernel-Infisicalzugriff fehlgeschlagen."))?;
            let secret = values
                .iter()
                .find(|(name, _)| name == &key)
                .map(|(_, value)| value.as_str())
                .ok_or_else(|| anyhow!("Kernel-Datenbankzugang fehlt in Infisical."))?;
            options = options.password(secret);
        }
        _ => return Err(anyhow!("Unbekannte Kernel-Datenbankanmeldung.")),
    }
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(database.max_connections.min(4))
        .acquire_timeout(std::time::Duration::from_secs(15))
        .connect_with(options)
        .await
        .map_err(|_| anyhow!("Kernel-Datenbank ist nicht erreichbar."))
}

#[cfg(test)]
mod forum_pool_configuration_tests {
    use super::*;
    #[tokio::test]
    async fn single_connection_pool_is_rejected_before_secret_lookup() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("runtime.json");
        fs::write(&path,serde_json::to_vec(&json!({"postgres":{"socket_dir":root.path(),"port":5446,"username":"fixture","database":"fixture","auth":"password","password_env":"unused","max_connections":1}})).unwrap()).unwrap();
        assert!(
            forum_kernel_pool(&path, &root.path().join("missing-infisical.json"))
                .await
                .unwrap_err()
                .to_string()
                .contains("Ungültige Kernel-Datenbankkonfiguration")
        );
    }
}
