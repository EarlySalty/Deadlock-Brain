"""Fail closed: collecting no tests or skipping a test is not CI evidence."""
import pytest


def pytest_sessionfinish(session, exitstatus):
    reporter = session.config.pluginmanager.get_plugin("terminalreporter")
    skipped = reporter.stats.get("skipped", []) if reporter is not None else []
    if session.testscollected == 0 or skipped:
        session.exitstatus = pytest.ExitCode.TESTS_FAILED
