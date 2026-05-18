from __future__ import annotations

import argparse
import json
import sys
import time
from pathlib import Path
from typing import Any

from deadlock_brain.analysis_notes import list_analysis_notes, save_review_analysis_note
from deadlock_brain.build_learning import (
    build_learning_context,
    build_minimax_build_learning_request,
    import_steam_builds,
    list_learned_builds,
    list_pending_build_learning_targets,
    save_build_learning_note,
)
from deadlock_brain.build_optimizer import build_hero_build_context, build_item_context
from deadlock_brain.config import load_settings
from deadlock_brain.entity_normalizer import normalize_entities
from deadlock_brain.http import HttpClient
from deadlock_brain.legacy_entities import build_legacy_entities
from deadlock_brain.lineage import build_entity_lineage, related_names_for_query
from deadlock_brain.minimax_client import (
    MiniMaxConfig,
    build_minimax_review_request,
    call_minimax_chat,
    extract_minimax_text,
    minimax_usage_summary,
)
from deadlock_brain.patch_event_enricher import build_patch_event_enrichments
from deadlock_brain.patch_parser import parse_all_patchnotes
from deadlock_brain.player_decision_learning import (
    build_minimax_player_match_decision_request,
    build_player_match_decision_context,
    list_pending_player_match_decision_targets,
    list_player_matches,
    save_player_match_decision_note,
)
from deadlock_brain.quality import run_quality_checks
from deadlock_brain.retrieval import build_entity_context
from deadlock_brain.review_context import build_review_context
from deadlock_brain.sheet_normalizer import normalize_sheet_stats
from deadlock_brain.sheet_tabs import normalize_sheet_tabs
from deadlock_brain.sources.assets_api import ENDPOINTS, pull_assets
from deadlock_brain.sources.deadlock_api import pull_match_metadata
from deadlock_brain.sources.google_sheet import pull_sheet
from deadlock_brain.sources.patchnotes_db import pull_patchnotes
from deadlock_brain.sources.statlocker import pull_statlocker
from deadlock_brain.sources.wiki import pull_wiki_page
from deadlock_brain.storage import BrainStore
from deadlock_brain.timeline import build_entity_timeline


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog="deadlock-brain")
    sub = parser.add_subparsers(dest="command", required=True)

    sub.add_parser("status", help="Zeigt lokale DB- und Source-Counts.")

    context = sub.add_parser("context", help="Baut einen kompakten Datenkontext fuer eine Entity-Frage.")
    context.add_argument("query", help="Hero, Item, Ability oder Alias.")
    context.add_argument("--limit-events", type=int, default=30)
    context.add_argument("--pretty", action="store_true", help="Kompakter menschenlesbarer Output statt JSON.")

    timeline = sub.add_parser("timeline", help="Baut eine Patch-Timeline fuer eine Entity.")
    timeline.add_argument("query", help="Hero, Item, Ability oder Alias.")
    timeline.add_argument("--limit-events", type=int, default=2000)
    timeline.add_argument("--descending", action="store_true", help="Neueste Patches zuerst.")
    timeline.add_argument("--pretty", action="store_true", help="Kompakter menschenlesbarer Output statt JSON.")

    review = sub.add_parser("review", help="Baut einen KI-tauglichen Review-Kontext ohne Modellaufruf.")
    review.add_argument("query", help="Hero, Item, Ability oder Alias.")
    review.add_argument("--limit-events", type=int, default=80)
    review.add_argument("--pretty", action="store_true", help="Kompakter menschenlesbarer Output statt JSON.")
    review.add_argument("--prompt-only", action="store_true", help="Nur den deutschen Prompt-Entwurf ausgeben.")

    quality = sub.add_parser("quality", help="Fuehrt lokale Datenqualitaetschecks aus.")
    quality.add_argument("--pretty", action="store_true", help="Kompakter menschenlesbarer Output statt JSON.")

    lineage = sub.add_parser("lineage", help="Zeigt Rename-/Rework-Beziehungen aus Patchnotes.")
    lineage.add_argument("query", nargs="?", help="Optionaler Hero, Item, Ability oder alter Name.")
    lineage.add_argument("--pretty", action="store_true", help="Kompakter menschenlesbarer Output statt JSON.")
    lineage.add_argument("--limit", type=int, default=50)

    legacy = sub.add_parser("legacy", help="Zeigt alte/entfernte Entities aus Patchnotes.")
    legacy.add_argument("query", nargs="?", help="Optionaler alter Name.")
    legacy.add_argument("--pretty", action="store_true", help="Kompakter menschenlesbarer Output statt JSON.")
    legacy.add_argument("--limit", type=int, default=50)

    build = sub.add_parser("build", help="Erzeugt einen erklaerbaren Hero-Build-Vorschlag aus API-/Sheet-/Patchdaten.")
    build.add_argument("query", help="Hero-Name.")
    build.add_argument("--limit-events", type=int, default=80)
    build.add_argument("--pretty", action="store_true", help="Kompakter menschenlesbarer Output statt JSON.")

    item = sub.add_parser("item", help="Zeigt strukturierte Item-Daten aus der Deadlock Assets API.")
    item.add_argument("query", help="Item-Name.")
    item.add_argument("--pretty", action="store_true", help="Kompakter menschenlesbarer Output statt JSON.")

    learn = sub.add_parser("learn", help="Importiert und analysiert Build-Trainingsdaten.")
    learn_sub = learn.add_subparsers(dest="target", required=True)
    learn_import_steam = learn_sub.add_parser("import-steam-builds", help="Importiert GC/Steam Hero-Builds aus der Steam-Bot-DB.")
    learn_import_steam.add_argument("--db-path", help="Pfad zur Steam-Bot SQLite DB mit hero_build_sources.")
    learn_import_steam.add_argument("--hero", help="Optional nur ein Hero.")
    learn_import_steam.add_argument("--language", type=int, default=0, help="Steam Build-Sprache, default Englisch=0.")
    learn_import_steam.add_argument("--limit-per-hero", type=int, default=10)
    learn_import_steam.add_argument("--pretty", action="store_true")
    learn_list = learn_sub.add_parser("list-builds", help="Listet importierte Trainings-Builds.")
    learn_list.add_argument("--hero")
    learn_list.add_argument("--limit", type=int, default=25)
    learn_list.add_argument("--pretty", action="store_true")
    learn_analyze = learn_sub.add_parser("analyze-build", help="Laesst MiniMax einen importierten Build als Trainingsbeispiel analysieren.")
    learn_analyze.add_argument("build_id", type=int, help="ID aus learned_builds.")
    learn_analyze.add_argument("--model")
    learn_analyze.add_argument("--max-completion-tokens", type=int)
    learn_analyze.add_argument("--temperature", type=float)
    learn_analyze.add_argument("--top-p", type=float)
    learn_analyze.add_argument("--dry-run", action="store_true")
    learn_analyze.add_argument("--pretty", action="store_true")
    learn_next = learn_sub.add_parser("analyze-next", help="Analysiert automatisch die naechsten noch offenen Trainings-Builds.")
    learn_next.add_argument("--hero", help="Optional nur ein Hero.")
    learn_next.add_argument("--limit", type=int, default=5)
    learn_next.add_argument("--model")
    learn_next.add_argument("--max-completion-tokens", type=int)
    learn_next.add_argument("--temperature", type=float)
    learn_next.add_argument("--top-p", type=float)
    learn_next.add_argument("--dry-run", action="store_true")
    learn_next.add_argument("--delay-seconds", type=float, default=2.0)
    learn_next.add_argument("--pretty", action="store_true")

    player = sub.add_parser("player", help="Analysiert Statlocker-Player-Matches als Entscheidungs-Training.")
    player_sub = player.add_subparsers(dest="target", required=True)
    player_list = player_sub.add_parser("list-matches", help="Listet importierte Statlocker Player-Matches.")
    player_list.add_argument("--account-id")
    player_list.add_argument("--limit", type=int, default=25)
    player_list.add_argument("--pretty", action="store_true")
    player_context = player_sub.add_parser("match-context", help="Baut einen MiniMax-tauglichen Player-Match-Kontext ohne Modellaufruf.")
    player_context.add_argument("account_id")
    player_context.add_argument("match_id")
    player_context.add_argument("--pretty", action="store_true")
    player_analyze = player_sub.add_parser("analyze-match", help="Laesst MiniMax ein Player-Match als Entscheidungsbeispiel analysieren.")
    player_analyze.add_argument("account_id")
    player_analyze.add_argument("match_id")
    player_analyze.add_argument("--model")
    player_analyze.add_argument("--max-completion-tokens", type=int)
    player_analyze.add_argument("--temperature", type=float)
    player_analyze.add_argument("--top-p", type=float)
    player_analyze.add_argument("--dry-run", action="store_true")
    player_analyze.add_argument("--pretty", action="store_true")
    player_next = player_sub.add_parser("analyze-next", help="Analysiert automatisch die naechsten offenen Player-Matches.")
    player_next.add_argument("--account-id")
    player_next.add_argument("--limit", type=int, default=5)
    player_next.add_argument("--model")
    player_next.add_argument("--max-completion-tokens", type=int)
    player_next.add_argument("--temperature", type=float)
    player_next.add_argument("--top-p", type=float)
    player_next.add_argument("--dry-run", action="store_true")
    player_next.add_argument("--delay-seconds", type=float, default=2.0)
    player_next.add_argument("--pretty", action="store_true")

    analysis = sub.add_parser("analysis", help="Speichert oder listet persistente Analyse-Kontexte.")
    analysis_sub = analysis.add_subparsers(dest="target", required=True)
    analysis_save = analysis_sub.add_parser("save-review", help="Speichert einen Review-Kontext ohne Modellaufruf.")
    analysis_save.add_argument("query", help="Hero, Item, Ability oder Alias.")
    analysis_save.add_argument("--limit-events", type=int, default=80)
    analysis_save.add_argument("--result-text", help="Optionaler fertiger Analyse-Text.")
    analysis_save.add_argument("--model", help="Optionaler Modellname fuer fertige Analyse.")
    analysis_save.add_argument("--confidence", type=float)
    analysis_save.add_argument("--pretty", action="store_true")
    analysis_minimax = analysis_sub.add_parser(
        "run-minimax",
        help="Ruft MiniMax fuer einen Review-Kontext auf und speichert das Ergebnis.",
    )
    analysis_minimax.add_argument("query", help="Hero, Item, Ability oder Alias.")
    analysis_minimax.add_argument("--limit-events", type=int, default=80)
    analysis_minimax.add_argument("--model")
    analysis_minimax.add_argument("--max-completion-tokens", type=int)
    analysis_minimax.add_argument("--temperature", type=float)
    analysis_minimax.add_argument("--top-p", type=float)
    analysis_minimax.add_argument(
        "--dry-run",
        action="store_true",
        help="Baut nur den MiniMax-Request ohne API-Aufruf.",
    )
    analysis_minimax.add_argument("--pretty", action="store_true")
    analysis_list = analysis_sub.add_parser("list", help="Listet gespeicherte Analyse-Notizen.")
    analysis_list.add_argument("--query")
    analysis_list.add_argument("--limit", type=int, default=25)
    analysis_list.add_argument("--pretty", action="store_true")

    events = sub.add_parser("events", help="Zeigt gespeicherte Patch-Events.")
    events.add_argument("--entity", help="Hero/Item/Ability Name oder Teilname.")
    events.add_argument(
        "--type",
        dest="entity_type",
        choices=[
            "hero",
            "hero_internal",
            "item",
            "item_special",
            "ability",
            "ability_internal",
            "weapon_or_internal",
            "item_or_ability",
            "general",
        ],
    )
    events.add_argument("--source-kind", choices=["forum", "steam", "other"])
    events.add_argument("--limit", type=int, default=25)

    entities = sub.add_parser("entities", help="Zeigt normalisierte Entities.")
    entities.add_argument(
        "--type",
        dest="entity_type",
        choices=["hero", "hero_internal", "item", "item_special", "ability", "ability_internal", "weapon_or_internal", "rank"],
    )
    entities.add_argument("--query", help="Name, Alias, Classname oder ID als Teilstring.")
    entities.add_argument("--limit", type=int, default=25)

    ask_parser = sub.add_parser("ask", help="Stellt eine Frage ans Brain (vollständige Pipeline).")
    ask_parser.add_argument("query")
    ask_parser.add_argument("--pretty", action="store_true")
    ask_parser.add_argument("--dry-run", action="store_true")

    normalize = sub.add_parser("normalize", help="Erzeugt normalisierte Daten aus Snapshots.")
    normalize_sub = normalize.add_subparsers(dest="target", required=True)
    normalize_entities_parser = normalize_sub.add_parser("entities", help="Baut entities und entity_aliases.")
    normalize_entities_parser.add_argument("--rebuild", action="store_true", help="Loescht normalisierte Entities vorher.")
    normalize_sheet_parser = normalize_sub.add_parser("sheet-stats", help="Baut normalisierte Hero-Stats aus dem Google Sheet.")
    normalize_sheet_parser.add_argument("--rebuild", action="store_true", help="Loescht Sheet-Stat-Tabellen vorher.")
    normalize_sheet_tabs_parser = normalize_sub.add_parser("sheet-tabs", help="Baut normalisierte Tabellen fuer Hero-Rankings, Boons/AP und Raw-Heroes.")
    normalize_sheet_tabs_parser.add_argument("--rebuild", action="store_true", help="Loescht Tab-Tabellen vorher.")

    sub.add_parser("refresh-sheet", help="Zieht alle Sheet-Tabs frisch und normalisiert Hero-Stats und Tab-Tabellen.")

    parse = sub.add_parser("parse", help="Erzeugt strukturierte Daten aus importierten Quellen.")
    parse_sub = parse.add_subparsers(dest="target", required=True)
    parse_patchnotes = parse_sub.add_parser("patchnotes", help="Parst Patchnotes zu patch_events.")
    parse_patchnotes.add_argument("--rebuild", action="store_true", help="Loescht patch_events vorher neu.")

    enrich = sub.add_parser("enrich", help="Reichert strukturierte Daten an.")
    enrich_sub = enrich.add_subparsers(dest="target", required=True)
    enrich_patch_events = enrich_sub.add_parser("patch-events", help="Baut Details aus patch_events.")
    enrich_patch_events.add_argument("--rebuild", action="store_true", help="Loescht Enrichments vorher.")
    enrich_lineage = enrich_sub.add_parser("lineage", help="Baut Rename-/Rework-Lineage aus patch_events.")
    enrich_lineage.add_argument("--rebuild", action="store_true", help="Loescht Lineage vorher.")
    enrich_legacy = enrich_sub.add_parser("legacy-entities", help="Baut alte/entfernte Entities aus patch_events.")
    enrich_legacy.add_argument("--rebuild", action="store_true", help="Loescht Legacy-Entities vorher.")
    enrich_patch_impact = enrich_sub.add_parser("patch-impact", help="Analysiert die Patch-Entwicklung von Entities.")
    enrich_patch_impact.add_argument("--hero", help="Optional: Beschraenken auf diesen Hero.")
    enrich_patch_impact.add_argument("--limit", type=int, default=10, help="Anzahl der abzuarbeitenden Entities.")
    enrich_meta_trends = enrich_sub.add_parser("meta-trends", help="Analysiert aktuelle Meta-Trends.")

    pull = sub.add_parser("pull", help="Zieht Daten aus einer Quelle.")
    pull_sub = pull.add_subparsers(dest="source", required=True)

    assets = pull_sub.add_parser("assets", help="Zieht Deadlock Assets API Daten.")
    assets.add_argument(
        "--kind",
        action="append",
        choices=sorted(ENDPOINTS.keys()),
        help="Endpoint auswaehlen. Mehrfach nutzbar. Default: items/heroes/raw_items/raw_heroes.",
    )

    sheet = pull_sub.add_parser("sheet", help="Zieht das Google Sheet als CSV.")
    sheet.add_argument("--sheet-id")
    sheet.add_argument("--gid")
    sheet.add_argument("--all-tabs", action="store_true", help="Zieht alle veroeffentlichten Tabs aus pubhtml.")
    sheet.add_argument("--cache-ttl-seconds", type=int, default=1800)

    patchnotes = pull_sub.add_parser("patchnotes", help="Importiert bestehende Patchnotes aus der zentralen Bot-DB.")
    patchnotes.add_argument("--db-path")

    wiki = pull_sub.add_parser("wiki", help="Zieht gezielt eine Wiki-Seite, kein Bulk-Crawl.")
    wiki.add_argument("title")
    wiki.add_argument("--allow-wiki-network", action="store_true")

    statlocker = pull_sub.add_parser("statlocker", help="Zieht gezielt Statlocker WPA-/Leaderboard-Daten.")
    statlocker.add_argument(
        "--kind",
        action="append",
        choices=[
            "wpa-patches",
            "wpa-items",
            "leaderboard",
            "player-profile",
            "player-matches",
            "match-detail",
            "player-build-analysis",
            "leaderboard-player-matches",
        ],
        help="Mehrfach nutzbar. Default: wpa-patches, wpa-items, leaderboard.",
    )
    statlocker.add_argument("--patch", help="z.B. patch_129989 oder 129989. Default: neuester Statlocker-WPA-Patch.")
    statlocker.add_argument("--hero", default="all", help="all oder Hero-Name, z.B. Mo & Krill.")
    statlocker.add_argument("--min-sample-size", type=int, default=500)
    statlocker.add_argument("--rank", default="rank_10,rank_11")
    statlocker.add_argument("--leaderboard-page", type=int, default=1)
    statlocker.add_argument("--leaderboard-page-size", type=int, default=100)
    statlocker.add_argument("--account-id", help="Statlocker/Steam accountId fuer Player-Endpunkte.")
    statlocker.add_argument("--match-id", help="Match-ID fuer Matchdetail-Endpunkt.")
    statlocker.add_argument("--hero-id", help="Hero-ID fuer Player-Build-Analysis.")
    statlocker.add_argument("--players-from-leaderboard", type=int, default=5)
    statlocker.add_argument("--matches-per-player", type=int, default=6)
    statlocker.add_argument("--include-match-details", action="store_true")
    statlocker.add_argument("--include-build-analysis", action="store_true")
    statlocker.add_argument("--game-mode", choices=["all", "standard", "brawl"], default="all")
    statlocker.add_argument("--delay-seconds", type=float, default=1.0)
    statlocker.add_argument("--cache-ttl-seconds", type=int, default=21600)

    deadlock_api = pull_sub.add_parser("deadlock-api", help="Zieht echte Match-Metadata aus der Deadlock API.")
    deadlock_api.add_argument("--match-id", action="append", required=True, help="Match-ID. Mehrfach nutzbar.")
    deadlock_api.add_argument("--account-id", action="append", help="Optional auf Account-ID filtern. Mehrfach nutzbar.")
    deadlock_api.add_argument("--hero-id", action="append", help="Optional auf Hero-ID filtern. Mehrfach nutzbar.")
    deadlock_api.add_argument("--cache-ttl-seconds", type=int, default=21600)
    deadlock_api.add_argument("--no-player-items", action="store_true")
    deadlock_api.add_argument("--no-player-stats", action="store_true")
    deadlock_api.add_argument("--no-death-details", action="store_true")
    deadlock_api.add_argument("--no-objectives", action="store_true")

    all_sources = pull_sub.add_parser("all", help="Zieht sichere Standardquellen ohne Wiki.")
    all_sources.add_argument("--include-patchnotes", action="store_true", default=True)
    return parser


def _build_minimax_config(settings, model: str | None = None, max_tokens: int | None = None, temperature: float | None = None) -> MiniMaxConfig:
    return MiniMaxConfig(
        api_key=settings.minimax_api_key,
        base_url=settings.minimax_base_url,
        model=model or settings.minimax_model,
        timeout_seconds=settings.minimax_timeout_seconds,
        max_completion_tokens=max_tokens or settings.minimax_max_completion_tokens,
        temperature=temperature if temperature is not None else settings.minimax_temperature,
        top_p=settings.minimax_top_p,
        use_token_plan=settings.minimax_use_token_plan,
    )


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    settings = load_settings()
    settings.data_dir.mkdir(parents=True, exist_ok=True)
    settings.raw_dir.mkdir(parents=True, exist_ok=True)
    settings.cache_dir.mkdir(parents=True, exist_ok=True)

    store = BrainStore(settings.db_path, settings.raw_dir)
    http = HttpClient(user_agent=settings.user_agent, cache_dir=settings.cache_dir)
    try:
        if args.command == "status":
            _print_status(store, settings)
            return 0
        if args.command == "pull":
            summary = _run_pull(args, store, http, settings)
            print(json.dumps(summary, ensure_ascii=False, indent=2))
            return 0
        if args.command == "parse":
            summary = _run_parse(args, store)
            print(json.dumps(summary, ensure_ascii=False, indent=2))
            return 0
        if args.command == "normalize":
            summary = _run_normalize(args, store)
            print(json.dumps(summary, ensure_ascii=False, indent=2))
            return 0
        if args.command == "enrich":
            summary = _run_enrich(args, store, settings)
            print(json.dumps(summary, ensure_ascii=False, indent=2))
            return 0
        if args.command == "context":
            ctx = build_entity_context(store.conn, args.query, limit_events=args.limit_events)
            if args.pretty:
                _print_context(ctx)
            else:
                print(json.dumps(ctx, ensure_ascii=False, indent=2))
            return 0
        if args.command == "timeline":
            timeline_data = build_entity_timeline(
                store.conn,
                args.query,
                limit_events=args.limit_events,
                ascending=not args.descending,
            )
            if args.pretty:
                _print_timeline(timeline_data)
            else:
                print(json.dumps(timeline_data, ensure_ascii=False, indent=2))
            return 0
        if args.command == "review":
            review_context = build_review_context(store.conn, args.query, limit_events=args.limit_events)
            if args.prompt_only:
                print(review_context.get("prompt_de") or "")
            elif args.pretty:
                _print_review_context(review_context)
            else:
                print(json.dumps(review_context, ensure_ascii=False, indent=2))
            return 0
        if args.command == "quality":
            quality_report = run_quality_checks(store.conn)
            if args.pretty:
                _print_quality_report(quality_report)
            else:
                print(json.dumps(quality_report, ensure_ascii=False, indent=2))
            return 0
        if args.command == "lineage":
            lineage_rows = _load_lineage(store, args.query, args.limit)
            if args.pretty:
                _print_lineage(lineage_rows, args.query)
            else:
                print(json.dumps({"query": args.query, "lineage": lineage_rows}, ensure_ascii=False, indent=2))
            return 0
        if args.command == "legacy":
            legacy_rows = _load_legacy(store, args.query, args.limit)
            if args.pretty:
                _print_legacy(legacy_rows, args.query)
            else:
                print(json.dumps({"query": args.query, "legacy_entities": legacy_rows}, ensure_ascii=False, indent=2))
            return 0
        if args.command == "build":
            build_context = build_hero_build_context(store.conn, args.query, limit_events=args.limit_events)
            if args.pretty:
                _print_build_context(build_context)
            else:
                print(json.dumps(build_context, ensure_ascii=False, indent=2))
            return 0
        if args.command == "item":
            item_context = build_item_context(store.conn, args.query)
            if args.pretty:
                _print_item_context(item_context)
            else:
                print(json.dumps(item_context, ensure_ascii=False, indent=2))
            return 0
        if args.command == "analysis":
            result = _run_analysis(args, store, settings)
            if args.pretty:
                _print_analysis_result(args, result)
            else:
                print(json.dumps(result, ensure_ascii=False, indent=2))
            return 0
        if args.command == "learn":
            result = _run_learn(args, store, settings)
            if args.pretty:
                _print_learn_result(args, result)
            else:
                print(json.dumps(result, ensure_ascii=False, indent=2))
            return 0
        if args.command == "player":
            result = _run_player(args, store, settings)
            if args.pretty:
                _print_player_result(args, result)
            else:
                print(json.dumps(result, ensure_ascii=False, indent=2))
            return 0
        if args.command == "events":
            _print_events(store, args)
            return 0
        if args.command == "entities":
            _print_entities(store, args)
            return 0
        if args.command == "ask":
            from deadlock_brain.brain_pipeline import ask as brain_ask
            result = brain_ask(args.query, store.conn, _build_minimax_config(settings))
            if args.pretty:
                print(json.dumps(result, ensure_ascii=False, indent=2))
            else:
                print(json.dumps(result, ensure_ascii=False))
            return 0
        if args.command == "refresh-sheet":
            summary = _run_refresh_sheet(store, http, settings)
            print(json.dumps(summary, ensure_ascii=False, indent=2))
            return 0
    finally:
        store.close()
    return 1


def _run_parse(args: argparse.Namespace, store: BrainStore) -> dict[str, Any]:
    run_id = store.begin_run(f"parse_{args.target}")
    try:
        if args.target == "patchnotes":
            summary = parse_all_patchnotes(store, rebuild=args.rebuild)
        else:
            raise ValueError(f"Unbekanntes Parse-Ziel: {args.target}")
        store.finish_run(run_id, status="ok", summary=summary)
        return summary
    except Exception as exc:
        store.finish_run(run_id, status="error", summary={"error": str(exc)})
        raise


def _run_normalize(args: argparse.Namespace, store: BrainStore) -> dict[str, Any]:
    run_id = store.begin_run(f"normalize_{args.target}")
    try:
        if args.target == "entities":
            summary = normalize_entities(store, rebuild=args.rebuild)
        elif args.target == "sheet-stats":
            summary = normalize_sheet_stats(store.conn, rebuild=args.rebuild)
        elif args.target == "sheet-tabs":
            summary = normalize_sheet_tabs(store.conn, rebuild=args.rebuild)
        else:
            raise ValueError(f"Unbekanntes Normalize-Ziel: {args.target}")
        store.finish_run(run_id, status="ok", summary=summary)
        return summary
    except Exception as exc:
        store.finish_run(run_id, status="error", summary={"error": str(exc)})
        raise


def _run_refresh_sheet(store: BrainStore, http: HttpClient, settings) -> dict[str, Any]:
    run_id = store.begin_run("refresh_sheet")
    try:
        pull_summary = pull_sheet(
            store, http,
            sheet_id=settings.sheet_id,
            gid=settings.sheet_gid,
            all_tabs=True,
            cache_ttl_seconds=0,
        )
        stats_summary = normalize_sheet_stats(store.conn, rebuild=True)
        tabs_summary = normalize_sheet_tabs(store.conn, rebuild=True)
        summary = {"pull": pull_summary, "normalize_sheet_stats": stats_summary, "normalize_sheet_tabs": tabs_summary}
        store.finish_run(run_id, status="ok", summary=summary)
        return summary
    except Exception as exc:
        store.finish_run(run_id, status="error", summary={"error": str(exc)})
        raise


def _run_enrich(args: argparse.Namespace, store: BrainStore, settings) -> dict[str, Any]:
    run_id = store.begin_run(f"enrich_{args.target}")
    try:
        if args.target == "patch-events":
            summary = build_patch_event_enrichments(store.conn, rebuild=args.rebuild)
        elif args.target == "lineage":
            summary = build_entity_lineage(store.conn, rebuild=args.rebuild)
        elif args.target == "legacy-entities":
            summary = build_legacy_entities(store.conn, rebuild=args.rebuild)
        elif args.target == "patch-impact":
            from deadlock_brain.patch_impact import run_patch_impact_batch, build_patch_impact_context, build_patch_impact_request
            config = _build_minimax_config(settings)
            if args.dry_run:
                if not args.hero:
                    raise ValueError("Fuer --dry-run muss --hero angegeben werden.")
                # We need the entity_type, we can just guess or lookup
                cursor = store.conn.execute("SELECT entity_type FROM entities WHERE canonical_name = ? LIMIT 1", (args.hero,))
                row = cursor.fetchone()
                entity_type = row[0] if row else "hero"
                context = build_patch_impact_context(store.conn, args.hero, entity_type)
                req_info = build_patch_impact_request(context, config)
                return {"dry_run": True, "prompt": req_info["prompt_text"], "request": req_info["request"]}
            elif args.hero:
                # If specific hero is requested without dry-run, we might need a custom flow or use limit=1.
                # However, run_patch_impact_batch uses list_pending, which might not pick this hero if it's already analyzed.
                # Since the instructions just said run_patch_impact_batch... Let's just reset its status if forced, or just run it directly.
                cursor = store.conn.execute("SELECT entity_type FROM entities WHERE canonical_name = ? LIMIT 1", (args.hero,))
                row = cursor.fetchone()
                entity_type = row[0] if row else "hero"
                from deadlock_brain.minimax_client import call_minimax_chat, extract_minimax_text
                from deadlock_brain.patch_impact import save_patch_impact_note
                context = build_patch_impact_context(store.conn, args.hero, entity_type)
                req_info = build_patch_impact_request(context, config)
                response = call_minimax_chat(req_info["request"], config)
                result_text = extract_minimax_text(response)
                save_patch_impact_note(store.conn, context, prompt_text=req_info["prompt_text"], result_text=result_text, model=config.model, status="analysis_ready")
                summary = {"processed": 1, "success": 1, "failed": 0, "hero": args.hero}
            else:
                summary = run_patch_impact_batch(store.conn, config, limit=args.limit)
        elif args.target == "meta-trends":
            from deadlock_brain.meta_trends import run_meta_trend_analysis
            config = _build_minimax_config(settings)
            summary = run_meta_trend_analysis(store.conn, config)
        else:
            raise ValueError(f"Unbekanntes Enrich-Ziel: {args.target}")
        store.finish_run(run_id, status="ok", summary=summary)
        return summary
    except Exception as exc:
        store.finish_run(run_id, status="error", summary={"error": str(exc)})
        raise


def _run_analysis(args: argparse.Namespace, store: BrainStore, settings) -> dict[str, Any] | list[dict[str, Any]]:
    if args.target == "save-review":
        review_context = build_review_context(store.conn, args.query, limit_events=args.limit_events)
        return save_review_analysis_note(
            store.conn,
            review_context,
            result_text=args.result_text,
            model=args.model,
            confidence=args.confidence,
            status="analysis_ready" if args.result_text else "context_ready",
        )
    if args.target == "list":
        return list_analysis_notes(store.conn, query=args.query, limit=args.limit)
    if args.target == "run-minimax":
        review_context = build_review_context(store.conn, args.query, limit_events=args.limit_events)
        config = MiniMaxConfig(
            api_key=settings.minimax_api_key,
            base_url=settings.minimax_base_url,
            model=args.model or settings.minimax_model,
            timeout_seconds=settings.minimax_timeout_seconds,
            max_completion_tokens=args.max_completion_tokens or settings.minimax_max_completion_tokens,
            temperature=args.temperature if args.temperature is not None else settings.minimax_temperature,
            top_p=args.top_p if args.top_p is not None else settings.minimax_top_p,
            use_token_plan=settings.minimax_use_token_plan,
        )
        request_payload = build_minimax_review_request(review_context, config)
        endpoint = f"{config.base_url.rstrip('/')}/chat/completions"
        if args.dry_run:
            return {
                "dry_run": True,
                "query": args.query,
                "model": config.model,
                "base_url": config.base_url,
                "endpoint": endpoint,
                "api_key_present": bool(config.api_key),
                "request": request_payload,
            }
        response = call_minimax_chat(request_payload, config)
        result_text = extract_minimax_text(response)
        if not result_text:
            raise RuntimeError("MiniMax response did not include message content.")
        provider_metadata = minimax_usage_summary(response)
        note = save_review_analysis_note(
            store.conn,
            review_context,
            result_text=result_text,
            model=config.model,
            status="analysis_ready",
            provider_metadata=provider_metadata,
        )
        return {
            "note": note,
            "query": args.query,
            "model": config.model,
            "endpoint": endpoint,
            "result_text": result_text,
            "provider_metadata": provider_metadata,
        }
    raise ValueError(f"Unbekanntes Analysis-Ziel: {args.target}")


def _run_learn(args: argparse.Namespace, store: BrainStore, settings) -> dict[str, Any] | list[dict[str, Any]]:
    if args.target == "import-steam-builds":
        default_steam_db = settings.central_deadlock_db_path
        steam_db_path = Path(args.db_path).expanduser() if args.db_path else default_steam_db
        return import_steam_builds(
            store.conn,
            steam_db_path=steam_db_path,
            hero=args.hero,
            language=args.language,
            limit_per_hero=args.limit_per_hero,
        )
    if args.target == "list-builds":
        return list_learned_builds(store.conn, hero=args.hero, limit=args.limit)
    if args.target == "analyze-build":
        config = _build_learn_minimax_config(args, settings)
        return _run_single_build_learning_analysis(store, args.build_id, config, dry_run=args.dry_run, include_request=True)
    if args.target == "analyze-next":
        config = _build_learn_minimax_config(args, settings)
        targets = list_pending_build_learning_targets(store.conn, hero=args.hero, limit=args.limit, model=config.model)
        if not args.dry_run and not config.api_key:
            raise RuntimeError("MiniMax API key fehlt. Setze MINIMAX_API_KEY oder MINIMAX_TOKEN_PLAN_KEY.")
        results = []
        for index, target in enumerate(targets):
            try:
                result = _run_single_build_learning_analysis(
                    store,
                    int(target["id"]),
                    config,
                    dry_run=args.dry_run,
                    include_request=False,
                )
                note = result.get("note") if isinstance(result.get("note"), dict) else {}
                results.append(
                    {
                        "build_id": target["id"],
                        "hero_name": target.get("hero_name"),
                        "source_rank": target.get("source_rank"),
                        "quality_tier": target.get("quality_tier"),
                        "name": target.get("name"),
                        "status": note.get("status") or ("context_ready" if args.dry_run else "analysis_ready"),
                        "note_id": note.get("id"),
                        "provider_metadata": result.get("provider_metadata"),
                    }
                )
            except Exception as exc:
                results.append(
                    {
                        "build_id": target.get("id"),
                        "hero_name": target.get("hero_name"),
                        "source_rank": target.get("source_rank"),
                        "name": target.get("name"),
                        "status": "error",
                        "error": str(exc),
                    }
                )
                if not args.dry_run:
                    break
            if not args.dry_run and index < len(targets) - 1 and args.delay_seconds > 0:
                time.sleep(float(args.delay_seconds))
        return {
            "dry_run": bool(args.dry_run),
            "hero": args.hero,
            "limit": args.limit,
            "model": config.model,
            "api_key_present": bool(config.api_key),
            "pending_selected": len(targets),
            "results": results,
        }
    raise ValueError(f"Unbekanntes Learn-Ziel: {args.target}")


def _run_player(args: argparse.Namespace, store: BrainStore, settings) -> dict[str, Any] | list[dict[str, Any]]:
    if args.target == "list-matches":
        return list_player_matches(store.conn, account_id=args.account_id, limit=args.limit)
    if args.target == "match-context":
        return build_player_match_decision_context(store.conn, account_id=args.account_id, match_id=args.match_id)
    if args.target == "analyze-match":
        config = _build_learn_minimax_config(args, settings)
        return _run_single_player_match_analysis(
            store,
            account_id=args.account_id,
            match_id=args.match_id,
            config=config,
            dry_run=args.dry_run,
            include_request=True,
        )
    if args.target == "analyze-next":
        config = _build_learn_minimax_config(args, settings)
        targets = list_pending_player_match_decision_targets(
            store.conn,
            account_id=args.account_id,
            limit=args.limit,
            model=config.model,
        )
        if not args.dry_run and not config.api_key:
            raise RuntimeError("MiniMax API key fehlt. Setze MINIMAX_API_KEY oder MINIMAX_TOKEN_PLAN_KEY.")
        results = []
        for index, target in enumerate(targets):
            try:
                result = _run_single_player_match_analysis(
                    store,
                    account_id=str(target["account_id"]),
                    match_id=str(target["match_id"]),
                    config=config,
                    dry_run=args.dry_run,
                    include_request=False,
                )
                note = result.get("note") if isinstance(result.get("note"), dict) else {}
                results.append(
                    {
                        "account_id": target.get("account_id"),
                        "match_id": target.get("match_id"),
                        "hero_id": result.get("hero_id") or target.get("hero_id"),
                        "hero_name": result.get("hero_name"),
                        "status": note.get("status") or ("context_ready" if args.dry_run else "analysis_ready"),
                        "note_id": note.get("id"),
                        "provider_metadata": result.get("provider_metadata"),
                    }
                )
            except Exception as exc:
                results.append(
                    {
                        "account_id": target.get("account_id"),
                        "match_id": target.get("match_id"),
                        "hero_id": target.get("hero_id"),
                        "status": "error",
                        "error": str(exc),
                    }
                )
                if not args.dry_run:
                    break
            if not args.dry_run and index < len(targets) - 1 and args.delay_seconds > 0:
                time.sleep(float(args.delay_seconds))
        return {
            "dry_run": bool(args.dry_run),
            "account_id": args.account_id,
            "limit": args.limit,
            "model": config.model,
            "api_key_present": bool(config.api_key),
            "pending_selected": len(targets),
            "results": results,
        }
    raise ValueError(f"Unbekanntes Player-Ziel: {args.target}")


def _run_single_player_match_analysis(
    store: BrainStore,
    *,
    account_id: str,
    match_id: str,
    config: MiniMaxConfig,
    dry_run: bool,
    include_request: bool,
) -> dict[str, Any]:
    context = build_player_match_decision_context(store.conn, account_id=account_id, match_id=match_id)
    request_payload = build_minimax_player_match_decision_request(context, config)
    prompt_text = "\n\n".join(str(message.get("content") or "") for message in request_payload.get("messages") or [])
    endpoint = f"{config.base_url.rstrip('/')}/chat/completions"
    if dry_run:
        note = save_player_match_decision_note(
            store.conn,
            context,
            prompt_text=prompt_text,
            result_text=None,
            model=config.model,
            status="context_ready",
        )
        result = {
            "dry_run": True,
            "account_id": account_id,
            "match_id": match_id,
            "hero_id": context.get("hero_id"),
            "hero_name": context.get("hero_name"),
            "model": config.model,
            "endpoint": endpoint,
            "api_key_present": bool(config.api_key),
            "note": note,
        }
        if include_request:
            result["request"] = request_payload
        return result
    response = call_minimax_chat(request_payload, config)
    result_text = extract_minimax_text(response)
    if not result_text:
        raise RuntimeError("MiniMax response did not include message content.")
    note = save_player_match_decision_note(
        store.conn,
        context,
        prompt_text=prompt_text,
        result_text=result_text,
        model=config.model,
        status="analysis_ready",
    )
    return {
        "account_id": account_id,
        "match_id": match_id,
        "hero_id": context.get("hero_id"),
        "hero_name": context.get("hero_name"),
        "model": config.model,
        "endpoint": endpoint,
        "note": note,
        "result_text": result_text,
        "provider_metadata": minimax_usage_summary(response),
    }


def _build_learn_minimax_config(args: argparse.Namespace, settings) -> MiniMaxConfig:
    return MiniMaxConfig(
        api_key=settings.minimax_api_key,
        base_url=settings.minimax_base_url,
        model=args.model or settings.minimax_model,
        timeout_seconds=settings.minimax_timeout_seconds,
        max_completion_tokens=args.max_completion_tokens or settings.minimax_max_completion_tokens,
        temperature=args.temperature if args.temperature is not None else settings.minimax_temperature,
        top_p=args.top_p if args.top_p is not None else settings.minimax_top_p,
        use_token_plan=settings.minimax_use_token_plan,
    )


def _run_single_build_learning_analysis(
    store: BrainStore,
    build_id: int,
    config: MiniMaxConfig,
    *,
    dry_run: bool,
    include_request: bool,
) -> dict[str, Any]:
    context = build_learning_context(store.conn, build_id)
    request_payload = build_minimax_build_learning_request(context, config)
    prompt_text = "\n\n".join(str(message.get("content") or "") for message in request_payload.get("messages") or [])
    endpoint = f"{config.base_url.rstrip('/')}/chat/completions"
    if dry_run:
        note = save_build_learning_note(
            store.conn,
            context,
            prompt_text=prompt_text,
            result_text=None,
            model=config.model,
            status="context_ready",
        )
        result = {
            "dry_run": True,
            "build_id": build_id,
            "model": config.model,
            "endpoint": endpoint,
            "api_key_present": bool(config.api_key),
            "note": note,
        }
        if include_request:
            result["request"] = request_payload
        return result
    response = call_minimax_chat(request_payload, config)
    result_text = extract_minimax_text(response)
    if not result_text:
        raise RuntimeError("MiniMax response did not include message content.")
    note = save_build_learning_note(
        store.conn,
        context,
        prompt_text=prompt_text,
        result_text=result_text,
        model=config.model,
        status="analysis_ready",
    )
    return {
        "build_id": build_id,
        "model": config.model,
        "endpoint": endpoint,
        "note": note,
        "result_text": result_text,
        "provider_metadata": minimax_usage_summary(response),
    }


def _run_pull(args: argparse.Namespace, store: BrainStore, http: HttpClient, settings) -> dict[str, Any]:
    run_id = store.begin_run(args.source)
    try:
        if args.source == "assets":
            summary = pull_assets(store, http, kinds=args.kind)
        elif args.source == "sheet":
            summary = pull_sheet(
                store,
                http,
                sheet_id=args.sheet_id or settings.sheet_id,
                gid=args.gid or settings.sheet_gid,
                all_tabs=args.all_tabs,
                cache_ttl_seconds=args.cache_ttl_seconds,
            )
        elif args.source == "patchnotes":
            from pathlib import Path

            summary = pull_patchnotes(
                store,
                central_db_path=Path(args.db_path).expanduser() if args.db_path else settings.central_deadlock_db_path,
            )
        elif args.source == "wiki":
            summary = pull_wiki_page(
                store,
                http,
                title=args.title,
                enabled=settings.wiki_enabled or args.allow_wiki_network,
                cache_ttl_seconds=settings.wiki_cache_ttl_seconds,
                min_delay_seconds=settings.wiki_min_delay_seconds,
            )
        elif args.source == "statlocker":
            summary = pull_statlocker(
                store,
                http,
                kinds=args.kind,
                patch=args.patch,
                hero=args.hero,
                min_sample_size=args.min_sample_size,
                rank=args.rank,
                leaderboard_page=args.leaderboard_page,
                leaderboard_page_size=args.leaderboard_page_size,
                account_id=args.account_id,
                match_id=args.match_id,
                hero_id=args.hero_id,
                players_from_leaderboard=args.players_from_leaderboard,
                matches_per_player=args.matches_per_player,
                include_match_details=args.include_match_details,
                include_build_analysis=args.include_build_analysis,
                game_mode=args.game_mode,
                delay_seconds=args.delay_seconds,
                cache_ttl_seconds=args.cache_ttl_seconds,
            )
        elif args.source == "deadlock-api":
            summary = pull_match_metadata(
                store,
                http,
                match_ids=args.match_id,
                account_ids=args.account_id,
                hero_ids=args.hero_id,
                include_player_items=not args.no_player_items,
                include_player_stats=not args.no_player_stats,
                include_player_death_details=not args.no_death_details,
                include_objectives=not args.no_objectives,
                cache_ttl_seconds=args.cache_ttl_seconds,
            )
        elif args.source == "all":
            summary = {
                "assets": pull_assets(store, http),
                "sheet": pull_sheet(store, http, sheet_id=settings.sheet_id, gid=settings.sheet_gid, all_tabs=True),
                "patchnotes": pull_patchnotes(store, central_db_path=settings.central_deadlock_db_path),
                "wiki": "skipped: wiki is opt-in and never included in bulk pulls",
                "statlocker": "skipped: statlocker is opt-in and never included in bulk pulls",
                "deadlock_api": "skipped: match metadata is opt-in and needs match ids",
            }
        else:
            raise ValueError(f"Unbekannte Quelle: {args.source}")
        store.finish_run(run_id, status="ok", summary=summary)
        return summary
    except Exception as exc:
        store.finish_run(run_id, status="error", summary={"error": str(exc)})
        raise


def _print_status(store: BrainStore, settings) -> None:
    print(f"Project: {settings.project_root}")
    print(f"DB:      {settings.db_path}")
    print(f"Raw:     {settings.raw_dir}")
    print(f"Cache:   {settings.cache_dir}")
    print(f"Patch DB:{settings.central_deadlock_db_path}")
    print("")
    print("Source documents:")
    rows = store.latest_counts()
    if not rows:
        print("  none")
    for row in rows:
        print(f"  {row['source']}: {row['documents']}")
    print("")
    print("Entity snapshots:")
    rows = store.snapshot_counts()
    if not rows:
        print("  none")
    for row in rows:
        print(f"  {row['source']} / {row['entity_type']}: {row['snapshots']}")
    print("")
    print("Patch events:")
    rows = store.patch_event_counts()
    if not rows:
        print("  none")
    for row in rows:
        print(f"  {row['source_kind']} / {row['entity_type']}: {row['events']}")
    print("")
    print("Entities:")
    rows = store.entity_counts()
    if not rows:
        print("  none")
    for row in rows:
        print(f"  {row['entity_type']}: {row['entities']}")
    print("")
    print("Derived data:")
    derived_rows = _derived_counts(store)
    if not derived_rows:
        print("  none")
    for name, count in derived_rows:
        print(f"  {name}: {count}")


def _derived_counts(store: BrainStore) -> list[tuple[str, int]]:
    tables = (
        "patch_event_enrichments",
        "entity_lineage",
        "legacy_entities",
        "hero_stat_profiles",
        "hero_stat_values",
        "analysis_notes",
        "build_learning_notes",
        "player_match_decision_notes",
    )
    rows: list[tuple[str, int]] = []
    for table in tables:
        exists = store.conn.execute(
            "SELECT 1 FROM sqlite_master WHERE type='table' AND name=?",
            (table,),
        ).fetchone()
        if not exists:
            continue
        count = store.conn.execute(f"SELECT COUNT(*) AS count FROM {table}").fetchone()["count"]
        rows.append((table, int(count)))
    return rows


def _print_events(store: BrainStore, args: argparse.Namespace) -> None:
    where = []
    params: list[Any] = []
    if args.entity:
        where.append("(entity_name LIKE ? OR normalized_line LIKE ?)")
        params.extend([f"%{args.entity}%", f"%{args.entity}%"])
    if args.entity_type:
        where.append("entity_type=?")
        params.append(args.entity_type)
    if args.source_kind:
        where.append("source_kind=?")
        params.append(args.source_kind)
    sql = """
        SELECT posted_at, source_kind, section, entity_type, entity_name,
               change_type, normalized_line, patch_title, patch_url
        FROM patch_events
    """
    if where:
        sql += " WHERE " + " AND ".join(where)
    sql += " ORDER BY COALESCE(posted_at, ''), id LIMIT ?"
    params.append(max(1, min(args.limit, 500)))
    rows = store.conn.execute(sql, params).fetchall()
    if not rows:
        print("Keine Events gefunden.")
        return
    for row in rows:
        date = row["posted_at"] or "unknown-date"
        section = row["section"] or "Unsectioned"
        entity = row["entity_name"] or row["entity_type"]
        print(f"{date} [{row['source_kind']}] {section} / {entity} / {row['change_type']}")
        print(f"  {row['normalized_line']}")
        print(f"  {row['patch_url']}")


def _print_entities(store: BrainStore, args: argparse.Namespace) -> None:
    rows = store.find_entities(entity_type=args.entity_type, query=args.query, limit=args.limit)
    if not rows:
        print("Keine Entities gefunden.")
        return
    for row in rows:
        aliases = [alias for alias in (row["aliases"] or "").split(",") if alias and alias != row["canonical_name"]]
        suffix = f" ({row['primary_external_id']})" if row["primary_external_id"] else ""
        print(f"{row['entity_type']}: {row['canonical_name']}{suffix}")
        if aliases:
            print(f"  aliases: {', '.join(aliases[:8])}")


def _print_context(ctx: dict[str, Any]) -> None:
    match = ctx.get("best_match")
    if match:
        print(f"Best match: {match.get('entity_type')} / {match.get('canonical_name')} ({match.get('primary_external_id')})")
    else:
        print("Best match: none")
    print(f"Aliases: {len(ctx.get('aliases') or [])}")
    sheet = ctx.get("sheet_stats") or {}
    print(f"Sheet stats: {'yes' if sheet.get('available') else 'no'}")
    enrichments = ctx.get("enrichments") or {}
    print(f"Enrichments: {'yes' if enrichments.get('available') else 'no'} / {len(enrichments.get('rows') or [])} rows")
    events = ctx.get("patch_events") or []
    print(f"Patch events: {len(events)}")
    for event in events[:10]:
        date = event.get("posted_at") or "unknown-date"
        section = event.get("section") or "Unsectioned"
        entity = event.get("entity_name") or event.get("entity_type")
        print(f"- {date} [{event.get('source_kind')}] {section} / {entity} / {event.get('change_type')}")
        print(f"  {event.get('normalized_line')}")


def _print_timeline(timeline: dict[str, Any]) -> None:
    match = timeline.get("best_match")
    if match:
        print(f"Timeline: {match.get('entity_type')} / {match.get('canonical_name')} ({match.get('primary_external_id')})")
    else:
        print(f"Timeline: {timeline.get('query')} (fallback)")
    print(f"Patches: {timeline.get('patch_count')} / Events: {timeline.get('event_count')}")
    impact = timeline.get("impact_summary") or {}
    by_kind = impact.get("by_kind") or {}
    if by_kind:
        print("Impact kinds: " + ", ".join(f"{key}={value}" for key, value in by_kind.items()))
    for patch in (timeline.get("patches") or [])[:12]:
        date = patch.get("date") or "unknown-date"
        print(f"- {date} [{patch.get('source')}] {patch.get('title')}")
        for event in (patch.get("events") or [])[:8]:
            kind = event.get("impact_kind") or "unknown"
            level = event.get("impact_level") or "unknown"
            print(f"  - {kind}/{level}: {event.get('normalized_line') or event.get('raw_line')}")


def _print_review_context(review_context: dict[str, Any]) -> None:
    summary = review_context.get("entity_summary") or {}
    name = summary.get("name") or review_context.get("query")
    entity_type = summary.get("entity_type") or "unknown"
    print(f"Review context: {entity_type} / {name}")
    print(f"Matched: {'yes' if summary.get('matched') else 'no'}")
    timeline = review_context.get("timeline_signals") or {}
    print(f"Events: {timeline.get('event_count', 0)}")
    latest = timeline.get("latest_patch") or {}
    if latest:
        print(f"Latest patch: {latest.get('posted_at') or 'unknown-date'} / {latest.get('patch_title')}")
    stats = review_context.get("current_stat_hints") or {}
    print(f"Sheet stats: {'yes' if stats.get('available') else 'no'} / {len(stats.get('hints') or [])} hints")
    questions = review_context.get("open_questions") or []
    if questions:
        print("Open questions:")
        for question in questions[:10]:
            print(f"- {question}")


def _print_quality_report(report: dict[str, Any]) -> None:
    summary = report.get("summary") or {}
    print(f"Quality: {'ok' if report.get('ok') else 'needs attention'}")
    print(f"Checks: {summary.get('checks', 0)} / warnings={summary.get('warnings', 0)} / errors={summary.get('errors', 0)}")
    for check in report.get("checks") or []:
        severity = check.get("severity")
        if severity == "ok":
            continue
        print(f"- {severity} {check.get('check')}: {check.get('rows', 0)}")
        print(f"  {check.get('message')}")
        for sample in (check.get("samples") or [])[:3]:
            print(f"  sample: {sample}")


def _load_lineage(store: BrainStore, query: str | None, limit: int) -> list[dict[str, Any]]:
    max_rows = max(1, min(int(limit), 500))
    if not store.conn.execute("SELECT 1 FROM sqlite_master WHERE type='table' AND name='entity_lineage'").fetchone():
        return []
    if query:
        return related_names_for_query(store.conn, query)[:max_rows]
    rows = store.conn.execute(
        """
        SELECT *
        FROM entity_lineage
        ORDER BY patch_event_id DESC, id DESC
        LIMIT ?
        """,
        (max_rows,),
    ).fetchall()
    loaded = []
    for row in rows:
        item = {key: row[key] for key in row.keys()}
        try:
            item["metadata"] = json.loads(str(item.pop("metadata_json", "{}")))
        except json.JSONDecodeError:
            item["metadata"] = {}
        loaded.append(item)
    return loaded


def _print_lineage(rows: list[dict[str, Any]], query: str | None) -> None:
    label = query or "latest"
    print(f"Lineage: {label} / rows={len(rows)}")
    for row in rows:
        relation = row.get("relation_type")
        source = row.get("source_name")
        target = row.get("target_name")
        owner = row.get("owner_name")
        metadata = row.get("metadata") or {}
        owner_text = f" / owner={owner}" if owner else ""
        target_text = f" -> {target}" if target else ""
        print(f"- {relation}: {source}{target_text}{owner_text}")
        print(f"  {metadata.get('posted_at') or 'unknown-date'} / {metadata.get('patch_title')}")
        line = metadata.get("line")
        if line:
            print(f"  {line}")


def _load_legacy(store: BrainStore, query: str | None, limit: int) -> list[dict[str, Any]]:
    max_rows = max(1, min(int(limit), 500))
    if not store.conn.execute("SELECT 1 FROM sqlite_master WHERE type='table' AND name='legacy_entities'").fetchone():
        return []
    params: list[Any] = []
    sql = "SELECT * FROM legacy_entities"
    if query:
        sql += " WHERE canonical_name LIKE ? OR name_norm LIKE ?"
        params.extend([f"%{query}%", f"%{query.casefold()}%"])
    sql += " ORDER BY confidence DESC, event_count DESC, canonical_name LIMIT ?"
    params.append(max_rows)
    rows = store.conn.execute(sql, tuple(params)).fetchall()
    loaded = []
    for row in rows:
        item = {key: row[key] for key in row.keys()}
        try:
            item["samples"] = json.loads(str(item.pop("samples_json", "[]")))
        except json.JSONDecodeError:
            item["samples"] = []
        loaded.append(item)
    return loaded


def _print_legacy(rows: list[dict[str, Any]], query: str | None) -> None:
    label = query or "top"
    print(f"Legacy entities: {label} / rows={len(rows)}")
    for row in rows:
        print(
            f"- {row.get('legacy_type')}: {row.get('canonical_name')} "
            f"events={row.get('event_count')} confidence={row.get('confidence')} status={row.get('status')}"
        )
        samples = row.get("samples") or []
        if samples:
            sample = samples[0]
            print(f"  {sample.get('posted_at') or 'unknown-date'} / {sample.get('patch_title')}")
            print(f"  {sample.get('line')}")


def _print_build_context(ctx: dict[str, Any]) -> None:
    hero = ctx.get("hero") or {}
    economy = ctx.get("economy") or {}
    build = ctx.get("build") or {}
    print(f"Build: {hero.get('name')} / {hero.get('hero_type')}")
    role = hero.get("role")
    playstyle = hero.get("playstyle")
    if role:
        print(f"Role: {role}")
    if playstyle:
        print(f"Playstyle: {playstyle}")
    gameplan = hero.get("inferred_gameplan") or {}
    if gameplan:
        print(f"Understood gameplan: {gameplan.get('summary')}")
        needs = gameplan.get("needs") or []
        if needs:
            print(f"Needs: {', '.join(str(need) for need in needs)}")
        damage_profile = gameplan.get("damage_profile") or {}
        if damage_profile:
            print(
                "Damage profile: "
                f"{damage_profile.get('damage_plan')} "
                f"(ability_damage={damage_profile.get('ability_damage_sources')}, "
                f"spirit_scaling={damage_profile.get('spirit_scaling_damage_sources')}, "
                f"weapon_hooks={damage_profile.get('weapon_stat_sources')})"
            )
        priority_stats = gameplan.get("priority_scaling_stats") or []
        if priority_stats:
            print(f"Priority scaling: {', '.join(str(stat) for stat in priority_stats)}")
        abilities = hero.get("abilities") or []
        if abilities:
            print("Abilities:")
            for ability in abilities:
                tags = ", ".join(str(tag) for tag in (ability.get("role_tags") or []))
                key_props = []
                for prop in (ability.get("key_properties") or [])[:4]:
                    key_props.append(f"{prop.get('label')}: {_format_prop_value(prop)}")
                print(f"- {ability.get('name')}: {tags}")
                if key_props:
                    print(f"  {'; '.join(key_props)}")
        lane_priorities = gameplan.get("lane_priorities") or []
        if lane_priorities:
            print("Lane priorities:")
            for priority in lane_priorities:
                print(f"- {priority}")
    print(f"Shop spike: {economy.get('important_shop_spike')} souls per category")
    wiki_rules = economy.get("wiki_rules") or {}
    if wiki_rules:
        tiers = ", ".join(str(tier) for tier in (wiki_rules.get("item_tiers") or []))
        print(f"Shop rules: tiers={tiers}; active limit={wiki_rules.get('active_item_limit')}; bonus cap={wiki_rules.get('shop_bonus_cap')}")
    print(f"Plan: {build.get('plan')}")
    lane_model = build.get("lane_decision_model") or []
    if lane_model:
        print("Lane decision model:")
        for line in lane_model:
            print(f"- {line}")
    statlocker_signals = ctx.get("statlocker_signals") or []
    if statlocker_signals:
        print("Statlocker WPA signals:")
        for signal in statlocker_signals[:8]:
            wpa = signal.get("cost_relative_wpa") if signal.get("cost_relative_wpa") is not None else signal.get("wpa")
            print(f"- {signal.get('item')}: wpa={wpa} n={signal.get('sample_size')} time={signal.get('mean_purchase_time_min')}")
    print("")
    for section in ("early", "core", "late", "situational"):
        rows = list(build.get(section) or [])
        if not rows:
            continue
        print(section.capitalize() + ":")
        for row in rows:
            print(f"- {row.get('name')} [{row.get('slot')} T{row.get('tier')} / {row.get('cost')}] score={row.get('score')}")
            archetypes = row.get("archetypes") or []
            if archetypes:
                print(f"  archetypes: {', '.join(str(item) for item in archetypes)}")
            why = row.get("why") or []
            if why:
                print(f"  why: {'; '.join(str(item) for item in why)}")
            warnings = row.get("warnings") or []
            if warnings:
                print(f"  warn: {'; '.join(str(item) for item in warnings)}")
        print("")
    targets = build.get("shop_routes_to_4800") or {}
    if targets:
        print("Shop routes to 4800:")
        for slot, target in targets.items():
            route_names = ", ".join(str(row.get("name")) for row in (target.get("items") or [])[:5])
            print(
                f"- {target.get('label') or slot}: spend={target.get('spend')} "
                f"/ reached={'yes' if target.get('target_reached') else 'no'} / {route_names}"
            )
    rejected = list(ctx.get("rejected_expensive_items") or [])
    if rejected:
        print("")
        print("Expensive items to justify:")
        for row in rejected[:6]:
            warnings = "; ".join(str(item) for item in (row.get("warnings") or []))
            print(f"- {row.get('name')} cost={row.get('cost')} score={row.get('score')} bucket={row.get('bucket')} {warnings}")


def _print_item_context(ctx: dict[str, Any]) -> None:
    print(f"Item: {ctx.get('name')} [{ctx.get('slot')} T{ctx.get('tier')} / {ctx.get('cost')}]")
    print(f"Active: {'yes' if ctx.get('is_active') else 'no'} / activation={ctx.get('activation')}")
    archetypes = ctx.get("archetypes") or []
    if archetypes:
        print(f"Archetypes: {', '.join(str(item) for item in archetypes)}")
    description = ctx.get("description")
    if description:
        print(f"Desc: {description}")
    properties = list(ctx.get("properties") or [])
    if properties:
        print("Properties:")
        for prop in properties[:20]:
            print(f"- {prop.get('label')}: {_format_prop_value(prop)}")
            scales = prop.get("scales_with") or []
            if scales:
                print(f"  scales: {', '.join(str(item) for item in scales)}")


def _format_prop_value(prop: dict[str, Any]) -> str:
    prefix = str(prop.get("prefix") or "")
    value = str(prop.get("value") if prop.get("value") is not None else "")
    postfix = str(prop.get("postfix") or "")
    if postfix and value.endswith(postfix.strip()):
        postfix = ""
    return f"{prefix}{value}{postfix}"


def _print_analysis_result(args: argparse.Namespace, result: dict[str, Any] | list[dict[str, Any]]) -> None:
    if args.target == "save-review":
        assert isinstance(result, dict)
        print(f"Analysis note: {result.get('id')} / {result.get('query')} / {result.get('status')}")
        print(f"Entity: {result.get('entity_type')} / {result.get('entity_name')}")
        print(f"Context hash: {result.get('context_hash')}")
        return
    if args.target == "run-minimax":
        assert isinstance(result, dict)
        if result.get("dry_run"):
            request = result.get("request") if isinstance(result.get("request"), dict) else {}
            messages = request.get("messages") if isinstance(request.get("messages"), list) else []
            print(f"MiniMax dry-run: {result.get('model')}")
            print(f"Endpoint: {result.get('endpoint')}")
            print(f"API key present: {'yes' if result.get('api_key_present') else 'no'}")
            print(f"Messages: {len(messages)}")
            return
        note = result.get("note") if isinstance(result.get("note"), dict) else {}
        print(f"MiniMax analysis: note={note.get('id')} model={result.get('model')}")
        print(result.get("result_text") or "")
        return
    assert isinstance(result, list)
    print(f"Analysis notes: rows={len(result)}")
    for row in result:
        print(f"- {row.get('id')}: {row.get('query')} / {row.get('entity_type')} / {row.get('entity_name')} / {row.get('status')}")
        print(f"  {row.get('prompt_version')} / {row.get('model') or 'no-model'} / {row.get('context_hash')}")


def _print_learn_result(args: argparse.Namespace, result: dict[str, Any] | list[dict[str, Any]]) -> None:
    if args.target == "import-steam-builds":
        assert isinstance(result, dict)
        print(f"Steam build import: {result.get('steam_db_path')}")
        print(
            f"rows={result.get('rows_seen', 0)} imported={result.get('imported', 0)} "
            f"updated={result.get('updated', 0)} skipped={result.get('skipped', 0)}"
        )
        if result.get("error"):
            print(f"error={result.get('error')}")
        return
    if args.target == "list-builds":
        assert isinstance(result, list)
        print(f"Learned builds: rows={len(result)}")
        for row in result:
            items = ", ".join(str(item) for item in (row.get("item_names") or [])[:8])
            print(
                f"- {row.get('id')}: {row.get('hero_name')} rank={row.get('source_rank')} "
                f"quality={row.get('quality_tier')} score={row.get('quality_score')} / {row.get('name')}"
            )
            if items:
                print(f"  items: {items}")
        return
    if args.target == "analyze-build":
        assert isinstance(result, dict)
        if result.get("dry_run"):
            request = result.get("request") if isinstance(result.get("request"), dict) else {}
            messages = request.get("messages") if isinstance(request.get("messages"), list) else []
            print(f"Build learning dry-run: build={result.get('build_id')} model={result.get('model')}")
            print(f"Endpoint: {result.get('endpoint')}")
            print(f"API key present: {'yes' if result.get('api_key_present') else 'no'}")
            print(f"Messages: {len(messages)}")
            note = result.get("note") if isinstance(result.get("note"), dict) else {}
            if note:
                print(f"Stored context note: {note.get('id')} / {note.get('status')}")
            return
        print(f"Build learning analysis: build={result.get('build_id')} model={result.get('model')}")
        print(result.get("result_text") or "")
        return
    if args.target == "analyze-next":
        assert isinstance(result, dict)
        print(f"Build learning batch: selected={result.get('pending_selected', 0)} model={result.get('model')}")
        print(f"Dry run: {'yes' if result.get('dry_run') else 'no'}")
        print(f"API key present: {'yes' if result.get('api_key_present') else 'no'}")
        for row in result.get("results") or []:
            print(
                f"- {row.get('build_id')}: {row.get('hero_name')} rank={row.get('source_rank')} "
                f"{row.get('status')} note={row.get('note_id') or '-'} / {row.get('name')}"
            )
            if row.get("error"):
                print(f"  error: {row.get('error')}")
        return


def _print_player_result(args: argparse.Namespace, result: dict[str, Any] | list[dict[str, Any]]) -> None:
    if args.target == "list-matches":
        assert isinstance(result, list)
        print(f"Player matches: rows={len(result)}")
        for row in result:
            summary = row.get("summary") if isinstance(row.get("summary"), dict) else {}
            hero_id = row.get("hero_id") or summary.get("heroId") or summary.get("hero_id") or "-"
            won = summary.get("won")
            result_text = "unknown" if won is None else "win" if bool(won) else "loss"
            print(f"- account={row.get('account_id')} match={row.get('match_id')} hero_id={hero_id} result={result_text}")
        return
    if args.target == "match-context":
        assert isinstance(result, dict)
        print(f"Player match context: account={result.get('account_id')} match={result.get('match_id')}")
        print(f"Hero: {result.get('hero_name') or '-'} ({result.get('hero_id') or '-'})")
        player_match = result.get("player_match") if isinstance(result.get("player_match"), dict) else {}
        print(f"Match row keys: {', '.join(player_match.keys()) if player_match else '-'}")
        detail = result.get("match_detail") if isinstance(result.get("match_detail"), dict) else {}
        print(f"Match detail: {'yes' if detail else 'no'}")
        api_match = result.get("deadlock_api_match") if isinstance(result.get("deadlock_api_match"), dict) else {}
        player = api_match.get("player") if isinstance(api_match.get("player"), dict) else {}
        actual_items = player.get("actual_item_timeline") if isinstance(player.get("actual_item_timeline"), list) else []
        print(f"Deadlock API match: {'yes' if api_match else 'no'}")
        if actual_items:
            print(f"Actual item events: {len(actual_items)}")
        build_analysis = result.get("player_build_analysis") if isinstance(result.get("player_build_analysis"), dict) else {}
        print(f"Player build analysis: {'yes' if build_analysis else 'no'}")
        return
    if args.target == "analyze-match":
        assert isinstance(result, dict)
        if result.get("dry_run"):
            request = result.get("request") if isinstance(result.get("request"), dict) else {}
            messages = request.get("messages") if isinstance(request.get("messages"), list) else []
            print(f"Player match dry-run: account={result.get('account_id')} match={result.get('match_id')} model={result.get('model')}")
            print(f"Hero: {result.get('hero_name') or '-'} ({result.get('hero_id') or '-'})")
            print(f"Endpoint: {result.get('endpoint')}")
            print(f"API key present: {'yes' if result.get('api_key_present') else 'no'}")
            print(f"Messages: {len(messages)}")
            note = result.get("note") if isinstance(result.get("note"), dict) else {}
            if note:
                print(f"Stored context note: {note.get('id')} / {note.get('status')}")
            return
        print(f"Player match analysis: account={result.get('account_id')} match={result.get('match_id')} model={result.get('model')}")
        print(f"Hero: {result.get('hero_name') or '-'} ({result.get('hero_id') or '-'})")
        print(result.get("result_text") or "")
        return
    if args.target == "analyze-next":
        assert isinstance(result, dict)
        print(f"Player match batch: selected={result.get('pending_selected', 0)} model={result.get('model')}")
        print(f"Dry run: {'yes' if result.get('dry_run') else 'no'}")
        print(f"API key present: {'yes' if result.get('api_key_present') else 'no'}")
        for row in result.get("results") or []:
            print(
                f"- account={row.get('account_id')} match={row.get('match_id')} "
                f"hero={row.get('hero_name') or row.get('hero_id') or '-'} "
                f"{row.get('status')} note={row.get('note_id') or '-'}"
            )
            if row.get("error"):
                print(f"  error: {row.get('error')}")
        return


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
