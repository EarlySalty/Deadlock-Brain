use brain_job::{mechanics, registry::EXTRACTORS, Outcome, RawObject};
use brain_storage::gamefacts::receipts;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[test]
#[ignore = "requires the read-only deadlock-data cache in BRAIN_MECHANICS_REPOSITORY"]
fn cached_deadlock_data_yields_every_mechanic_with_verified_receipts() {
    let repository = std::path::PathBuf::from(
        std::env::var_os("BRAIN_MECHANICS_REPOSITORY").expect("cache path required"),
    );
    let mut aliases = BTreeSet::new();
    let mut paths = BTreeSet::new();
    for object in brain_job::sources::SOURCES[0].1(&repository, false) {
        let object = object.unwrap();
        let raw = RawObject {
            id: 1,
            source: "deadlock-data".into(),
            revision: object.revision,
            path: object.path,
            content_sha256: Sha256::digest(&object.bytes).into(),
            bytes: object.bytes,
        };
        let Some(extractor) = EXTRACTORS
            .iter()
            .find(|extractor| extractor.accepts(&raw.source, &raw.path))
        else {
            continue;
        };
        let Outcome::Accepted(set) = extractor.extract(&raw) else {
            panic!("{} was not accepted", raw.path);
        };
        receipts::verify(&raw.bytes, &set).unwrap();
        paths.insert(raw.path.clone());
        aliases.extend(set.aliases.into_iter().map(|alias| alias.alias));
    }
    assert!(mechanics::PATHS.iter().all(|path| paths.contains(*path)));
    let expected: BTreeSet<String> = [
        "lane_troopers",
        "soul_sharing",
        "denizens",
        "objectives",
        "mid_boss",
        "rejuvenator",
        "hero_kills",
        "respawn",
        "urn",
        "breakables",
        "sinners_sacrifice",
        "item_tiers",
        "levels",
        "street_brawl",
    ]
    .into_iter()
    .map(mechanics::alias)
    .collect();
    let found: BTreeSet<String> = aliases
        .into_iter()
        .filter(|alias| alias.starts_with("mechanic_"))
        .collect();
    assert_eq!(found, expected);
}
