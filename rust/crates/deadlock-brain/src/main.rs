#![forbid(unsafe_code)]
#![allow(dead_code)]

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "deadlock-brain")]
struct Cli {
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
    #[command(about = "Fuehrt lokale Datenqualitaetschecks aus.")]
    Quality(PrettyArgs),
    #[command(about = "Zeigt Rename-/Rework-Beziehungen aus Patchnotes.")]
    Lineage(LineageArgs),
    #[command(about = "Zeigt alte/entfernte Entities aus Patchnotes.")]
    Legacy(LegacyArgs),
    #[command(about = "Erzeugt einen erklaerbaren Hero-Build-Vorschlag aus API-/Sheet-/Patchdaten.")]
    Build(BuildArgs),
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
}

#[derive(Debug, Args)]
struct PrettyArgs {
    #[arg(long, help = "Kompakter menschenlesbarer Output statt JSON.")]
    pretty: bool,
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
enum PullCommands {
    #[command(about = "Zieht Deadlock Assets API Daten.")]
    Assets(PullAssetsArgs),
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

#[derive(Debug, Subcommand)]
enum NormalizeCommands {
    #[command(about = "Baut entities und entity_aliases.")]
    Entities(NormalizeEntitiesArgs),
    #[command(name = "sheet-stats", about = "Baut normalisierte Hero-Stats aus dem Google Sheet.")]
    SheetStats(NormalizeSheetStatsArgs),
    #[command(name = "sheet-tabs", about = "Baut normalisierte Tabellen fuer Hero-Rankings, Boons/AP und Raw-Heroes.")]
    SheetTabs(NormalizeSheetTabsArgs),
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

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    run(cli)
}

fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Status => todo!("Wave 1: status"),
        Commands::Context(_) => todo!("Wave 1: context"),
        Commands::Timeline(_) => todo!("Wave 1: timeline"),
        Commands::Review(_) => todo!("Wave 1: review"),
        Commands::Quality(_) => todo!("Wave 1: quality"),
        Commands::Lineage(_) => todo!("Wave 1: lineage"),
        Commands::Legacy(_) => todo!("Wave 1: legacy"),
        Commands::Build(_) => todo!("Wave 1: build"),
        Commands::Item(_) => todo!("Wave 1: item"),
        Commands::Learn { target } => match target {
            LearnCommands::ImportSteamBuilds(_) => todo!("Wave 1: learn import-steam-builds"),
            LearnCommands::ListBuilds(_) => todo!("Wave 1: learn list-builds"),
            LearnCommands::AnalyzeBuild(_) => todo!("Wave 1: learn analyze-build"),
            LearnCommands::AnalyzeNext(_) => todo!("Wave 1: learn analyze-next"),
        },
        Commands::Player { target } => match target {
            PlayerCommands::ListMatches(_) => todo!("Wave 1: player list-matches"),
            PlayerCommands::MatchContext(_) => todo!("Wave 1: player match-context"),
            PlayerCommands::AnalyzeMatch(_) => todo!("Wave 1: player analyze-match"),
            PlayerCommands::AnalyzeNext(_) => todo!("Wave 1: player analyze-next"),
        },
        Commands::Youtube { target } => match target {
            YoutubeCommands::Discover(_) => todo!("Wave 1: youtube discover"),
            YoutubeCommands::ImportTranscripts(_) => todo!("Wave 1: youtube import-transcripts"),
            YoutubeCommands::FetchTranscripts(_) => todo!("Wave 1: youtube fetch-transcripts"),
            YoutubeCommands::TranscribeLocal(_) => todo!("Wave 1: youtube transcribe-local"),
            YoutubeCommands::AnalyzeNext(_) => todo!("Wave 1: youtube analyze-next"),
            YoutubeCommands::AutoLearn(_) => todo!("Wave 1: youtube auto-learn"),
            YoutubeCommands::Queue(_) => todo!("Wave 1: youtube queue"),
        },
        Commands::Pull { source } => match source {
            PullCommands::Assets(_) => todo!("Wave 1: pull assets"),
            PullCommands::Patchnotes(_) => todo!("Wave 1: pull patchnotes"),
            PullCommands::Statlocker(_) => todo!("Wave 1: pull statlocker"),
        },
        Commands::RefreshSheet => todo!("Wave 1: refresh-sheet"),
        Commands::Normalize { target } => match target {
            NormalizeCommands::Entities(_) => todo!("Wave 1: normalize entities"),
            NormalizeCommands::SheetStats(_) => todo!("Wave 1: normalize sheet-stats"),
            NormalizeCommands::SheetTabs(_) => todo!("Wave 1: normalize sheet-tabs"),
        },
        Commands::Parse { target } => match target {
            ParseCommands::Patchnotes(_) => todo!("Wave 1: parse patchnotes"),
        },
        Commands::Enrich { target } => match target {
            EnrichCommands::PatchEvents(_) => todo!("Wave 1: enrich patch-events"),
            EnrichCommands::Lineage(_) => todo!("Wave 1: enrich lineage"),
            EnrichCommands::LegacyEntities(_) => todo!("Wave 1: enrich legacy-entities"),
            EnrichCommands::PatchImpact(_) => todo!("Wave 1: enrich patch-impact"),
            EnrichCommands::MetaTrends => todo!("Wave 1: enrich meta-trends"),
        },
    }
}
