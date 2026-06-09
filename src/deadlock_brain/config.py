from __future__ import annotations

import os
from dataclasses import dataclass
from pathlib import Path


PROJECT_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_DATA_DIR = PROJECT_ROOT / "data"


def _load_dotenv(path: Path) -> None:
    if not path.exists():
        return
    for line in path.read_text(encoding="utf-8").splitlines():
        stripped = line.strip()
        if not stripped or stripped.startswith("#") or "=" not in stripped:
            continue
        key, value = stripped.split("=", 1)
        key = key.strip()
        value = value.strip().strip('"').strip("'")
        if key and key not in os.environ:
            os.environ[key] = value


def _path_env(name: str, default: Path) -> Path:
    raw = os.getenv(name)
    if not raw:
        return default
    return Path(raw).expanduser()


@dataclass(frozen=True)
class Settings:
    project_root: Path
    data_dir: Path
    raw_dir: Path
    cache_dir: Path
    db_path: Path
    central_deadlock_db_path: Path
    user_agent: str
    sheet_id: str
    sheet_gid: str
    wiki_enabled: bool
    wiki_min_delay_seconds: float
    wiki_cache_ttl_seconds: int
    minimax_api_key: str | None
    minimax_base_url: str
    minimax_model: str
    minimax_timeout_seconds: int
    minimax_max_completion_tokens: int
    minimax_temperature: float
    minimax_top_p: float
    minimax_use_token_plan: bool


def load_settings() -> Settings:
    _load_dotenv(PROJECT_ROOT / ".env")
    data_dir = _path_env("DEADLOCK_BRAIN_DATA_DIR", DEFAULT_DATA_DIR)
    db_path = _path_env("DEADLOCK_BRAIN_DB_PATH", data_dir / "deadlock_brain.sqlite3")
    central_db = _path_env(
        "DEADLOCK_DB_PATH",
        PROJECT_ROOT.parent / "Deadlock-Bots" / "data" / "deadlock.sqlite3",
    )
    wiki_enabled = os.getenv("DEADLOCK_BRAIN_WIKI_ENABLED", "0").strip().lower()
    minimax_token_plan_key = os.getenv("MINIMAX_TOKEN_PLAN_KEY")
    minimax_api_key = os.getenv("MINIMAX_API_KEY") or minimax_token_plan_key or None
    minimax_use_token_plan = bool(minimax_token_plan_key and minimax_api_key == minimax_token_plan_key)
    default_minimax_base_url = (
        "https://api.minimax.io/anthropic/v1" if minimax_use_token_plan else "https://api.minimax.io/v1"
    )
    return Settings(
        project_root=PROJECT_ROOT,
        data_dir=data_dir,
        raw_dir=data_dir / "raw",
        cache_dir=data_dir / "cache",
        db_path=db_path,
        central_deadlock_db_path=central_db,
        user_agent=os.getenv(
            "DEADLOCK_BRAIN_USER_AGENT",
            "DeadlockBrain/0.1 contact=admin@earlysalty.com",
        ),
        sheet_id=os.getenv(
            "DEADLOCK_STATS_SHEET_ID",
            "1fj9XMQmVUY0FY4cbozvB18PMBnpbdsaMFTZRLa74VRY",
        ),
        sheet_gid=os.getenv("DEADLOCK_STATS_SHEET_GID", "0"),
        wiki_enabled=wiki_enabled in {"1", "true", "yes", "on"},
        wiki_min_delay_seconds=float(os.getenv("DEADLOCK_BRAIN_WIKI_MIN_DELAY_SECONDS", "5")),
        wiki_cache_ttl_seconds=int(os.getenv("DEADLOCK_BRAIN_WIKI_CACHE_TTL_SECONDS", "604800")),
        minimax_api_key=minimax_api_key,
        minimax_base_url=os.getenv(
            "MINIMAX_TOKEN_PLAN_BASE_URL" if minimax_use_token_plan else "MINIMAX_BASE_URL",
            default_minimax_base_url,
        ).rstrip("/"),
        minimax_model=os.getenv("MINIMAX_MODEL", "MiniMax-M3"),
        minimax_timeout_seconds=int(os.getenv("MINIMAX_TIMEOUT_SECONDS", "300")),
        minimax_max_completion_tokens=int(os.getenv("MINIMAX_MAX_COMPLETION_TOKENS", "16000")),
        minimax_temperature=float(os.getenv("MINIMAX_TEMPERATURE", "0.2")),
        minimax_top_p=float(os.getenv("MINIMAX_TOP_P", "0.9")),
        minimax_use_token_plan=minimax_use_token_plan,
    )
