use brain_legacy_refresh::{
    activate_generation, capture, capture_and_stage, inspect_plan, read_plan, validate_generation,
    PrivateDirectory, Result,
};
use std::path::Path;

fn run() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("plan") if args.len() == 3 => {
            let plan = read_plan(Path::new(&args[2]))?;
            let mut source = plan.source.connect()?;
            let mut target = plan.target.connect()?;
            inspect_plan(&plan, &mut source, &mut target)?;
            println!("PLAN_VALID read_only=true activation_performed=false g5_complete=false");
        }
        Some("capture") if args.len() == 4 => {
            let plan = read_plan(Path::new(&args[2]))?;
            let directory = PrivateDirectory::open(Path::new(&args[3]))?;
            let mut source = plan.source.connect()?;
            capture(&plan, &mut source, &directory)?;
            println!("SNAPSHOT_CAPTURED activation_performed=false g5_complete=false");
        }
        Some("stage") if args.len() == 4 => {
            let plan = read_plan(Path::new(&args[2]))?;
            let directory = PrivateDirectory::open(Path::new(&args[3]))?;
            let mut source = plan.source.connect()?;
            let mut target = plan.target.connect()?;
            inspect_plan(&plan, &mut source, &mut target)?;
            capture_and_stage(&plan, &mut source, &mut target, &directory)?;
            println!("GENERATION_STAGED activation_performed=false g5_complete=false");
        }
        Some("activate") if args.len() == 4 => {
            let directory = PrivateDirectory::open(Path::new(&args[2]))?;
            let manifest = directory.read_manifest()?;
            if args[3] != manifest.archive_sha256 {
                return Err(brain_legacy_refresh::Error::Contract("Erwarteter Archivhash weicht ab"));
            }
            let mut target = manifest.plan.target.connect()?;
            let proof = validate_generation(&manifest, &mut target)?;
            let receipt = activate_generation(proof, &mut target, &directory)?;
            if directory.write_json("receipt.json", &receipt).is_err() {
                return Err(brain_legacy_refresh::Error::Contract("Archivwechsel bestätigt; Belegablage fehlgeschlagen. Nicht erneut aktivieren, Aktivierungsabsicht und OIDs lesend prüfen"));
            }
            println!("ARCHIVE_ACTIVATED q_binding_required=true g5_complete=false");
        }
        Some("validate") if args.len() == 3 => {
            let directory = PrivateDirectory::open(Path::new(&args[2]))?;
            let manifest = directory.read_manifest()?;
            let mut target = manifest.plan.target.connect()?;
            validate_generation(&manifest, &mut target)?;
            println!("GENERATION_VALID read_only=true activation_performed=false g5_complete=false");
        }
        _ => return Err(brain_legacy_refresh::Error::Contract("Aufruf: plan <privater Plan> | capture oder stage <privater Plan> <privates Verzeichnis> | validate <privates Verzeichnis> | activate <privates Verzeichnis> <erwarteter Archivhash>")),
    }
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
