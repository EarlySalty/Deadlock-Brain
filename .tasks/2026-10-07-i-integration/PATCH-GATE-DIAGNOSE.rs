use super::*;

fn diagnostic_post(source: &str, link: &str) -> ApiPatchPost {
    ApiPatchPost {
        source: source.into(),
        title: "Minor Update".into(),
        pub_date: "2026-10-05T23:05:32Z".into(),
        link: link.into(),
        content: "- Weapon damage increased from 50 to 60".into(),
    }
}

#[test]
fn gate_diagnostic_selects_an_unrequested_steam_event() {
    let requested = "https://store.steampowered.com/news/app/1422450/view/703281025618282632";
    let post = diagnostic_post("steam", requested);
    let foreign_body = "[p]- Weapon damage increased from 50 to 60[/p]";
    let events = json!([{
        "gid":"703281025618282631",
        "event_name":"Minor Update",
        "rtime32_start_time":1791241532_i64,
        "announcement_body":{"headline":"Minor Update","body":foreign_body}
    }]);
    let encoded = events
        .to_string()
        .replace('&', "&amp;")
        .replace('"', "&quot;");
    let html = format!("<meta property=\"og:url\" content=\"{requested}\"><div data-partnereventstore=\"{encoded}\"></div>");
    let index = EntityIndex::default();
    let resolved = resolve_api_source(&post, &index, |_| Ok(html.clone()))
        .unwrap()
        .expect("Diagnose erwartet den beobachteten falschen Fallback");
    let prepared =
        prepare_api_patch(&post_row(&post, Some(17)).unwrap(), &resolved, &index).unwrap();
    assert_eq!(resolved.raw_content, foreign_body);
    assert_eq!(resolved.source_url.as_deref(), Some(requested));
    assert!(!prepared.events.is_empty());
    assert_eq!(prepared.events[0].metadata["url"], requested);
    println!("DIAGNOSE: angefragt=703281025618282632; einziges Ereignis=703281025618282631; fremder Inhalt unter angefragter URL vorbereitet=true");
}

#[test]
fn gate_diagnostic_fragments_pass_feed_validation_but_fail_real_http() {
    let cache = tempfile::tempdir().unwrap();
    let http = HttpClient::new("Deadlock-Brain-Gate-Diagnose/1", cache.path()).unwrap();
    for (source, link) in [
        (
            "steam",
            "https://store.steampowered.com/news/app/1422450/view/703281025618282632#notes",
        ),
        (
            "forum",
            "https://forums.playdeadlock.com/threads/update.75046/#post-1",
        ),
    ] {
        let post = diagnostic_post(source, link);
        validate_post(&post).unwrap();
        let error = resolve_api_source(&post, &EntityIndex::default(), |url| {
            let response = http.get_bounded(url, SourceHttpOptions::default())?;
            response.ensure_success()?;
            Ok(std::str::from_utf8(&response.content)?.to_owned())
        })
        .unwrap_err();
        assert!(error
            .to_string()
            .contains("without credentials or fragment"));
        println!("DIAGNOSE: Quelle={source}; Feed akzeptiert Fragment=true; echter HTTP-Guard liefert Fehler vor Abruf=true; Resolver liefert Err=true");
    }
}
