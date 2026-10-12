use brain_job::{extract, git::Source};
use std::path::PathBuf;

#[test]
#[ignore]
fn immutable_public_documents_prove_baba_without_a_profile_snapshot() {
    let root = PathBuf::from(
        std::env::var_os("BRAIN_JOB_PUBLIC_FIXTURES").expect("public fixture directory"),
    );
    for (source, local, path) in [
        (Source::ALL[0], "hero-data.json", "data/json/hero-data.json"),
        (
            Source::ALL[0],
            "ability-data.json",
            "data/json/ability-data.json",
        ),
        (
            Source::ALL[1],
            "heroes.vdata",
            "game/citadel/pak01_dir/scripts/heroes.vdata",
        ),
        (
            Source::ALL[1],
            "abilities.vdata",
            "game/citadel/pak01_dir/scripts/abilities.vdata",
        ),
    ] {
        let bytes = std::fs::read(root.join(local)).unwrap();
        let assertions = extract::extract(source, path, &bytes).expect(path);
        extract::prove(source, path, &bytes, &assertions).expect(path);
        println!(
            "source={} path={} bytes={} assertions={}",
            source.key,
            path,
            bytes.len(),
            assertions.len()
        );
        if local.ends_with(".json") {
            let document: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            let records = document.as_object().unwrap();
            for (key, record) in records {
                let Some(name) = record["Name"].as_str() else {
                    assert!(!assertions
                        .iter()
                        .any(|a| a.entity == *key && a.stat == "identity.name"));
                    continue;
                };
                assert!(
                    assertions.iter().any(|a| a.entity == *key
                        && a.stat == "identity.name"
                        && a.locator == format!("/{key}/Name")
                        && matches!(&a.value, extract::Atom::Text(value) if value == name)),
                    "missing identity for {key}"
                );
            }
            if local == "hero-data.json" {
                let abilities: serde_json::Value =
                    serde_json::from_slice(&std::fs::read(root.join("ability-data.json")).unwrap())
                        .unwrap();
                for assertion in &assertions {
                    if let Some((target, _)) = &assertion.relation {
                        assert!(abilities.get(target).is_some(), "missing target {target}");
                    }
                }
            }
            println!("verified_identities={}", records.len());
        }
        if local == "hero-data.json" {
            assert_eq!(
                assertions
                    .iter()
                    .filter(|a| a.entity == "hero_baba" && a.relation.is_some())
                    .count(),
                4
            );
            assert!(assertions.iter().any(|a| a.entity == "hero_baba"
                && a.stat == "stats.max_health"
                && matches!(&a.value, extract::Atom::Number(n) if n == "780.0")));
        } else if local == "heroes.vdata" {
            assert!(assertions.iter().any(|a| a.entity == "hero_baba"
                && a.stat == "identity.status"
                && matches!(&a.value, extract::Atom::Text(s) if s == "released")));
        } else {
            assert!(assertions
                .iter()
                .any(|a| a.entity == "ability_baba_bubbling_brew"
                    && a.stat == "ability.AbilityCooldown"
                    && matches!(&a.value, extract::Atom::Number(n) if n == "26.0")));
        }
    }
}
