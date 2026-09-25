use std::{env, process::Command};
fn main() {
    println!("cargo:rerun-if-env-changed=RUSTC");
    let compiler = env::var_os("RUSTC").expect("Cargo must supply RUSTC");
    let output = Command::new(compiler)
        .arg("--version")
        .output()
        .expect("compiler provenance unavailable");
    assert!(output.status.success(), "compiler provenance failed");
    let version = String::from_utf8(output.stdout).expect("compiler provenance is not UTF-8");
    println!("cargo:rustc-env=S10_RUSTC_VERSION={}", version.trim());
}
