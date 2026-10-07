use brain_maintenance::hero_compare_render::{
    render_hero_compare, BoonValue, CompareBinding, CompareMetric, CompareSource, DisplayValue,
    HeroCompareInput, HeroCompareSeries, PublicationStatus, VersionBinding,
};
use scraper::{Html, Selector};

fn synthetic_input() -> HeroCompareInput {
    let binding = CompareBinding {
        result_id: "synthetic-structure-only".into(),
        version: VersionBinding {
            snapshot_id: "synthetic-snapshot".into(),
            client_version: "synthetic-client".into(),
        },
        conditions: vec!["Synthetischer Strukturtest, keine Spielaussage".into()],
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
            evidence: "Erfundene Testwerte, keine echte Quelle oder Veröffentlichungsfreigabe"
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

fn rejected(change: impl FnOnce(&mut HeroCompareInput)) {
    let mut input = synthetic_input();
    change(&mut input);
    assert!(render_hero_compare(&input).is_err());
}

fn selector(value: &str) -> Selector {
    Selector::parse(value).expect("gültiger Testselektor")
}

fn node_text(node: scraper::ElementRef<'_>) -> String {
    node.text().collect::<String>()
}

#[test]
fn synthetic_valid_input_renders_deterministically_for_every_metric() {
    for metric in [
        CompareMetric::BaseDps,
        CompareMetric::MagazineDamage,
        CompareMetric::Hp,
    ] {
        let mut input = synthetic_input();
        input.metric = metric;
        for hero in &mut input.heroes {
            hero.metric = metric;
        }
        let rendered = render_hero_compare(&input).unwrap();
        assert_eq!(rendered, render_hero_compare(&input).unwrap());
        assert!(!rendered.html.is_empty());
        assert!(!rendered.svg.is_empty());
    }
}

#[test]
fn nonpublic_input_heroes_and_sources_are_rejected() {
    for publication in [
        PublicationStatus::Pending,
        PublicationStatus::Private,
        PublicationStatus::Blocked,
    ] {
        rejected(|input| input.publication = publication);
        for index in 0..2 {
            rejected(|input| input.heroes[index].publication = publication);
        }
        rejected(|input| input.sources[0].publication = publication);
        rejected(|input| {
            let mut extra = input.sources[0].clone();
            extra.source_id = "unused-private-source".into();
            extra.publication = publication;
            input.sources.push(extra);
        });
    }
}

#[test]
fn mixed_result_version_conditions_and_metrics_are_rejected() {
    for index in 0..2 {
        rejected(|input| input.heroes[index].binding.result_id.push('x'));
        rejected(|input| input.heroes[index].binding.version.snapshot_id.push('x'));
        rejected(|input| input.heroes[index].binding.version.client_version.push('x'));
        rejected(|input| {
            input.heroes[index]
                .binding
                .conditions
                .push("anderes Szenario".into())
        });
        rejected(|input| input.heroes[index].metric = CompareMetric::Hp);
    }
    rejected(|input| input.sources[0].version.snapshot_id.push('x'));
    rejected(|input| input.sources[0].version.client_version.push('x'));
}

#[test]
fn missing_or_ambiguous_provenance_is_rejected() {
    rejected(|input| input.sources.clear());
    rejected(|input| input.sources[0].source_id.clear());
    rejected(|input| input.sources[0].evidence = " ".into());
    rejected(|input| input.sources.push(input.sources[0].clone()));
    rejected(|input| {
        let mut extra = input.sources[0].clone();
        extra.source_id = "unreferenced".into();
        input.sources.push(extra);
    });
    for index in 0..2 {
        rejected(|input| input.heroes[index].source_ids.clear());
        rejected(|input| input.heroes[index].source_ids = vec!["unknown".into()]);
        rejected(|input| {
            input.heroes[index]
                .source_ids
                .push("synthetic-source".into())
        });
    }
}

#[test]
fn incomplete_duplicate_or_unsorted_boon_rows_are_rejected() {
    rejected(|input| input.valid_boon_states.clear());
    rejected(|input| input.valid_boon_states.swap(0, 1));
    rejected(|input| input.valid_boon_states[1] = input.valid_boon_states[0]);
    for index in 0..2 {
        rejected(|input| {
            input.heroes[index].values.pop();
        });
        rejected(|input| {
            input.heroes[index].values.push(BoonValue {
                boon: 8,
                value: DisplayValue::Quantified(1.0),
            });
        });
        rejected(|input| input.heroes[index].values.swap(0, 1));
        rejected(|input| input.heroes[index].values[1].boon = 3);
        rejected(|input| input.heroes[index].values[1].boon = 0);
        rejected(|input| input.heroes[index].values[1].value = DisplayValue::Missing);
        rejected(|input| input.heroes[index].values[1].value = DisplayValue::Unquantified);
    }
}

#[test]
fn nonfinite_and_negative_values_are_rejected_at_every_point() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0] {
        for hero in 0..2 {
            for point in 0..3 {
                rejected(|input| {
                    input.heroes[hero].values[point].value = DisplayValue::Quantified(value);
                });
            }
        }
    }
}

#[test]
fn finite_extremes_zero_and_single_boon_have_finite_svg_geometry() {
    for value in [0.0, f64::from_bits(1), f64::MIN_POSITIVE, f64::MAX] {
        for single in [false, true] {
            let mut input = synthetic_input();
            if single {
                input.valid_boon_states = vec![u32::MAX];
                for hero in &mut input.heroes {
                    hero.values.truncate(1);
                    hero.values[0].boon = u32::MAX;
                }
            }
            for hero in &mut input.heroes {
                for point in &mut hero.values {
                    point.value = DisplayValue::Quantified(value);
                }
            }
            let rendered = render_hero_compare(&input).unwrap();
            let document = Html::parse_fragment(&rendered.svg);
            for node in document.select(&selector("*")) {
                for attribute in [
                    "x", "y", "cx", "cy", "x1", "x2", "y1", "y2", "width", "height",
                ] {
                    if let Some(raw) = node.value().attr(attribute) {
                        assert!(raw.parse::<f64>().unwrap().is_finite());
                    }
                }
                if let Some(points) = node.value().attr("points") {
                    for coordinate in points.split([' ', ',']) {
                        assert!(coordinate.parse::<f64>().unwrap().is_finite());
                    }
                }
                if let Some(path) = node.value().attr("d") {
                    for coordinate in path
                        .split_whitespace()
                        .filter(|part| *part != "M" && *part != "L")
                    {
                        assert!(coordinate.parse::<f64>().unwrap().is_finite());
                    }
                }
            }
        }
    }
}

#[test]
fn required_binding_text_and_distinct_heroes_are_enforced() {
    rejected(|input| input.binding.result_id.clear());
    rejected(|input| input.binding.version.snapshot_id.clear());
    rejected(|input| input.binding.version.client_version.clear());
    rejected(|input| input.binding.conditions.clear());
    rejected(|input| input.binding.conditions[0] = " ".into());
    rejected(|input| input.binding.conditions[0] = "line\ncontrol".into());
    rejected(|input| input.heroes[1].hero_id = input.heroes[0].hero_id.clone());
    rejected(|input| input.heroes[1].hero_name = input.heroes[0].hero_name.to_uppercase());
}

#[test]
fn untrusted_text_is_preserved_as_text_not_markup() {
    let payload = "<script>bad()</script>&\"'";
    let mut input = synthetic_input();
    input.binding.result_id = payload.into();
    input.binding.version.snapshot_id = payload.into();
    input.binding.version.client_version = payload.into();
    input.binding.conditions = vec![payload.into()];
    input.sources[0].source_id = payload.into();
    input.sources[0].evidence = payload.into();
    input.sources[0].version = input.binding.version.clone();
    for (index, hero) in input.heroes.iter_mut().enumerate() {
        hero.hero_id = format!("{index}{payload}");
        hero.hero_name = format!("{index}{payload}");
        hero.binding = input.binding.clone();
        hero.source_ids = vec![payload.into()];
    }
    let rendered = render_hero_compare(&input).unwrap();
    for markup in [&rendered.html, &rendered.svg] {
        assert!(!markup.contains(payload));
        let document = Html::parse_fragment(markup);
        assert_eq!(
            document
                .select(&selector("script, iframe, object, img"))
                .count(),
            0
        );
        assert!(document
            .root_element()
            .text()
            .collect::<String>()
            .contains(payload));
        for node in document.select(&selector("*")) {
            assert!(!node.value().attrs().any(|(name, _)| name.starts_with("on")));
        }
    }
}

#[test]
fn svg_and_accessible_table_share_exact_values_and_result_binding() {
    let input = synthetic_input();
    let rendered = render_hero_compare(&input).unwrap();
    let html = Html::parse_document(&rendered.html);
    let svg = Html::parse_fragment(&rendered.svg);
    let description = node_text(svg.select(&selector("desc")).next().unwrap());
    assert!(description.contains(&input.binding.result_id));
    let metadata = html
        .select(&selector("section li"))
        .map(node_text)
        .collect::<Vec<_>>()
        .join(" ");
    for binding in [
        &input.binding.result_id,
        &input.binding.version.snapshot_id,
        &input.binding.version.client_version,
    ] {
        assert!(description.contains(binding));
        assert!(metadata.contains(binding));
    }
    assert!(rendered.html.contains(&rendered.svg));
    let rows = html.select(&selector("tbody tr")).collect::<Vec<_>>();
    assert_eq!(rows.len(), input.valid_boon_states.len());
    for (index, row) in rows.iter().enumerate() {
        assert_eq!(
            node_text(row.select(&selector("th[scope='row']")).next().unwrap()),
            input.valid_boon_states[index].to_string()
        );
        let values = row
            .select(&selector("td"))
            .map(node_text)
            .collect::<Vec<_>>();
        assert_eq!(values.len(), 2);
        for (hero, actual) in input.heroes.iter().zip(values) {
            let DisplayValue::Quantified(value) = hero.values[index].value else {
                panic!("Testfixture muss beziffert sein");
            };
            assert_eq!(actual, value.to_string().replace('.', ","));
            assert!(description.contains(&actual));
            assert!(svg
                .select(&selector("circle title, rect title"))
                .any(|node| node_text(node).contains(&actual)));
        }
    }
}

#[test]
fn mobile_output_has_keyboard_regions_semantic_table_and_svg_description() {
    let rendered = render_hero_compare(&synthetic_input()).unwrap();
    let html = Html::parse_document(&rendered.html);
    assert_eq!(html.select(&selector("html[lang='de']")).count(), 1);
    let viewport = html
        .select(&selector("meta[name='viewport']"))
        .next()
        .unwrap();
    assert!(viewport
        .value()
        .attr("content")
        .unwrap()
        .contains("width=device-width"));
    assert_eq!(html.select(&selector("main h1")).count(), 1);
    assert_eq!(html.select(&selector("table caption")).count(), 1);
    assert_eq!(html.select(&selector("thead th[scope='col']")).count(), 3);
    assert_eq!(html.select(&selector("tbody th[scope='row']")).count(), 3);
    for container in [".chart", ".table"] {
        let node = html.select(&selector(container)).next().unwrap();
        assert_eq!(node.value().attr("tabindex"), Some("0"));
        assert_eq!(node.value().attr("role"), Some("region"));
        assert!(!node.value().attr("aria-label").unwrap().trim().is_empty());
    }
    let svg = html
        .select(&selector("svg[role='img'][viewBox]"))
        .next()
        .unwrap();
    for id in svg
        .value()
        .attr("aria-labelledby")
        .unwrap()
        .split_whitespace()
    {
        let target = html.select(&selector(&format!("#{id}"))).next().unwrap();
        assert!(!node_text(target).trim().is_empty());
    }
    assert!(html
        .select(&selector("polyline[stroke-dasharray]"))
        .next()
        .is_some());
}

#[test]
fn xml_forbidden_noncharacters_are_rejected_across_text_fields() {
    for character in ['\u{fffe}', '\u{ffff}'] {
        for field in 0..8 {
            let mut input = synthetic_input();
            match field {
                0 => input.binding.result_id.push(character),
                1 => input.binding.version.snapshot_id.push(character),
                2 => input.binding.version.client_version.push(character),
                3 => input.binding.conditions[0].push(character),
                4 => {
                    input.sources[0].source_id.push(character);
                    for hero in &mut input.heroes {
                        hero.source_ids[0] = input.sources[0].source_id.clone();
                    }
                }
                5 => input.sources[0].evidence.push(character),
                6 => input.heroes[0].hero_id.push(character),
                7 => input.heroes[0].hero_name.push(character),
                _ => unreachable!(),
            }
            input.sources[0].version = input.binding.version.clone();
            for hero in &mut input.heroes {
                hero.binding = input.binding.clone();
            }
            let error = render_hero_compare(&input).unwrap_err();
            assert!(error.to_string().contains("ungültig"), "{field}: {error}");
        }
        for index in 0..2 {
            rejected(|input| input.heroes[index].hero_id.push(character));
            rejected(|input| input.heroes[index].hero_name.push(character));
            rejected(|input| input.heroes[index].source_ids[0].push(character));
            rejected(|input| input.heroes[index].binding.result_id.push(character));
            rejected(|input| {
                input.heroes[index]
                    .binding
                    .version
                    .snapshot_id
                    .push(character)
            });
            rejected(|input| {
                input.heroes[index]
                    .binding
                    .version
                    .client_version
                    .push(character)
            });
            rejected(|input| input.heroes[index].binding.conditions[0].push(character));
        }
        rejected(|input| input.sources[0].version.snapshot_id.push(character));
        rejected(|input| input.sources[0].version.client_version.push(character));
    }
}

#[test]
fn complete_long_names_fit_svg_legend_and_end_label_bounds() {
    let original = render_hero_compare(&synthetic_input()).unwrap();
    let original_svg = Html::parse_fragment(&original.svg);
    let original_points = original_svg
        .select(&selector("polyline"))
        .map(|node| node.value().attr("points").unwrap().to_owned())
        .collect::<Vec<_>>();
    for name in [
        "W".repeat(40),
        "漢".repeat(40),
        format!("{}&<'\"", "Ä".repeat(36)),
    ] {
        let mut input = synthetic_input();
        input.heroes[0].hero_name = name.clone();
        input.heroes[1].hero_name = format!("B{}", name.chars().skip(1).collect::<String>());
        assert_eq!(input.heroes[0].hero_name.chars().count(), 40);
        let rendered = render_hero_compare(&input).unwrap();
        let svg = Html::parse_fragment(&rendered.svg);
        let root = svg.select(&selector("svg")).next().unwrap();
        let width = root.value().attr("width").unwrap().parse::<f64>().unwrap();
        assert!(width > 800.0);
        assert_eq!(
            root.value()
                .attr("viewBox")
                .unwrap()
                .split_whitespace()
                .nth(2)
                .unwrap()
                .parse::<f64>()
                .unwrap(),
            width
        );
        assert!(rendered.html.contains(&format!("min-width:{width}px")));
        let labels = svg
            .select(&selector("text[textLength]"))
            .collect::<Vec<_>>();
        assert_eq!(labels.len(), 4);
        for hero in &input.heroes {
            assert_eq!(
                labels
                    .iter()
                    .filter(|node| node_text(**node) == hero.hero_name)
                    .count(),
                2
            );
        }
        for node in &labels {
            let start = node.value().attr("x").unwrap().parse::<f64>().unwrap();
            let length = node
                .value()
                .attr("textLength")
                .unwrap()
                .parse::<f64>()
                .unwrap();
            assert_eq!(node.value().attr("lengthAdjust"), Some("spacingAndGlyphs"));
            assert_eq!(length, 360.0);
            assert!(start + length <= width - 36.0);
        }
        let legend_end = labels[0].value().attr("x").unwrap().parse::<f64>().unwrap() + 360.0;
        let second_start = labels[1].value().attr("x").unwrap().parse::<f64>().unwrap();
        assert!(legend_end + 42.0 <= second_start);
        assert_eq!(
            svg.select(&selector("polyline"))
                .map(|node| node.value().attr("points").unwrap().to_owned())
                .collect::<Vec<_>>(),
            original_points
        );
        let html = Html::parse_document(&rendered.html);
        assert_eq!(
            html.select(&selector("tbody"))
                .map(node_text)
                .collect::<Vec<_>>(),
            Html::parse_document(&original.html)
                .select(&selector("tbody"))
                .map(node_text)
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn full_thirty_six_point_input_is_preserved() {
    let mut input = synthetic_input();
    input.valid_boon_states = (0..=35).collect();
    for hero in &mut input.heroes {
        hero.values = input
            .valid_boon_states
            .iter()
            .map(|boon| BoonValue {
                boon: *boon,
                value: DisplayValue::Quantified(17.5),
            })
            .collect();
    }
    let rendered = render_hero_compare(&input).unwrap();
    let html = Html::parse_document(&rendered.html);
    let rows = html
        .select(&selector("tbody th[scope='row']"))
        .collect::<Vec<_>>();
    assert_eq!(rows.len(), input.valid_boon_states.len());
    for (row, boon) in rows.into_iter().zip(&input.valid_boon_states) {
        assert_eq!(node_text(row), boon.to_string());
    }
    let svg = Html::parse_fragment(&rendered.svg);
    for line in svg.select(&selector("polyline")) {
        assert_eq!(
            line.value()
                .attr("points")
                .unwrap()
                .split_whitespace()
                .count(),
            36
        );
    }
}
