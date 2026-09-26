fn main() {
    let strict = match std::env::args().nth(1).as_deref() {
        Some("--strict") => true,
        Some("--characterize") => false,
        _ => { eprintln!("expected --strict or --characterize"); std::process::exit(2); }
    };
    let mut same = 0;
    let mut differences = 0;
    let mut unexpected = 0;
    for &(id, input, expected, baseline, disposition) in CASES {
        let actual = normalize_alias(input);
        if actual == expected { same += 1; } else { differences += 1; }
        if actual != baseline { unexpected += 1; }
        println!("{id}\tlegacy={expected:?}\trust={actual:?}\t{disposition}");
    }
    // Exhaustive bounded corpus; no random generator or mutable clock inputs.
    // Every length-0..4 string over eight symbols, plus long/expanding cases.
    let alphabet = ['a', 'A', '_', ' ', '\t', 'ß', '\u{1c}', 'Σ'];
    let mut frontier = vec![String::new()];
    let mut property_cases = 0;
    for depth in 0..=4 {
        let mut next = Vec::new();
        for input in &frontier {
            let output = normalize_alias(input);
            assert_eq!(normalize_alias(&output), output, "not idempotent: {input:?}");
            assert!(!output.contains('_'), "underscore retained: {input:?}");
            assert_eq!(output.trim(), output, "edge whitespace: {input:?}");
            assert!(!output.contains("  "), "duplicate space: {input:?}");
            assert_eq!(normalize_alias(input), output, "not deterministic: {input:?}");
            property_cases += 1;
            if depth < 4 {
                for ch in alphabet { let mut text = input.clone(); text.push(ch); next.push(text); }
            }
        }
        frontier = next;
    }
    for input in ["A_".repeat(2048), "İ".repeat(1024), "___".into()] {
        let output = normalize_alias(&input);
        assert_eq!(normalize_alias(&output), output);
        property_cases += 1;
    }
    println!("SUMMARY parity_equal={same}/{} differences={differences} unexpected_baseline_changes={unexpected} property_cases={property_cases}", CASES.len());
    println!("SCOPE one pure function extracted unchanged; NOT whole-crate, DB, release or full-domain parity.");
    if unexpected != 0 || (strict && differences != 0) {
        eprintln!("PARITY NOT ACCEPTED: unresolved legacy/Rust differences require owner decision.");
        std::process::exit(1);
    }
    println!("CHARACTERIZATION PASSED; unresolved parity differences remain blockers.");
}
