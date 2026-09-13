use std::{fs, path::PathBuf, process::Command};

fn git(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn main() {
    // Re-evaluate on every Cargo invocation, including worktree HEAD/ref changes
    // and dirty dependency sources outside this crate. No runtime configuration.
    println!("cargo:rerun-if-changed=build-provenance-always-recheck");
    let revision = git(&["rev-parse", "HEAD"]).unwrap_or_default();
    let clean = git(&["status", "--porcelain", "--untracked-files=normal"])
        .is_some_and(|status| status.is_empty());
    let destination = PathBuf::from(std::env::var_os("OUT_DIR").expect("Cargo OUT_DIR"));
    fs::write(
        destination.join("evaluation_provenance.rs"),
        format!(
            "const ALGORITHM_REVISION: &str = {revision:?};\nconst SOURCE_CLEAN: bool = {clean};\n"
        ),
    )
    .expect("write compile-time evaluation provenance");
}
