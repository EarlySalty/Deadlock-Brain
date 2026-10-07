use std::{fs, path::PathBuf, process::Command};

#[test]
fn assets_cli_exposes_the_durable_default() {
    let output = Command::new(env!("CARGO_BIN_EXE_deadlock-brain"))
        .env_clear()
        .args(["pull", "assets", "--help"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8(output.stdout)
        .unwrap()
        .contains("/home/nathanael/.local/share/deadlock-brain"));
}

#[test]
fn assets_cli_uses_configured_directories_and_keeps_explicit_cli_priority() {
    for explicit in [false, true] {
        let directory = tempfile::tempdir().unwrap();
        let configured = directory.path().join("configured");
        let cli_path = directory.path().join("explicit");
        let mut command = Command::new(env!("CARGO_BIN_EXE_deadlock-brain"));
        command
            .env_clear()
            .env("DEADLOCK_BRAIN_DATA_DIR", &configured)
            .current_dir(directory.path())
            .args(["pull", "assets"]);
        if explicit {
            command.arg("--data-dir").arg(&cli_path);
        }
        let output = command.output().unwrap();
        assert_eq!(output.status.code(), Some(1));
        assert!(String::from_utf8_lossy(&output.stderr).contains("DEADLOCK_CENTRAL_DSN"));
        assert!(output.stdout.is_empty());
        let selected = if explicit { &cli_path } else { &configured };
        assert!(selected.join("raw").is_dir());
        assert!(selected.join("cache").is_dir());
        assert_eq!(configured.exists(), !explicit);
        assert_eq!(cli_path.exists(), explicit);
    }
}

#[test]
fn assets_cli_rejects_relative_environment_directories_before_database_and_writes() {
    for data_dir in ["relative-assets", "./relative-assets", "../relative-assets"] {
        let directory = tempfile::tempdir().unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_deadlock-brain"))
            .env_clear()
            .env("DEADLOCK_BRAIN_DATA_DIR", data_dir)
            .current_dir(directory.path())
            .args(["pull", "assets"])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(1));
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("--data-dir"));
        assert!(stderr.contains("DEADLOCK_BRAIN_DATA_DIR"));
        assert!(!stderr.contains("DEADLOCK_CENTRAL_DSN"));
        assert!(output.stdout.is_empty());
        assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 0);
    }
}

#[test]
fn assets_cli_rejects_build_tree_paths_including_parent_components_and_symlinks() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap();
    let directory = tempfile::tempdir().unwrap();
    let alias = directory.path().join("build-alias");
    std::os::unix::fs::symlink(&root, &alias).unwrap();
    let suffix = directory.path().file_name().unwrap();
    let selected = root.join(suffix);
    assert!(!selected.exists());
    for data_dir in [
        selected.clone(),
        root.join("rust/..").join(suffix),
        alias.join(suffix),
        directory
            .path()
            .join("build-alias/..")
            .join(root.file_name().unwrap())
            .join(suffix),
    ] {
        for explicit in [false, true] {
            let mut command = Command::new(env!("CARGO_BIN_EXE_deadlock-brain"));
            command
                .env_clear()
                .current_dir(directory.path())
                .args(["pull", "assets"]);
            if explicit {
                command.arg("--data-dir").arg(&data_dir);
            } else {
                command.env("DEADLOCK_BRAIN_DATA_DIR", &data_dir);
            }
            let output = command.output().unwrap();
            assert_eq!(output.status.code(), Some(1));
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(stderr.contains("--data-dir"));
            assert!(!stderr.contains("DEADLOCK_CENTRAL_DSN"));
            assert!(output.stdout.is_empty());
            assert!(!selected.exists());
            assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 1);
        }
    }
}

fn rejects_build_tree_write_target(subpath: &str) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap();
    for explicit in [false, true] {
        for linked_target in ["direct", "alias", "relative"] {
            let directory = tempfile::tempdir().unwrap();
            let data_dir = directory.path().join("durable");
            let write_target = data_dir.join(subpath);
            fs::create_dir_all(write_target.parent().unwrap()).unwrap();
            let alias = directory.path().join("build-alias");
            std::os::unix::fs::symlink(&root, &alias).unwrap();
            let target = match linked_target {
                "direct" => root.clone(),
                "alias" => alias.join("rust/.."),
                "relative" if subpath.contains('/') => PathBuf::from("../../build-alias/rust/.."),
                "relative" => PathBuf::from("../build-alias/rust/.."),
                _ => unreachable!(),
            };
            std::os::unix::fs::symlink(&target, &write_target).unwrap();
            let mut command = Command::new(env!("CARGO_BIN_EXE_deadlock-brain"));
            command
                .env_clear()
                .current_dir(directory.path())
                .args(["pull", "assets"]);
            if explicit {
                command.arg("--data-dir").arg(&data_dir);
            } else {
                command.env("DEADLOCK_BRAIN_DATA_DIR", &data_dir);
            }
            let output = command.output().unwrap();
            assert_eq!(output.status.code(), Some(1));
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(stderr.contains("--data-dir"), "{stderr}");
            assert!(!stderr.contains("DEADLOCK_CENTRAL_DSN"));
            assert!(output.stdout.is_empty());
            assert_eq!(fs::read_link(&write_target).unwrap(), target);
            assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 2);
            assert_eq!(fs::read_dir(&data_dir).unwrap().count(), 1);
            if subpath.contains('/') {
                assert_eq!(fs::read_dir(data_dir.join("raw")).unwrap().count(), 1);
            }
        }
    }
}

#[test]
fn assets_cli_rejects_raw_directory_links_before_database_and_writes() {
    rejects_build_tree_write_target("raw");
}

#[test]
fn assets_cli_rejects_cache_directory_links_before_database_and_writes() {
    rejects_build_tree_write_target("cache");
}

#[test]
fn assets_cli_rejects_source_directory_links_before_database_and_writes() {
    rejects_build_tree_write_target("raw/deadlock_assets_api");
}

#[test]
fn assets_cli_accepts_write_target_links_outside_the_build_tree() {
    for explicit in [false, true] {
        let directory = tempfile::tempdir().unwrap();
        let data_dir = directory.path().join("durable");
        let raw = directory.path().join("originals");
        let cache = directory.path().join("responses");
        let source = directory.path().join("assets");
        for path in [&data_dir, &raw, &cache, &source] {
            fs::create_dir_all(path).unwrap();
        }
        for (target, link) in [
            (&raw, data_dir.join("raw")),
            (&cache, data_dir.join("cache")),
            (&source, raw.join("deadlock_assets_api")),
        ] {
            std::os::unix::fs::symlink(target, link).unwrap();
        }
        let mut command = Command::new(env!("CARGO_BIN_EXE_deadlock-brain"));
        command
            .env_clear()
            .current_dir(directory.path())
            .args(["pull", "assets"]);
        if explicit {
            command.arg("--data-dir").arg(&data_dir);
        } else {
            command.env("DEADLOCK_BRAIN_DATA_DIR", &data_dir);
        }
        let output = command.output().unwrap();
        assert_eq!(output.status.code(), Some(1));
        assert!(String::from_utf8_lossy(&output.stderr).contains("DEADLOCK_CENTRAL_DSN"));
        assert!(output.stdout.is_empty());
        assert_eq!(fs::read_dir(&source).unwrap().count(), 0);
        assert_eq!(fs::read_dir(&cache).unwrap().count(), 0);
        assert_eq!(fs::read_dir(&raw).unwrap().count(), 1);
        assert_eq!(fs::read_dir(&data_dir).unwrap().count(), 2);
    }
}

#[test]
fn assets_cli_absolute_override_takes_priority_over_invalid_configuration() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap();
    for configured in [PathBuf::from("relative-assets"), root.join("data")] {
        let directory = tempfile::tempdir().unwrap();
        let cli_path = directory.path().join("explicit");
        let output = Command::new(env!("CARGO_BIN_EXE_deadlock-brain"))
            .env_clear()
            .env("DEADLOCK_BRAIN_DATA_DIR", configured)
            .current_dir(directory.path())
            .args(["pull", "assets", "--data-dir"])
            .arg(&cli_path)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(1));
        assert!(String::from_utf8_lossy(&output.stderr).contains("DEADLOCK_CENTRAL_DSN"));
        assert!(output.stdout.is_empty());
        assert!(cli_path.join("raw").is_dir());
        assert!(cli_path.join("cache").is_dir());
        assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 1);
    }
}

#[test]
fn assets_cli_rejects_relative_and_empty_directories_without_writes() {
    for (data_dir, exit_code) in [("relative-assets", 1), ("", 2)] {
        let directory = tempfile::tempdir().unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_deadlock-brain"))
            .env_clear()
            .current_dir(directory.path())
            .args(["pull", "assets", "--data-dir", data_dir])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(exit_code));
        assert!(output.stdout.is_empty());
        assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 0);
    }
}
