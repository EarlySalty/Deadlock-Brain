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
