#!/usr/bin/env python3
"""Run the retained Rust suite and source integration; never connect to a DB/Wiki.
Use the repository CI toolchain (Rust 1.97.1) on PATH. No credentials required.
Logs and exact command/exit evidence are written incrementally under reports/.
"""
from __future__ import annotations
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
REPORTS = Path(__file__).resolve().parent / "reports"
COMMANDS = [
    ("offline-suite", ["bash", "architecture/migration/s12/check.sh"]),
    ("contracts-source-tests", ["cargo", "test", "--manifest-path", "rust/Cargo.toml", "-p", "brain-contracts", "-p", "dbrain-sources", "--all-targets", "--locked", "--offline", "-j2"]),
    ("contracts-source-clippy", ["cargo", "clippy", "--manifest-path", "rust/Cargo.toml", "-p", "brain-contracts", "-p", "dbrain-sources", "--all-targets", "--no-deps", "--locked", "--offline", "--", "-D", "warnings"]),
    ("contracts-source-release", ["cargo", "build", "--manifest-path", "rust/Cargo.toml", "-p", "brain-contracts", "-p", "dbrain-sources", "--release", "--locked", "--offline", "-j2"]),
    ("retained-hero-tests", ["cargo", "test", "--manifest-path", "rust/Cargo.toml", "-p", "dbrain-retrieval", "--lib", "--locked", "--offline", "-j2", "hero_dossier"]),
    ("workspace-all-targets", ["cargo", "check", "--manifest-path", "rust/Cargo.toml", "--workspace", "--all-targets", "--locked", "--offline", "-j2"]),
    ("changed-source-format", ["rustfmt", "--check", "--edition", "2021", "--config", "skip_children=true", "rust/crates/dbrain-sources/src/lib.rs", "rust/crates/dbrain-sources/src/wiki_capture_io.rs", "rust/crates/dbrain-sources/tests/wiki_knowledge_contract.rs"]),
    ("contracts-format", ["cargo", "fmt", "--manifest-path", "rust/Cargo.toml", "-p", "brain-contracts", "--", "--check"]),
    ("diff-whitespace", ["git", "diff", "--check"]),
]

def main() -> int:
    env = os.environ.copy()
    env["SQLX_OFFLINE"] = "true"
    for name in ("DATABASE_URL", "DEADLOCK_CENTRAL_DSN", "BRAIN_DATABASE_URL", "PGSERVICE", "PGHOST", "PGPASSWORD"):
        env.pop(name, None)
    REPORTS.mkdir(parents=True, exist_ok=True)
    result = {"complete": False, "environment": {"SQLX_OFFLINE": "true", "database_credentials_removed": True}, "commands": []}
    for name in ("rustc", "cargo"):
        result["environment"][name] = subprocess.check_output([name, "--version"], env=env, text=True).strip()
    output = REPORTS / "completion-results.json"
    for label, command in COMMANDS:
        print(f"RUN {label}", flush=True)
        run = subprocess.run(command, cwd=ROOT, env=env, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, check=False)
        log = REPORTS / f"completion-{label}.log"
        # Keep captured output, normalizing trailing blank lines for git diff --check.
        log.write_text(run.stdout.rstrip() + "\n" if run.stdout.strip() else "")
        result["commands"].append({"name": label, "argv": command, "exit_code": run.returncode, "log": log.name})
        output.write_text(json.dumps(result, indent=2) + "\n")
        print(f"EXIT {run.returncode} {label}", flush=True)
        if run.returncode:
            print(run.stdout[-8000:], flush=True)
    paths = set()
    for relative in ("architecture/migration/s12/src", "architecture/migration/s12/tests", "architecture/migration/s12/examples", "architecture/migration/s12/fixtures", "rust/crates/brain-contracts"):
        paths.update(p for p in (ROOT / relative).rglob("*") if p.is_file())
    for relative in ("architecture/migration/s12/Cargo.toml", "architecture/migration/s12/Cargo.lock", "architecture/migration/s12/check.sh", "architecture/migration/s12/check-completion.py", "rust/Cargo.toml", "rust/Cargo.lock", "rust/crates/dbrain-sources/Cargo.toml", "rust/crates/dbrain-sources/src/lib.rs", "rust/crates/dbrain-sources/src/wiki_capture_io.rs", "rust/crates/dbrain-sources/tests/wiki_knowledge_contract.rs"):
        paths.add(ROOT / relative)
    result["source_sha256"] = {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(paths)}
    result["complete"] = True
    output.write_text(json.dumps(result, indent=2) + "\n")
    return int(any(c["exit_code"] for c in result["commands"]))

if __name__ == "__main__":
    sys.exit(main())
