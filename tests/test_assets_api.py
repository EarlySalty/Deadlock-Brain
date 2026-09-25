#!/usr/bin/env python3
"""Stdlib-Tests fuer die Python-Assets-Strecke. Nur Parsing/Validierung, kein Netz, keine DB."""
from __future__ import annotations

import io
import sys
import unittest
from contextlib import redirect_stderr, redirect_stdout
from pathlib import Path
from unittest.mock import MagicMock

SRC = Path(__file__).resolve().parent.parent / "src"
if str(SRC) not in sys.path:
    sys.path.insert(0, str(SRC))

from deadlock_brain.cli import build_parser  # noqa: E402
from deadlock_brain.sources import assets_api  # noqa: E402


class ResolveKindsTest(unittest.TestCase):
    def test_default_is_items_and_heroes(self):
        self.assertEqual(assets_api.resolve_kinds(None), ["items", "heroes"])
        self.assertEqual(assets_api.resolve_kinds([]), ["items", "heroes"])

    def test_supported_kinds_pass_through(self):
        kinds = ["items", "heroes", "heroes_all", "ranks", "colors", "build_tags", "npc_units"]
        self.assertEqual(assets_api.resolve_kinds(kinds), kinds)

    def test_retired_kinds_are_rejected(self):
        for kind in sorted(assets_api.RETIRED_KINDS):
            with self.subTest(kind=kind):
                with self.assertRaises(ValueError) as ctx:
                    assets_api.resolve_kinds([kind])
                self.assertIn(kind, str(ctx.exception))
                self.assertIn("api.deadlock-api.com", str(ctx.exception))


class PullAssetsGuardTest(unittest.TestCase):
    def test_base_url_and_endpoints_match_rust_fetcher(self):
        self.assertEqual(assets_api.BASE_URL, "https://api.deadlock-api.com")
        self.assertEqual(
            sorted(assets_api.ENDPOINTS),
            ["build_tags", "colors", "heroes", "heroes_all", "items", "npc_units", "ranks"],
        )
        for endpoint in assets_api.ENDPOINTS.values():
            self.assertTrue(endpoint.startswith("/v1/assets/"), endpoint)

    def test_default_urls_use_main_api_base(self):
        store, http = MagicMock(), MagicMock()
        http.get_json.return_value = []
        store.insert_many_snapshots.return_value = 0

        summary = assets_api.pull_assets(store, http, kinds=None)

        self.assertEqual(
            [call.args[0] for call in http.get_json.call_args_list],
            [
                "https://api.deadlock-api.com/v1/assets/items",
                "https://api.deadlock-api.com/v1/assets/heroes?only_active=true",
            ],
        )
        self.assertEqual(sorted(summary["endpoints"]), ["heroes", "items"])
        self.assertEqual(summary["snapshots"], 0)

    def test_retired_or_unknown_kinds_write_nothing(self):
        for kinds in (["items", "raw_items"], ["raw_items", "items"], ["raw_heroes"], ["nope"]):
            with self.subTest(kinds=kinds):
                store, http = MagicMock(), MagicMock()
                with self.assertRaises(ValueError):
                    assets_api.pull_assets(store, http, kinds=kinds)
                http.get_json.assert_not_called()
                store.write_raw.assert_not_called()
                store.upsert_source_document.assert_not_called()
                store.insert_many_snapshots.assert_not_called()


class CliChoicesTest(unittest.TestCase):
    def test_cli_accepts_supported_kinds(self):
        args = build_parser().parse_args(["pull", "assets", "--kind", "items", "--kind", "ranks"])
        self.assertEqual(args.kind, ["items", "ranks"])

    def test_cli_rejects_retired_kind(self):
        stderr = io.StringIO()
        with self.assertRaises(SystemExit) as ctx, redirect_stderr(stderr):
            build_parser().parse_args(["pull", "assets", "--kind", "raw_items"])
        self.assertEqual(ctx.exception.code, 2)
        self.assertIn("invalid choice", stderr.getvalue())

    def test_cli_help_shows_new_default_without_raw_kinds(self):
        stdout = io.StringIO()
        with self.assertRaises(SystemExit) as ctx, redirect_stdout(stdout):
            build_parser().parse_args(["pull", "assets", "--help"])
        self.assertEqual(ctx.exception.code, 0)
        text = " ".join(stdout.getvalue().split())
        self.assertIn("Default: items/heroes", text)
        self.assertNotIn("raw_items", text)
        self.assertNotIn("raw_heroes", text)


if __name__ == "__main__":
    unittest.main()
