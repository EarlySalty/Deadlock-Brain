use anyhow::{ensure, Result};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareMetric {
    BaseDps,
    MagazineDamage,
    Hp,
}

impl CompareMetric {
    pub fn label(self) -> &'static str {
        match self {
            Self::BaseDps => "Grund-DPS",
            Self::MagazineDamage => "Magazinschaden",
            Self::Hp => "HP",
        }
    }

    pub fn unit(self) -> &'static str {
        match self {
            Self::BaseDps => "Schaden/s",
            Self::MagazineDamage => "Schaden",
            Self::Hp => "HP",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationStatus {
    PublicApproved,
    Pending,
    Private,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionBinding {
    pub snapshot_id: String,
    pub client_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompareBinding {
    pub result_id: String,
    pub version: VersionBinding,
    pub conditions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompareSource {
    pub source_id: String,
    pub evidence: String,
    pub version: VersionBinding,
    pub publication: PublicationStatus,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayValue {
    Quantified(f64),
    Missing,
    Unquantified,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoonValue {
    pub boon: u32,
    pub value: DisplayValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HeroCompareSeries {
    pub hero_id: String,
    pub hero_name: String,
    pub publication: PublicationStatus,
    pub binding: CompareBinding,
    pub metric: CompareMetric,
    pub source_ids: Vec<String>,
    pub values: Vec<BoonValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HeroCompareInput {
    pub binding: CompareBinding,
    pub publication: PublicationStatus,
    pub metric: CompareMetric,
    pub valid_boon_states: Vec<u32>,
    pub sources: Vec<CompareSource>,
    pub heroes: [HeroCompareSeries; 2],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedHeroCompare {
    pub html: String,
    pub svg: String,
}

fn text(value: &str, maximum: usize) -> bool {
    !value.trim().is_empty()
        && value.chars().count() <= maximum
        && !value
            .chars()
            .any(|character| character.is_control() || matches!(character, '\u{fffe}' | '\u{ffff}'))
}

fn escape(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#39;"),
            _ => output.push(character),
        }
    }
    output
}

fn validate(input: &HeroCompareInput) -> Result<()> {
    ensure!(
        input.publication == PublicationStatus::PublicApproved,
        "Vergleich ist nicht öffentlich freigegeben"
    );
    ensure!(
        text(&input.binding.result_id, 160),
        "Ergebnis-ID fehlt oder ist ungültig"
    );
    ensure!(
        text(&input.binding.version.snapshot_id, 160)
            && text(&input.binding.version.client_version, 160),
        "Datenstand fehlt oder ist ungültig"
    );
    ensure!(
        !input.binding.conditions.is_empty()
            && input.binding.conditions.len() <= 16
            && input
                .binding
                .conditions
                .iter()
                .all(|value| text(value, 300)),
        "Bedingungen fehlen oder sind ungültig"
    );
    ensure!(
        !input.valid_boon_states.is_empty()
            && input
                .valid_boon_states
                .windows(2)
                .all(|pair| pair[0] < pair[1]),
        "Gültige Boonstände fehlen oder sind nicht eindeutig aufsteigend"
    );
    ensure!(
        !input.sources.is_empty() && input.sources.len() <= 32,
        "Quellen fehlen oder sind zu zahlreich"
    );
    let mut sources = BTreeSet::new();
    for source in &input.sources {
        ensure!(
            text(&source.source_id, 160) && text(&source.evidence, 1000),
            "Quellenbeleg fehlt oder ist ungültig"
        );
        ensure!(
            sources.insert(source.source_id.as_str()),
            "Quellen-ID ist doppelt"
        );
        ensure!(
            source.publication == PublicationStatus::PublicApproved,
            "Quelle ist nicht öffentlich freigegeben"
        );
        ensure!(
            source.version == input.binding.version,
            "Quelle gehört zu einem anderen Datenstand"
        );
    }
    ensure!(
        input.heroes[0].hero_id.trim() != input.heroes[1].hero_id.trim()
            && input.heroes[0].hero_name.trim().to_lowercase()
                != input.heroes[1].hero_name.trim().to_lowercase(),
        "Zwei verschiedene Helden sind erforderlich"
    );
    let mut used_sources = BTreeSet::new();
    for hero in &input.heroes {
        ensure!(
            text(&hero.hero_id, 160) && text(&hero.hero_name, 40),
            "Heldenname oder ID fehlt oder ist ungültig"
        );
        ensure!(
            hero.publication == PublicationStatus::PublicApproved,
            "Held ist nicht öffentlich freigegeben"
        );
        ensure!(hero.binding == input.binding && hero.metric == input.metric, "Heldenreihen haben unterschiedliche Ergebnisse, Bedingungen, Kennzahlen oder Datenstände");
        ensure!(
            !hero.source_ids.is_empty() && hero.source_ids.len() <= 32,
            "Quellenbindung der Heldenreihe fehlt"
        );
        let mut unique = BTreeSet::new();
        for source_id in &hero.source_ids {
            ensure!(
                sources.contains(source_id.as_str()) && unique.insert(source_id.as_str()),
                "Quellenbindung ist unbekannt oder doppelt"
            );
            used_sources.insert(source_id.as_str());
        }
        ensure!(
            hero.values.len() == input.valid_boon_states.len(),
            "Heldenreihen sind unvollständig"
        );
        for (point, boon) in hero.values.iter().zip(&input.valid_boon_states) {
            ensure!(
                point.boon == *boon,
                "Heldenreihen haben abweichende oder ungültige Boonstände"
            );
            match point.value {
                DisplayValue::Quantified(value) => ensure!(
                    value.is_finite() && value >= 0.0,
                    "Wert ist nicht endlich oder negativ"
                ),
                DisplayValue::Missing | DisplayValue::Unquantified => {
                    anyhow::bail!("Kennzahl fehlt oder ist nicht beziffert")
                }
            }
        }
    }
    ensure!(
        used_sources == sources,
        "Eine Quelle ist keiner Heldenreihe zugeordnet"
    );
    Ok(())
}

fn number(value: f64) -> String {
    value.to_string().replace('.', ",")
}

fn chart_number(value: f64) -> String {
    if value != 0.0 && !(0.001..10_000_000.0).contains(&value) {
        format!("{value:.3e}").replace('.', ",")
    } else {
        format!("{value:.3}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .replace('.', ",")
    }
}

fn quantified(point: &BoonValue) -> f64 {
    match point.value {
        DisplayValue::Quantified(value) => value,
        DisplayValue::Missing | DisplayValue::Unquantified => unreachable!(),
    }
}

fn metadata(input: &HeroCompareInput) -> Vec<String> {
    let mut lines = vec![
        format!("Ergebnis: {}", input.binding.result_id),
        format!("Snapshot: {}", input.binding.version.snapshot_id),
        format!("Clientversion: {}", input.binding.version.client_version),
        "Öffentlich freigegeben".to_owned(),
    ];
    for condition in &input.binding.conditions {
        lines.push(format!("Bedingung: {condition}"));
    }
    for hero in &input.heroes {
        lines.push(format!(
            "{}: Quellen {}",
            hero.hero_name,
            hero.source_ids.join(", ")
        ));
    }
    for source in &input.sources {
        lines.push(format!("Quelle {}: {}", source.source_id, source.evidence));
    }
    lines
}

fn wrap(value: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    let mut length = 0;
    for word in value.split_whitespace() {
        let characters: Vec<char> = word.chars().collect();
        for chunk in characters.chunks(82) {
            let separator = usize::from(!line.is_empty());
            if length + separator + chunk.len() > 82 {
                lines.push(std::mem::take(&mut line));
                length = 0;
            }
            if !line.is_empty() {
                line.push(' ');
                length += 1;
            }
            line.extend(chunk);
            length += chunk.len();
        }
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

fn name_width(value: &str) -> usize {
    value.chars().count() * 9
}

fn svg_name(value: &str, x: usize, y: f64) -> String {
    format!(
        r##"<text x="{x}" y="{y:.2}" font-family="monospace" textLength="{}" lengthAdjust="spacingAndGlyphs" xml:space="preserve">{}</text>"##,
        name_width(value),
        escape(value)
    )
}

pub fn render_hero_compare(input: &HeroCompareInput) -> Result<RenderedHeroCompare> {
    validate(input)?;
    let title = format!(
        "{} und {}: {}",
        input.heroes[0].hero_name,
        input.heroes[1].hero_name,
        input.metric.label()
    );
    let meta = metadata(input);
    let footer: Vec<String> = meta.iter().flat_map(|line| wrap(line)).collect();
    let height = 530 + footer.len() * 20;
    let width = (666
        + input
            .heroes
            .iter()
            .map(|hero| name_width(&hero.hero_name))
            .max()
            .unwrap())
    .max(800);
    let scroll_hint = if width > 960 { "block" } else { "none" };
    let maximum = input
        .heroes
        .iter()
        .flat_map(|hero| &hero.values)
        .map(quantified)
        .fold(0.0_f64, f64::max);
    let scale = if maximum == 0.0 { 1.0 } else { maximum };
    let first = input.valid_boon_states[0];
    let last = input.valid_boon_states[input.valid_boon_states.len() - 1];
    let x = |boon: u32| {
        if first == last {
            350.0
        } else {
            100.0 + 500.0 * f64::from(boon - first) / f64::from(last - first)
        }
    };
    let y = |value: f64| 390.0 - 260.0 * (value / scale);
    let mut accessible = format!(
        "{title}. {} pro Boonstand. {}.",
        input.metric.unit(),
        meta.join(". ")
    );
    for (index, boon) in input.valid_boon_states.iter().enumerate() {
        accessible.push_str(&format!(
            " Boon {boon}: {} {}, {} {}.",
            input.heroes[0].hero_name,
            number(quantified(&input.heroes[0].values[index])),
            input.heroes[1].hero_name,
            number(quantified(&input.heroes[1].values[index]))
        ));
    }
    let mut svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" lang="de" role="img" aria-labelledby="compare-title compare-desc" viewBox="0 0 {width} {height}" width="{width}" height="{height}"><title id="compare-title">{}</title><desc id="compare-desc">{}</desc><rect width="{width}" height="{height}" fill="#111110"/><g font-family="system-ui,sans-serif" fill="#f3efe5" font-size="14"><text x="36" y="38" font-size="22">Heldenvergleich</text><text x="36" y="65">{} · {}</text>"##,
        escape(&title),
        escape(&accessible),
        input.metric.label(),
        input.metric.unit()
    );
    for (index, hero) in input.heroes.iter().enumerate() {
        let lx = 36 + index * ((width - 72) / 2);
        let dash = if index == 1 {
            " stroke-dasharray=\"8 5\""
        } else {
            ""
        };
        svg.push_str(&format!(
            r##"<line x1="{lx}" y1="91" x2="{}" y2="91" stroke="{}" stroke-width="2"{dash}/>{}"##,
            lx + 30,
            color(index),
            svg_name(&hero.hero_name, lx + 42, 96.0)
        ));
    }
    for tick in 0..=4 {
        let value = scale * (f64::from(tick) / 4.0);
        let py = y(value);
        svg.push_str(&format!(r##"<line x1="100" y1="{py:.2}" x2="600" y2="{py:.2}" stroke="#3b3932"/><text x="90" y="{:.2}" text-anchor="end">{}</text>"##, py + 5.0, chart_number(value)));
    }
    let mut last_label_x = -100.0;
    for boon in &input.valid_boon_states {
        let px = x(*boon);
        if *boon == last || (px - last_label_x >= 60.0 && x(last) - px >= 60.0) {
            last_label_x = px;
            svg.push_str(&format!(
                r##"<text x="{:.2}" y="418" text-anchor="middle">{boon}</text>"##,
                x(*boon)
            ));
        }
    }
    svg.push_str(r##"<text x="350" y="445" text-anchor="middle">Boonstand</text>"##);
    let mut ends = [
        y(quantified(
            &input.heroes[0].values[input.heroes[0].values.len() - 1],
        )),
        y(quantified(
            &input.heroes[1].values[input.heroes[1].values.len() - 1],
        )),
    ];
    if (ends[0] - ends[1]).abs() < 42.0 {
        let middle = ((ends[0] + ends[1]) / 2.0).clamp(151.0, 369.0);
        if ends[0] <= ends[1] {
            ends = [middle - 21.0, middle + 21.0];
        } else {
            ends = [middle + 21.0, middle - 21.0];
        }
    }
    for (index, hero) in input.heroes.iter().enumerate() {
        let dash = if index == 1 {
            " stroke-dasharray=\"8 5\""
        } else {
            ""
        };
        let points = hero
            .values
            .iter()
            .map(|point| format!("{:.2},{:.2}", x(point.boon), y(quantified(point))))
            .collect::<Vec<_>>()
            .join(" ");
        svg.push_str(&format!(
            r##"<polyline points="{points}" fill="none" stroke="{}" stroke-width="2"{dash}/>"##,
            color(index)
        ));
        for point in &hero.values {
            let px = x(point.boon);
            let py = y(quantified(point));
            let hint = escape(&format!(
                "{} · Boon {}: {} {}",
                hero.hero_name,
                point.boon,
                number(quantified(point)),
                input.metric.unit()
            ));
            if index == 0 {
                svg.push_str(&format!(r##"<circle cx="{px:.2}" cy="{py:.2}" r="5" fill="{}" stroke="#111110" stroke-width="2"><title>{hint}</title></circle>"##, color(index)));
            } else {
                svg.push_str(&format!(r##"<rect x="{:.2}" y="{:.2}" width="10" height="10" fill="{}" stroke="#111110" stroke-width="2"><title>{hint}</title></rect>"##, px - 5.0, py - 5.0, color(index)));
            }
            svg.push_str(&format!(r##"<circle cx="{px:.2}" cy="{py:.2}" r="12" fill="transparent"><title>{hint}</title></circle>"##));
        }
        let end = &hero.values[hero.values.len() - 1];
        let label_y = ends[index];
        svg.push_str(&format!(r##"<path d="M {:.2} {:.2} L 620 {label_y:.2}" fill="none" stroke="{}"{dash}/>{}<text x="630" y="{:.2}">{}</text>"##, x(end.boon), y(quantified(end)), color(index), svg_name(&hero.hero_name, 630, label_y), label_y + 17.0, chart_number(quantified(end))));
    }
    for (index, line) in footer.iter().enumerate() {
        svg.push_str(&format!(
            r##"<text x="36" y="{}" font-size="14" font-family="monospace" textLength="{}" lengthAdjust="spacingAndGlyphs" xml:space="preserve">{}</text>"##,
            505 + index * 20,
            name_width(line).min(width - 72),
            escape(line)
        ));
    }
    svg.push_str("</g></svg>");
    let mut html = format!(
        r##"<!doctype html><html lang="de"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>{}</title><style>body{{margin:0;background:#111110;color:#f3efe5;font:16px/1.6 system-ui,sans-serif}}main{{max-width:960px;margin:auto;padding:clamp(16px,4vw,48px)}}h1{{font-size:clamp(24px,5vw,40px);line-height:1.2;overflow-wrap:anywhere}}h2{{font-size:22px;margin-top:32px}}p,li,dd,th,td{{overflow-wrap:anywhere}}figure{{margin:24px 0}}svg{{display:block;width:100%;height:auto}}.chart{{overflow-x:auto}}.chart svg{{min-width:{width}px}}.scroll-hint{{display:{scroll_hint};color:#c9c3b4;margin:0 0 8px}}@media(max-width:895px){{.scroll-hint{{display:block}}}}figcaption{{color:#c9c3b4;margin-top:8px}}.table{{overflow-x:auto}}table{{border-collapse:collapse;width:100%;font-variant-numeric:tabular-nums}}caption{{text-align:left;font-weight:600}}th,td{{padding:12px;text-align:right;border-bottom:1px solid #3b3932}}th:first-child{{text-align:left}}dt{{color:#c9c3b4}}dd{{margin:0 0 12px}}.legend{{display:flex;flex-wrap:wrap;gap:12px 32px;list-style:none;padding:0}}.legend span{{display:inline-block;width:28px;margin-right:8px;border-top:2px solid #b08b20;vertical-align:middle}}.legend .second{{border-top:2px dashed #995108}}@media(forced-colors:active){{.legend span{{border-color:CanvasText}}}}@media print{{body{{background:white;color:black}}}}</style></head><body><main><h1>{}</h1><p>{} in {} bei gleichen Boonständen.</p><ul class="legend" aria-label="Helden"><li><span aria-hidden="true"></span>{} (durchgezogen)</li><li><span class="second" aria-hidden="true"></span>{} (gestrichelt)</li></ul><figure><p class="scroll-hint">Grafik seitlich verschieben</p><div class="chart" tabindex="0" role="region" aria-label="Vergleichsgrafik, seitlich verschiebbar">{svg}</div><figcaption>Gemeinsame Skala ab null. Werte und Bedingungen stehen unter der Grafik.</figcaption></figure><section aria-labelledby="values"><h2 id="values">Werte</h2><div class="table" tabindex="0" role="region" aria-label="Wertetabelle, seitlich verschiebbar"><table><caption>{} ({})</caption><thead><tr><th scope="col">Boonstand</th><th scope="col">{}</th><th scope="col">{}</th></tr></thead><tbody>"##,
        escape(&title),
        escape(&title),
        input.metric.label(),
        input.metric.unit(),
        escape(&input.heroes[0].hero_name),
        escape(&input.heroes[1].hero_name),
        input.metric.label(),
        input.metric.unit(),
        escape(&input.heroes[0].hero_name),
        escape(&input.heroes[1].hero_name)
    );
    for (index, boon) in input.valid_boon_states.iter().enumerate() {
        html.push_str(&format!(
            "<tr><th scope=\"row\">{boon}</th><td>{}</td><td>{}</td></tr>",
            number(quantified(&input.heroes[0].values[index])),
            number(quantified(&input.heroes[1].values[index]))
        ));
    }
    html.push_str("</tbody></table></div></section><section aria-labelledby=\"context\"><h2 id=\"context\">Stand und Bedingungen</h2><ul>");
    for line in &meta {
        html.push_str(&format!("<li>{}</li>", escape(line)));
    }
    html.push_str("</ul></section></main></body></html>");
    Ok(RenderedHeroCompare { html, svg })
}

fn color(index: usize) -> &'static str {
    if index == 0 {
        "#b08b20"
    } else {
        "#995108"
    }
}
