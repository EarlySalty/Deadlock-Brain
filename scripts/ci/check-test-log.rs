//! Stable libtest output verifier. No external crates or additional test runner.
use std::{collections::BTreeSet, env, fs};

fn verify(log: &str, allowed: &str, required: &str) -> Result<usize, String> {
    let expected: BTreeSet<&str> = allowed.lines().filter(|s| !s.is_empty()).collect();
    let mut ignored = BTreeSet::new();
    let mut passed = BTreeSet::new();
    let mut count = 0;
    for line in log.lines() {
        if let Some(rest) = line.strip_prefix("test ") {
            if let Some((name, result)) = rest.split_once(" ... ") {
                if result.starts_with("ignored") {
                    ignored.insert(name);
                } else if result == "ok" {
                    passed.insert(name.rsplit("::").next().unwrap_or(name));
                    count += 1;
                } else {
                    return Err(format!("unexpected test result: {line}"));
                }
            }
        }
        if line.contains("test result: FAILED") || line.to_lowercase().contains("skip:") {
            return Err(format!("failed or internally skipped test: {line}"));
        }
    }
    if count == 0 || ignored != expected {
        return Err(format!(
            "no tests or changed ignore inventory: passed={count}, unexpected={:?}, missing={:?}",
            ignored.difference(&expected).collect::<Vec<_>>(),
            expected.difference(&ignored).collect::<Vec<_>>()
        ));
    }
    for name in required.lines().filter(|s| !s.is_empty()) {
        if !passed.contains(name) {
            return Err(format!("required DB test did not pass: {name}"));
        }
    }
    Ok(count)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(
        args.len(),
        4,
        "usage: check-test-log LOG EXPECTED_IGNORES REQUIRED_DB_TESTS"
    );
    match verify(
        &fs::read_to_string(&args[1]).expect("test log"),
        &fs::read_to_string(&args[2]).expect("ignore inventory"),
        &fs::read_to_string(&args[3]).expect("required test inventory"),
    ) {
        Ok(count) => {
            println!("Verified {count} passing tests and the exact reviewed ignore inventory")
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::verify;
    const GOOD: &str =
        "test db::roundtrip ... ok\ntest corpus::historical ... ignored, explicit exception\n";
    #[test]
    fn accepts_complete_evidence() {
        assert_eq!(verify(GOOD, "corpus::historical", "roundtrip"), Ok(1));
    }
    #[test]
    fn rejects_zero_tests() {
        assert!(verify("running 0 tests", "", "").is_err());
    }
    #[test]
    fn rejects_missing_db_test() {
        assert!(verify(GOOD, "corpus::historical", "other").is_err());
    }
    #[test]
    fn rejects_unexpected_ignore() {
        assert!(verify(GOOD, "", "roundtrip").is_err());
    }
    #[test]
    fn rejects_disappeared_test() {
        assert!(verify("test roundtrip ... ok", "historical", "roundtrip").is_err());
    }
    #[test]
    fn rejects_internal_skip() {
        assert!(verify("skip: DB missing\ntest db ... ok", "", "").is_err());
    }
    #[test]
    fn rejects_failure() {
        assert!(verify("test db ... FAILED", "", "").is_err());
    }
}
