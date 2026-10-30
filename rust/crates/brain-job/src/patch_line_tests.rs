use super::{
    change_type, parse_corrections, plan, rows, statement_text, statements, Catalog, Key, Piece,
    Plan, Row, IMPORT_COLUMNS,
};
use std::collections::{BTreeMap, BTreeSet};

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
            (2, "hero".to_owned(), Some("Pocket".to_owned())),
            (3, "hero".to_owned(), None),
            (10, "ability".to_owned(), Some("Siphon Life".to_owned())),
            (11, "ability".to_owned(), Some("Barrage".to_owned())),
            (12, "ability".to_owned(), Some("Flying Cloak".to_owned())),
            (20, "item".to_owned(), Some("Extra Charge".to_owned())),
        ],
        &[(1, "bull".to_owned()), (3, "Wraithling".to_owned())],
        &[(1, 10), (2, 11), (2, 12)],
    )
}

fn at(rows: &[Row], line_no: i32, piece: i32) -> &Row {
    rows.iter()
        .find(|row| row.line_no == line_no && row.piece == piece)
        .unwrap_or_else(|| panic!("row {line_no}/{piece} missing"))
}

fn values(row: &Row) -> (Option<&str>, Option<&str>, Option<&str>) {
    (
        row.stat_name.as_deref(),
        row.old_value.as_deref(),
        row.new_value.as_deref(),
    )
}

#[test]
fn statements_index_every_piece_of_a_flattened_line() {
    let line = "[ General Changes ] - Added A - Removed B * Fixed C [ Heroes ] • Abrams: x -  - ";
    let pieces = statements(line);
    assert_eq!(
        pieces,
        [
            Piece::Section("General Changes"),
            Piece::Statement {
                text: "Added A",
                bulleted: true
            },
            Piece::Statement {
                text: "Removed B",
                bulleted: true
            },
            Piece::Statement {
                text: "Fixed C",
                bulleted: true
            },
            Piece::Section("Heroes"),
            Piece::Statement {
                text: "Abrams: x",
                bulleted: true
            },
        ]
    );
    let built = rows(line, &catalog());
    let keys: Vec<(i32, i32, &str)> = built
        .iter()
        .map(|row| (row.line_no, row.piece, row.raw_line.as_str()))
        .collect();
    assert_eq!(
        keys,
        [
            (0, 0, "Added A"),
            (0, 1, "Removed B"),
            (0, 2, "Fixed C"),
            (0, 3, "Abrams: x")
        ]
    );
    assert_eq!(statement_text("  - Fixed a crash "), "Fixed a crash");
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
    assert_eq!(values(item), (Some("Cooldown"), Some("10"), Some("8")));
    assert_eq!(item.change_type, Some("buff"));
    let unknown = at(&built, 4, 1);
    assert_eq!(
        (unknown.entity_type, unknown.entity_id, unknown.needs_review),
        ("item", None, true)
    );
    assert_eq!(unknown.entity_name.as_deref(), Some("Bonus Charge"));
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
    assert_eq!(hero.raw_line, "Bullet damage increased from 5 to 6");
    assert!(built.iter().all(|row| row.line_no != 7));
    assert!(built.iter().all(|row| row
        .entity_name
        .as_deref()
        .is_none_or(|name| !name.trim().is_empty())));
}

#[test]
fn polarity_follows_the_stat_in_both_directions() {
    let cases = [
        ("Cooldown reduction increased from 10% to 15%", "buff"),
        ("Cooldown reduction reduced from 15% to 10%", "nerf"),
        ("Cooldown increased from 10 to 12", "nerf"),
        ("Cooldown reduced from 12 to 10", "buff"),
        ("Damage taken reduction increased from 10% to 12%", "buff"),
        ("Damage taken reduction reduced from 12% to 10%", "nerf"),
        ("Bullet resist increased from 10% to 12%", "buff"),
        ("Bullet resist reduced from 12% to 10%", "nerf"),
        ("Reduced cooldown from 12 to 10", "buff"),
        ("Increased cooldown from 10 to 12", "nerf"),
        ("Reduced cost from 3000 to 2500", "buff"),
        ("Cost increased from 2500 to 3000", "nerf"),
        ("Damage increased from 30 to 40", "buff"),
        ("Damage reduced from 40 to 30", "nerf"),
    ];
    for (statement, expected) in cases {
        assert_eq!(
            change_type(statement, "item"),
            Some(expected),
            "{statement}"
        );
    }
}

#[test]
fn ability_binding_prefers_the_statement_start_and_flags_ambiguity() {
    let post = "[ Heroes ]
- Pocket: Barrage damage increased from 10 to 12
- Pocket: Barrage now also triggers Flying Cloak
- Pocket: Cooldown of Barrage and Flying Cloak reduced from 10 to 8
Pocket
- Flying Cloak duration increased from 2 to 3
- Shared cooldown between Barrage and Flying Cloak removed";
    let built = rows(post, &catalog());
    let direct = at(&built, 1, 0);
    assert_eq!(
        (direct.entity_type, direct.ability_id, direct.needs_review),
        ("ability", Some(11), false)
    );
    let leading = at(&built, 2, 0);
    assert_eq!(
        (leading.ability_id, leading.needs_review),
        (Some(11), false)
    );
    let ambiguous = at(&built, 3, 0);
    assert_eq!(
        (
            ambiguous.entity_type,
            ambiguous.entity_id,
            ambiguous.ability_id,
            ambiguous.needs_review
        ),
        ("hero", Some(2), None, true)
    );
    let headed = at(&built, 5, 0);
    assert_eq!((headed.ability_id, headed.needs_review), (Some(12), false));
    let headed_ambiguous = at(&built, 6, 0);
    assert_eq!(
        (
            headed_ambiguous.entity_type,
            headed_ambiguous.ability_id,
            headed_ambiguous.needs_review
        ),
        ("hero", None, true)
    );
}

#[test]
fn unknown_headings_and_missing_names_become_review_rows() {
    let post = "[ Heroes ]
Mystery
- Damage increased from 1 to 2
- Wraithling: Damage increased from 1 to 2
[ Items ]
- Unknown Gadget: Cooldown reduced from 10 to 8";
    let built = rows(post, &catalog());
    let mystery = at(&built, 2, 0);
    assert_eq!(
        (
            mystery.entity_type,
            mystery.entity_id,
            mystery.entity_name.as_deref(),
            mystery.needs_review
        ),
        ("hero", None, Some("Mystery"), true)
    );
    let aliased = at(&built, 3, 0);
    assert_eq!(
        (aliased.entity_id, aliased.entity_name.as_deref()),
        (Some(3), Some("Wraithling"))
    );
    let gadget = at(&built, 5, 0);
    assert_eq!(
        (gadget.entity_type, gadget.entity_id, gadget.needs_review),
        ("item", None, true)
    );
    assert!(built
        .iter()
        .all(|row| row.entity_name.as_deref() != Some("")));
}

#[test]
fn qualifiers_and_continuations_drop_unconditional_values() {
    let post = "[ Items ]
- Extra Charge: Damage increased from 10 to 20 - only against objectives
- Extra Charge: Cooldown reduced from 10 to 8
  - while airborne
- Extra Charge: Range increased from 5 to 6
(applies to troopers)
- Extra Charge: Charges increased from 1 to 2";
    let built = rows(post, &catalog());
    let inline = at(&built, 1, 0);
    assert_eq!(values(inline), (None, None, None));
    assert_eq!(inline.change_type, Some("buff"));
    let qualifier = at(&built, 1, 1);
    assert_eq!(
        (
            qualifier.entity_id,
            qualifier.raw_line.as_str(),
            qualifier.old_value.as_deref()
        ),
        (Some(20), "only against objectives", None)
    );
    assert_eq!(values(at(&built, 2, 0)), (None, None, None));
    assert_eq!(at(&built, 3, 0).raw_line, "while airborne");
    assert_eq!(values(at(&built, 4, 0)), (None, None, None));
    assert_eq!(at(&built, 5, 0).raw_line, "(applies to troopers)");
    assert_eq!(
        values(at(&built, 6, 0)),
        (Some("Charges"), Some("1"), Some("2"))
    );
}

fn key(ledger: i64, line: i32) -> Key {
    (ledger, line, 0)
}

#[test]
fn plan_remaps_archives_and_keeps_corrections() {
    let held = vec![
        (key(1, 1), "Same".to_owned()),
        (key(1, 2), "Old text".to_owned()),
        (key(1, 3), "Gone".to_owned()),
        (key(5, 4), "Moved".to_owned()),
        (key(6, 1), "Twice".to_owned()),
        (key(7, 1), "Nowhere".to_owned()),
    ];
    let ledgers: BTreeSet<i64> = [1, 9].into_iter().collect();
    let groups: BTreeMap<i64, i64> = [(5, 9), (6, 9)].into_iter().collect();
    let fresh: BTreeMap<Key, String> = [
        (key(1, 1), "Same"),
        (key(1, 2), "New text"),
        (key(9, 7), "Moved"),
        (key(9, 8), "Twice"),
        (key(9, 9), "Twice"),
    ]
    .into_iter()
    .map(|(key, text)| (key, text.to_owned()))
    .collect();
    assert_eq!(
        plan(&held, &ledgers, &groups, &fresh),
        Plan {
            remap: vec![(key(5, 4), key(9, 7))],
            archive: vec![
                (key(1, 2), "raw_line_changed"),
                (key(1, 3), "key_vanished"),
                (key(6, 1), "ledger_vanished"),
                (key(7, 1), "ledger_vanished"),
            ],
        }
    );
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
    assert_eq!(parsed[0].raw_line, "Urn: x");
    assert!(parse_corrections(&format!(
        "{header}\n{}\n",
        line.replace("manual", "extract")
    ))
    .is_err());
    assert!(parse_corrections(&format!("{header}\n{line}\n{line}\n")).is_err());
    assert!(parse_corrections(&format!("ledger_id\n{line}\n")).is_err());
}
