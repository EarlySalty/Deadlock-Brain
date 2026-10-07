#[path = "../src/hero_compare_render.rs"]
pub mod hero_compare_render;

use hero_compare_render::{
    render_hero_compare, BoonValue, CompareBinding, CompareMetric, CompareSource, DisplayValue,
    HeroCompareInput, HeroCompareSeries, PublicationStatus, VersionBinding,
};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::time::Duration;

fn synthetic_input() -> HeroCompareInput {
    let binding = CompareBinding {
        result_id: "synthetic-preview-only".into(),
        version: VersionBinding {
            snapshot_id: "synthetic-snapshot".into(),
            client_version: "synthetic-client".into(),
        },
        conditions: vec![
            "Synthetische Darstellungsvorschau, keine Spielaussage oder echte G-Probe".into(),
            "Freigaben sind nur simuliert, keine Freigabe echter Quellen".into(),
        ],
    };
    let series = |id: &str, name: &str, values: [f64; 3]| HeroCompareSeries {
        hero_id: id.into(),
        hero_name: name.into(),
        publication: PublicationStatus::PublicApproved,
        binding: binding.clone(),
        metric: CompareMetric::BaseDps,
        source_ids: vec!["synthetic-source".into()],
        values: [0, 2, 5]
            .into_iter()
            .zip(values)
            .map(|(boon, value)| BoonValue {
                boon,
                value: DisplayValue::Quantified(value),
            })
            .collect(),
    };
    let heroes = [
        series("synthetic-a", "Testheld A", [11.25, 17.5, 23.75]),
        series("synthetic-b", "Testheld B", [12.5, 18.75, 21.25]),
    ];
    HeroCompareInput {
        sources: vec![CompareSource {
            source_id: "synthetic-source".into(),
            evidence: "Erfundene Vorschauwerte, keine echte Quelle oder Veröffentlichungsfreigabe"
                .into(),
            version: binding.version.clone(),
            publication: PublicationStatus::PublicApproved,
        }],
        binding,
        publication: PublicationStatus::PublicApproved,
        metric: CompareMetric::BaseDps,
        valid_boon_states: vec![0, 2, 5],
        heroes,
    }
}

fn serve(mut stream: TcpStream, html: &str, svg: &str) -> io::Result<bool> {
    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
    stream.set_write_timeout(Some(Duration::from_secs(2)))?;
    let mut first = Vec::new();
    BufReader::new((&mut stream).take(2048)).read_until(b'\n', &mut first)?;
    let first = std::str::from_utf8(&first)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    let (status, content_type, body, close) = match first.trim_end_matches(['\r', '\n']) {
        "GET / HTTP/1.1" | "GET / HTTP/1.0" => ("200 OK", "text/html; charset=utf-8", html, false),
        "GET /compare.svg HTTP/1.1" | "GET /compare.svg HTTP/1.0" => {
            ("200 OK", "image/svg+xml; charset=utf-8", svg, false)
        }
        "GET /close HTTP/1.1" | "GET /close HTTP/1.0" => (
            "200 OK",
            "text/plain; charset=utf-8",
            "Synthetische Vorschau beendet",
            true,
        ),
        _ => (
            "404 Not Found",
            "text/plain; charset=utf-8",
            "Keine Vorschauseite",
            false,
        ),
    };
    if let Err(error) = write!(stream, "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\nCache-Control: no-store\r\nContent-Security-Policy: default-src 'none'; style-src 'unsafe-inline'\r\n\r\n{body}", body.len()) {
        eprintln!("Vorschauantwort konnte nicht gesendet werden: {error}");
    }
    Ok(close)
}

fn main() -> anyhow::Result<()> {
    let rendered = render_hero_compare(&synthetic_input())?;
    let simulated_approval = "Öffentlich freigegeben (Vorschau: simuliert)";
    let svg = rendered
        .svg
        .replace("Öffentlich freigegeben", simulated_approval);
    let html = rendered
        .html
        .replace("Öffentlich freigegeben", simulated_approval)
        .replace(
            "<main>",
            "<main><p role=\"note\">Synthetische Darstellungsvorschau. Keine echten Spielwerte, keine Quellenfreigabe und kein Livebeweis.</p>",
        );
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
    let address = listener.local_addr()?;
    eprintln!(
        "Synthetische Darstellungsvorschau: http://{address}/; beenden mit GET /close oder Strg+C"
    );
    for connection in listener.incoming() {
        match serve(connection?, &html, &svg) {
            Ok(true) => break,
            Ok(false) => {}
            Err(error) => eprintln!("Vorschauverbindung fehlgeschlagen: {error}"),
        }
    }
    Ok(())
}
