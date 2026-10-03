#![forbid(unsafe_code)]

mod provider_shadow;

fn main() {
    if let Err(error) = provider_shadow::execute(std::env::args().skip(1).collect()) {
        eprintln!(
            "{}",
            serde_json::json!({"event":"shadow_failed", "code":error})
        );
        std::process::exit(1);
    }
}
