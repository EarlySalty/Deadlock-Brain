from unittest.mock import patch

import pytest
from defusedxml.common import EntitiesForbidden

from deadlock_brain.http import HttpClient
from deadlock_brain.youtube_learning import _parse_atom_videos


def test_atom_entity_expansion_is_rejected():
    xml = '<!DOCTYPE feed [<!ENTITY data "expanded">]><feed>&data;</feed>'
    with pytest.raises(EntitiesForbidden):
        _parse_atom_videos(xml, "ci", "https://example.invalid/feed")


def test_empty_atom_feed_is_supported():
    assert _parse_atom_videos('<feed xmlns="http://www.w3.org/2005/Atom"/>', "ci", "https://example.invalid/feed") == []


@pytest.mark.parametrize("url", ["file:///ci-probe", "ftp://example.invalid/probe"])
def test_non_http_urls_are_rejected_before_network_or_file_access(tmp_path, url):
    client = HttpClient(user_agent="ci", cache_dir=tmp_path)
    with patch("urllib.request.urlopen") as opener:
        with pytest.raises(ValueError, match="Only HTTP"):
            client.get(url)
        opener.assert_not_called()


@pytest.mark.parametrize("classifier_module", ["deadlock_brain.sources.patchnotes_db", "deadlock_brain.patch_parser"])
@pytest.mark.parametrize(("url", "expected"), [
    ("https://steamcommunity.com/app/1422450", "steam"),
    ("https://store.steampowered.com/news", "steam"),
    ("https://STEAMCOMMUNITY.COM./news", "steam"),
    ("https://steamstore-a.akamaihd.net/news", "steam"),
    ("https://forums.playdeadlock.com/threads/ci", "forum"),
    ("https://example.invalid/steamcommunity.com", "other"),
    ("https://steamcommunity.com.example.invalid/news", "other"),
    ("https://notsteamcommunity.com/news", "other"),
    ("https://example.invalid/?url=forums.playdeadlock.com", "other"),
    ("https://steamcommunity.com@example.invalid/news", "other"),
    ("https://example.invalid@steamcommunity.com/news", "other"),
    ("file://steamcommunity.com/news", "other"),
    ("https://[invalid", "other"),
    (None, "other"),
])
def test_source_classification_uses_hostname_boundaries(classifier_module, url, expected):
    import importlib
    classifier = importlib.import_module(classifier_module).classify_source_kind
    assert classifier(url) == expected
