use brain_runtime_audit::{collect, discover, parse_unit, render};

const NAME: &str = "deadlock-brain-test.service";

fn service() -> String {
    format!(
        "Id={NAME}\nLoadState=loaded\nActiveState=inactive\nSubState=dead\n\
         UnitFileState=static\nResult=success\nExecMainStatus=0\nMainPID=0\n\
         MemoryMax=infinity\nCPUQuotaPerSecUSec=infinity\nTasksMax=infinity\n\
         TimeoutStopUSec=1min 30s\n\
         ExecStart={{ path=/usr/bin/bash ; argv[]=/usr/bin/bash /private/job --token NEVER_PRINT_THIS ; }}\n"
    )
}

fn timer() -> String {
    "Id=deadlock-brain-test.timer\nLoadState=loaded\nActiveState=active\nSubState=waiting\nUnitFileState=enabled\n".to_owned()
}

#[test]
fn discovery_unions_loaded_and_installed_deduplicated_and_sorted() {
    let actual = discover(
        "deadlock-brain-test.service loaded inactive dead job\ndl-knowledge.service loaded active running knowledge\n",
        "deadlock-brain-test.timer enabled enabled\ndeadlock-brain-test.service static -\n",
    ).unwrap();
    assert_eq!(
        actual,
        [
            "deadlock-brain-test.service",
            "deadlock-brain-test.timer",
            "dl-knowledge.service"
        ]
    );
}

#[test]
fn inactive_installed_unit_is_not_omitted() {
    assert_eq!(
        discover("", "deadlock-brain-test.service disabled disabled\n").unwrap(),
        [NAME]
    );
}

#[test]
fn unrelated_and_prefix_collision_units_are_excluded() {
    assert_eq!(discover("", "deadlock-brainstorm.service enabled\ndl-knowledge-other.service enabled\nssh.service enabled\ndeadlock-brain-test.service static\n").unwrap(), [NAME]);
}

#[test]
fn failed_unit_bullet_is_accepted_without_losing_coverage() {
    assert_eq!(
        discover(
            "● deadlock-brain-test.service loaded failed failed job\n",
            ""
        )
        .unwrap(),
        [NAME]
    );
}

#[test]
fn empty_inventory_is_an_error_not_a_clean_runtime() {
    assert!(discover("", "").is_err());
}

#[test]
fn unsafe_in_scope_unit_is_rejected() {
    for name in [
        "deadlock-brain-;stop.service",
        "deadlock-brain-$(id).service",
        "deadlock-brain-../x.service",
    ] {
        assert!(discover(&format!("{name} loaded active running\n"), "").is_err());
    }
}

#[test]
fn excessive_output_is_rejected() {
    assert!(discover(&"x".repeat(1_048_577), "").is_err());
}

#[test]
fn excessive_unit_count_is_rejected() {
    let loaded = (0..257)
        .map(|i| format!("deadlock-brain-{i}.service loaded active running\n"))
        .collect::<String>();
    assert!(discover(&loaded, "").is_err());
}

#[test]
fn service_properties_are_read_without_exposing_arguments() {
    let report = render(&[parse_unit(NAME, &service()).unwrap()]);
    assert!(report.contains(NAME));
    assert!(report.contains("shell_unverified"));
    assert!(report.contains("unbounded"));
    assert!(!report.contains("NEVER_PRINT_THIS"));
    assert!(!report.contains("/private"));
    assert!(!report.contains("/usr/bin"));
}

#[test]
fn unrequested_secret_properties_are_never_rendered() {
    let input = format!(
        "{}Environment=TOKEN=NEVER_PRINT_THIS\nWorkingDirectory=/private/secret\n",
        service()
    );
    let report = render(&[parse_unit(NAME, &input).unwrap()]);
    assert!(!report.contains("NEVER_PRINT_THIS"));
    assert!(!report.contains("/private"));
}

#[test]
fn python_entrypoint_is_a_candidate_not_hidden_as_native() {
    let input = service().replace("path=/usr/bin/bash", "path=/opt/venv/bin/python3.12");
    assert!(render(&[parse_unit(NAME, &input).unwrap()]).contains("python_candidate"));
}

#[test]
fn other_entrypoint_remains_unverified() {
    let input = service().replace("path=/usr/bin/bash", "path=/opt/bin/deadlock-brain");
    assert!(render(&[parse_unit(NAME, &input).unwrap()]).contains("other_unverified"));
}

#[test]
fn empty_or_multiple_entrypoints_remain_unknown() {
    for entry in ["", "{ path=/bin/one ; } { path=/bin/two ; }"] {
        let input = service()
            .lines()
            .filter(|line| !line.starts_with("ExecStart="))
            .collect::<Vec<_>>()
            .join("\n")
            + "\nExecStart="
            + entry;
        assert!(render(&[parse_unit(NAME, &input).unwrap()]).contains("unknown_entrypoint"));
    }
}

#[test]
fn timers_do_not_require_service_only_fields() {
    assert!(
        render(&[parse_unit("deadlock-brain-test.timer", &timer()).unwrap()]).contains("waiting")
    );
}

#[test]
fn missing_required_property_is_not_success() {
    let input = service().replace("Result=success\n", "");
    assert!(parse_unit(NAME, &input).is_err());
}

#[test]
fn duplicate_property_is_rejected() {
    assert!(parse_unit(NAME, &(service() + "ActiveState=active\n")).is_err());
}

#[test]
fn mismatched_or_foreign_unit_id_is_rejected() {
    assert!(parse_unit(
        NAME,
        &service().replace(NAME, "deadlock-brain-other.service")
    )
    .is_err());
    assert!(parse_unit("ssh.service", &service().replace(NAME, "ssh.service")).is_err());
}

#[test]
fn unrecognized_state_is_not_echoed_or_silently_approved() {
    let error = parse_unit(
        NAME,
        &service().replace("Result=success", "Result=NEVER_PRINT_THIS"),
    )
    .unwrap_err();
    assert!(!error.contains("NEVER_PRINT_THIS"));
}

#[test]
fn invalid_resource_values_are_rejected() {
    for (field, old) in [
        ("MemoryMax", "infinity"),
        ("CPUQuotaPerSecUSec", "infinity"),
        ("TasksMax", "infinity"),
        ("TimeoutStopUSec", "1min 30s"),
    ] {
        assert!(parse_unit(
            NAME,
            &service().replace(
                &format!("{field}={old}"),
                &format!("{field}=NEVER_PRINT_THIS")
            )
        )
        .is_err());
    }
}

#[test]
fn configured_limits_and_duration_are_classified() {
    let input = service()
        .replace("MemoryMax=infinity", "MemoryMax=1048576")
        .replace("CPUQuotaPerSecUSec=infinity", "CPUQuotaPerSecUSec=1s 500ms")
        .replace("TasksMax=infinity", "TasksMax=64");
    assert!(render(&[parse_unit(NAME, &input).unwrap()]).contains("bounded"));
}

#[test]
fn failed_services_stay_visible_in_a_complete_snapshot() {
    let input = service()
        .replace("ActiveState=inactive", "ActiveState=failed")
        .replace("SubState=dead", "SubState=failed")
        .replace("Result=success", "Result=exit-code")
        .replace("ExecMainStatus=0", "ExecMainStatus=1");
    let report = render(&[parse_unit(NAME, &input).unwrap()]);
    assert!(report.contains("failed"));
    assert!(report.contains("exit-code"));
}

#[test]
fn snapshot_never_claims_cutover_or_python_free_runtime() {
    let report = render(&[parse_unit(NAME, &service()).unwrap()]);
    assert!(report.contains("cutover_authorized=false"));
    assert!(report.contains("full_runtime_verification=not_performed"));
    assert!(report.contains("scope=user_systemd_metadata_only"));
}

#[test]
fn collection_uses_only_fixed_read_only_calls_and_no_environment_property() {
    let mut calls = Vec::<Vec<String>>::new();
    let units = collect(&mut |args| {
        calls.push(args.iter().map(|v| (*v).to_owned()).collect());
        if args.contains(&"list-units") {
            Ok(format!("{NAME} loaded inactive dead job\n"))
        } else if args.contains(&"list-unit-files") {
            Ok("deadlock-brain-test.timer enabled enabled\n".to_owned())
        } else if args.contains(&NAME) {
            Ok(service())
        } else {
            Ok(timer())
        }
    })
    .unwrap();
    assert_eq!(units.len(), 2);
    assert_eq!(calls.len(), 4);
    for call in &calls {
        assert!(call.iter().any(|s| s == "--user"));
        assert!(call.iter().any(|s| s == "--no-pager"));
        assert!(!call.iter().any(|s| [
            "start",
            "stop",
            "restart",
            "enable",
            "disable",
            "daemon-reload"
        ]
        .contains(&s.as_str())));
        assert!(!call.iter().any(|s| s.contains("Environment")));
    }
}

#[test]
fn command_failure_produces_no_partial_success_and_redacts_error() {
    let error = collect(&mut |_| Err("NEVER_PRINT_THIS")).unwrap_err();
    assert!(!error.contains("NEVER_PRINT_THIS"));
}

#[test]
fn failure_in_later_unit_invalidates_the_snapshot() {
    let mut count = 0;
    let result = collect(&mut |args| {
        count += 1;
        if args.contains(&"list-units") || args.contains(&"list-unit-files") {
            Ok(format!("{NAME} loaded\ndl-knowledge.service loaded\n"))
        } else if count == 3 {
            Ok(service())
        } else {
            Err("private stderr")
        }
    });
    assert!(result.is_err());
    assert_eq!(count, 4);
}
