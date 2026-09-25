mod common;
use common::*;
use dbrain_replay::*;
use prost::Message;
use valveprotos::common::*;

#[test]
fn fileinfo_trailer_offset_resolves_to_an_actual_command() {
    let mut bytes = minimal();
    let offset = bytes.len() as i32;
    bytes[8..12].copy_from_slice(&offset.to_le_bytes());
    bytes.extend(command(
        EDemoCommands::DemFileInfo,
        100,
        &CDemoFileInfo {
            playback_time: Some(2.0),
            playback_ticks: Some(100),
            ..Default::default()
        }
        .encode_to_vec(),
        false,
    ));
    let report = decode(&bytes, &request()).unwrap();
    assert_eq!(report.commands, 3);
    assert!(matches!(
        report.observations[1].event,
        ObservationKind::FileInfo { .. }
    ));
}
#[test]
fn missing_baseline_table_is_unknown_not_silently_dropped() {
    let full = CDemoFullPacket {
        string_table: Some(CDemoStringTables {
            tables: vec![c_demo_string_tables::TableT {
                table_name: Some("instancebaseline".into()),
                ..Default::default()
            }],
        }),
        packet: None,
    };
    let bytes = container(
        &[command(
            EDemoCommands::DemFullPacket,
            0,
            &full.encode_to_vec(),
            false,
        )],
        false,
    );
    assert_eq!(
        decode(&bytes, &request()),
        Err(ReplayFailure::UnknownStructure)
    );
}
#[test]
fn unordered_class_ids_are_rejected_before_upstream_vector_indexing() {
    let classes = CDemoClassInfo {
        classes: vec![
            c_demo_class_info::ClassT {
                class_id: Some(1),
                network_name: Some("A".into()),
                ..Default::default()
            },
            c_demo_class_info::ClassT {
                class_id: Some(0),
                network_name: Some("B".into()),
                ..Default::default()
            },
        ],
    };
    let bytes = container(
        &[command(
            EDemoCommands::DemClassInfo,
            -1,
            &classes.encode_to_vec(),
            false,
        )],
        false,
    );
    assert_eq!(
        decode(&bytes, &request()),
        Err(ReplayFailure::UnknownStructure)
    );
}
#[test]
fn invalid_protobuf_header_does_not_return_partial_metadata() {
    let mut bytes = b"PBDEMS2\0".to_vec();
    bytes.extend([0; 8]);
    bytes.extend(command(EDemoCommands::DemFileHeader, -1, &[0xff], false));
    bytes.extend(command(EDemoCommands::DemStop, 0, &[], false));
    assert_eq!(
        decode(&bytes, &request()),
        Err(ReplayFailure::DamagedReplay)
    );
}
#[test]
fn budgets_cannot_be_disabled_or_silently_made_unlimited() {
    let mut r = request();
    r.budget.cpu_seconds = 0;
    assert_eq!(validate_request(&r), Err(ReplayFailure::InvalidRequest));
    r = request();
    r.budget.memory_bytes = u64::MAX;
    assert_eq!(validate_request(&r), Err(ReplayFailure::InvalidRequest));
    r = request();
    r.budget.max_output_bytes = u64::MAX;
    assert_eq!(validate_request(&r), Err(ReplayFailure::InvalidRequest));
    r = request();
    r.selection.sample_every_ticks = 0;
    assert_eq!(validate_request(&r), Err(ReplayFailure::InvalidRequest));
}
