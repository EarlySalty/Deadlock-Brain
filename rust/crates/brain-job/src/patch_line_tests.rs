use super::{parse_corrections, rows, Catalog, Row, IMPORT_COLUMNS};
use crate::patch_extract::{note_lines, subject_lines};
use std::collections::BTreeSet;

const POST: &str = "[ General ]
- Urn: Gold reward reduced from 100 to 80
- Fixed a bug with something
[ Items ]
- Extra Charge: Cooldown reduced from 10 to 8 - Bonus Charge: now 2
[ Heroes ]
- Abrams: Siphon Life damage increased from 30 to 40
Abrams
- Bullet damage increased from 5 to 6
- Guardians now take 10% more damage";

fn catalog() -> Catalog {
    Catalog::new(
        &[
            (1, "hero".to_owned(), Some("Abrams".to_owned())),
            (10, "ability".to_owned(), Some("Siphon Life".to_owned())),
            (20, "item".to_owned(), Some("Extra Charge".to_owned())),
        ],
        &[(1, "bull".to_owned())],
        &[(1, 10)],
    )
}

fn at(rows: &[Row], line_no: i32, piece: i32) -> &Row {
    rows.iter()
        .find(|row| row.line_no == line_no && row.piece == piece)
        .unwrap_or_else(|| panic!("Zeile {line_no}/{piece} fehlt"))
}

#[test]
fn keys_match_extractor_line_and_piece_indices() {
    let built = rows(POST, &catalog());
    let keys: BTreeSet<(usize, usize)> = built
        .iter()
        .map(|row| (row.line_no as usize, row.piece as usize))
        .collect();
    let extracted: Vec<(usize, usize)> = subject_lines(POST)
        .iter()
        .map(|line| (line.line, line.piece))
        .chain(note_lines(POST).iter().map(|note| (note.line, note.piece)))
        .collect();
    assert!(!extracted.is_empty());
    assert!(extracted.iter().all(|key| keys.contains(key)));
    assert_eq!(built.len(), 7);
}

#[test]
fn rows_follow_the_canonical_contract() {
    let built = rows(POST, &catalog());
    let urn = at(&built, 1, 0);
    assert_eq!(
        (
            urn.entity_type,
            urn.entity_name.as_deref(),
            urn.section.as_str()
        ),
        ("objective", Some("Urn"), "General")
    );
    assert_eq!(urn.change_type, Some("changed"));
    let bug = at(&built, 2, 0);
    assert_eq!(
        (bug.entity_type, bug.change_type),
        ("general", Some("bugfix"))
    );
    let item = at(&built, 4, 0);
    assert_eq!((item.entity_type, item.entity_id), ("item", Some(20)));
    assert_eq!(
        (
            item.stat_name.as_deref(),
            item.old_value.as_deref(),
            item.new_value.as_deref()
        ),
        (Some("Cooldown"), Some("10"), Some("8"))
    );
    assert_eq!(item.change_type, Some("buff"));
    let unknown = at(&built, 4, 1);
    assert_eq!(
        (unknown.entity_type, unknown.entity_id, unknown.needs_review),
        ("item", None, true)
    );
    let ability = at(&built, 6, 0);
    assert_eq!(
        (ability.entity_type, ability.entity_id, ability.ability_id),
        ("ability", Some(1), Some(10))
    );
    assert_eq!(ability.ability_name.as_deref(), Some("Siphon Life"));
    assert_eq!(ability.stat_name.as_deref(), Some("damage"));
    assert_eq!(ability.change_type, Some("buff"));
    let hero = at(&built, 8, 0);
    assert_eq!(
        (hero.entity_type, hero.entity_id, hero.section.as_str()),
        ("hero", Some(1), "Heroes/Abrams")
    );
    assert_eq!(hero.raw_line, "- Bullet damage increased from 5 to 6");
    assert!(built.iter().all(|row| row.line_no != 7));
}

#[test]
fn corrections_need_the_exact_header_and_reject_extract() {
    let header = IMPORT_COLUMNS.join("\t");
    let line =
        "5\t1\t0\tGeneral\tobjective\t\\N\tUrn\t\t\t\tnerf\t\t\t\t- Urn: x\tmanual\tFABLE-1\tfalse";
    let parsed = parse_corrections(&format!("{header}\n{line}\n")).map_err(|e| e.to_string());
    let parsed = parsed.unwrap_or_else(|error| panic!("{error}"));
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].entity_id, None);
    assert_eq!(parsed[0].change_type.as_deref(), Some("nerf"));
    assert!(parse_corrections(&format!(
        "{header}\n{}\n",
        line.replace("manual", "extract")
    ))
    .is_err());
    assert!(parse_corrections(&format!("{header}\n{line}\n{line}\n")).is_err());
    assert!(parse_corrections(&format!("ledger_id\n{line}\n")).is_err());
}
