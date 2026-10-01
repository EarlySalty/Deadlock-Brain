mod common;
use common::*;
use dbrain_replay::*;
use proptest::prelude::*;
use prost::Message;
use sha2::{Digest, Sha256};
use valveprotos::common::{CsvcMsgCreateStringTable, EDemoCommands, SvcMessages};

#[test]
fn minimal_authored_source2_container_decodes_through_real_worker() {
    let bytes = minimal();
    let report = decode(&bytes, &request()).unwrap();
    assert_eq!(
        report.artifact.sha256,
        format!("{:x}", Sha256::digest(&bytes))
    );
    assert_eq!(report.artifact.byte_length, bytes.len() as u64);
    assert_eq!(report.commands, 2);
    assert_eq!(report.observations.len(), 1);
    assert_eq!(report.observations[0].raw.file_byte_offset, 16);
    assert_eq!(
        report.observations[0].time.tick,
        Observed::unknown(UnknownReason::InitializationTick)
    );
    assert!(matches!(
        report.observations[0].time.game_time_seconds,
        Observed::Unknown {
            reason: UnknownReason::InitializationTick
        }
    ));
    assert!(!report.real_replay_verified && !report.coaching_eligible);
    assert!(report.capabilities.iter().all(|c| !c.real_replay_verified));
}
#[test]
fn snappy_outer_command_preserves_observed_values_not_raw_identity() {
    let plain = decode(&minimal(), &request()).unwrap();
    let packed = decode(&container(&[], true), &request()).unwrap();
    assert_eq!(plain.observations[0].event, packed.observations[0].event);
    assert_ne!(plain.artifact.sha256, packed.artifact.sha256);
    assert_ne!(plain.generation_id, packed.generation_id);
    assert!(packed.observations[0].raw.compressed);
}
#[test]
fn observed_server_interval_never_becomes_an_assumed_game_clock() {
    let report = decode(&container(&[server(42, Some(0.02))], false), &request()).unwrap();
    let o = &report.observations[1];
    assert_eq!(o.time.tick, Observed::known(42));
    assert_eq!(o.time.tick_interval_seconds, Observed::known(0.02));
    assert!(matches!(
        o.time.game_time_seconds,
        Observed::Unknown {
            reason: UnknownReason::TickOriginNotEstablished
        }
    ));
    assert_eq!(o.raw.packet_index, Some(0));
}
#[test]
fn absent_interval_is_unknown_not_zero_or_sixty_hz() {
    let report = decode(&container(&[server(42, None)], false), &request()).unwrap();
    assert!(matches!(
        report.observations[1].time.tick_interval_seconds,
        Observed::Unknown {
            reason: UnknownReason::NotPresent
        }
    ));
}
#[test]
fn invalid_intervals_are_quarantined() {
    for interval in [0.0, -1.0, f32::NAN, f32::INFINITY, 2.0] {
        assert_eq!(
            decode(&container(&[server(42, Some(interval))], false), &request()),
            Err(ReplayFailure::UnknownStructure)
        );
    }
}
#[test]
fn unknown_network_types_are_opaque_not_invented_game_events() {
    let report = decode(
        &container(&[packet(8, &[(909, vec![1]), (909, vec![2])])], false),
        &request(),
    )
    .unwrap();
    assert_eq!(report.packets, 2);
    assert_eq!(report.observations.len(), 2);
    assert!(matches!(
        report.observations[1].event,
        ObservationKind::UnknownPacket { message_type: 909 }
    ));
}
#[test]
fn every_strict_prefix_of_minimal_container_is_rejected() {
    let bytes = minimal();
    for end in 0..bytes.len() {
        assert!(
            decode(&bytes[..end], &request()).is_err(),
            "accepted truncated prefix {end}"
        );
    }
}
#[test]
fn partial_final_varint_is_not_a_clean_eof() {
    for suffix in [vec![0x80], vec![0xff; 5], vec![0, 0x80], vec![0, 0, 0x80]] {
        let mut bytes = minimal();
        bytes.extend(suffix);
        assert!(decode(&bytes, &request()).is_err());
    }
}
#[test]
fn overlong_varint_is_rejected() {
    let mut bytes = b"PBDEMS2\0".to_vec();
    bytes.extend([0; 8]);
    bytes.extend([0x81, 0, 0, 0]);
    assert_eq!(
        decode(&bytes, &request()),
        Err(ReplayFailure::DamagedReplay)
    );
}
#[test]
fn unsupported_magic_is_not_decompressed_by_guessing() {
    let mut bytes = minimal();
    bytes[..8].copy_from_slice(b"HL2DEMO\0");
    assert_eq!(
        decode(&bytes, &request()),
        Err(ReplayFailure::InvalidContainer)
    );
}
#[test]
fn nonboundary_fileinfo_pointer_is_damaged() {
    let mut bytes = minimal();
    bytes[8..12].copy_from_slice(&17i32.to_le_bytes());
    assert_eq!(
        decode(&bytes, &request()),
        Err(ReplayFailure::DamagedReplay)
    );
}
#[test]
fn unknown_command_sentinel_is_quarantined() {
    let bytes = container(&[stored_command(19, 10, &[])], false);
    assert_eq!(
        decode(&bytes, &request()),
        Err(ReplayFailure::UnknownStructure)
    );
}
#[test]
fn negative_tick_other_than_initialization_is_not_reinterpreted() {
    let bytes = container(
        &[command(EDemoCommands::DemSyncTick, -2, &[], false)],
        false,
    );
    assert_eq!(
        decode(&bytes, &request()),
        Err(ReplayFailure::UnknownStructure)
    );
}
#[test]
fn oversized_declared_command_is_rejected_before_allocation() {
    let mut bytes = b"PBDEMS2\0".to_vec();
    bytes.extend([0; 8]);
    bytes.extend([1, 0]);
    bytes.extend(varint(u32::MAX));
    assert_eq!(
        decode(&bytes, &request()),
        Err(ReplayFailure::BudgetExceeded)
    );
}
#[test]
fn outer_snappy_bomb_is_rejected_before_allocating_declared_output() {
    let bomb = varint(u32::MAX);
    let bytes = container(
        &[stored_command(
            EDemoCommands::DemSyncTick as u32 | 64,
            1,
            &bomb,
        )],
        false,
    );
    assert_eq!(
        decode(&bytes, &request()),
        Err(ReplayFailure::BudgetExceeded)
    );
}
#[test]
fn nested_string_table_snappy_bomb_is_bounded_before_upstream_decode() {
    let msg = CsvcMsgCreateStringTable {
        name: Some("synthetic".into()),
        num_entries: Some(1),
        data_compressed: Some(true),
        string_data: Some(varint(u32::MAX)),
        ..Default::default()
    };
    let bytes = container(
        &[packet(
            1,
            &[(
                SvcMessages::SvcCreateStringTable as u32,
                msg.encode_to_vec(),
            )],
        )],
        false,
    );
    assert_eq!(
        decode(&bytes, &request()),
        Err(ReplayFailure::BudgetExceeded)
    );
}
#[test]
fn file_size_budget_is_enforced() {
    let mut r = request();
    r.budget.max_file_bytes = 8;
    assert_eq!(decode(&minimal(), &r), Err(ReplayFailure::BudgetExceeded));
}
#[test]
fn total_decoded_budget_is_enforced() {
    let mut r = request();
    r.budget.max_total_decoded_bytes = 1;
    assert_eq!(decode(&minimal(), &r), Err(ReplayFailure::BudgetExceeded));
}
#[test]
fn command_count_budget_is_enforced() {
    let mut r = request();
    r.budget.max_commands = 1;
    assert_eq!(decode(&minimal(), &r), Err(ReplayFailure::BudgetExceeded));
}
#[test]
fn packet_count_budget_is_enforced() {
    let mut r = request();
    r.budget.max_packets = 1;
    assert_eq!(
        decode(
            &container(&[server(1, Some(0.02)), server(2, Some(0.02))], false),
            &r
        ),
        Err(ReplayFailure::BudgetExceeded)
    );
}
#[test]
fn observation_budget_discards_partial_results() {
    let mut r = request();
    r.budget.max_observations = 1;
    assert_eq!(
        decode(&container(&[server(1, Some(0.02))], false), &r),
        Err(ReplayFailure::BudgetExceeded)
    );
}
#[test]
fn output_budget_is_an_os_enforced_file_limit() {
    let mut r = request();
    r.budget.max_output_bytes = 256;
    assert_eq!(decode(&minimal(), &r), Err(ReplayFailure::BudgetExceeded));
}
#[test]
fn hash_mismatch_is_rejected() {
    let mut r = request();
    r.source.expected_sha256 = Some("0".repeat(64));
    assert_eq!(decode(&minimal(), &r), Err(ReplayFailure::HashMismatch));
}
#[test]
fn rights_are_checked_before_opening_any_raw_path() {
    let mut r = request();
    r.source.rights.local_processing_allowed = false;
    assert_eq!(
        worker().decode(std::path::Path::new("/does/not/exist"), &r),
        Err(ReplayFailure::RightsDenied)
    );
    r.source.rights.local_processing_allowed = true;
    r.source.rights.external_egress_allowed = true;
    assert_eq!(decode(&minimal(), &r), Err(ReplayFailure::RightsDenied));
}
#[test]
fn raw_bytes_remain_unchanged_and_private_headers_are_not_exported() {
    let temp = tempfile::tempdir().unwrap();
    let input = temp.path().join("private-original.raw");
    let bytes = minimal();
    std::fs::write(&input, &bytes).unwrap();
    let report = worker().decode(&input, &request()).unwrap();
    assert_eq!(std::fs::read(&input).unwrap(), bytes);
    let output = serde_json::to_string(&report).unwrap();
    assert!(!output.contains("PRIVATE-SYNTHETIC") && !output.contains("private-original.raw"));
}
#[cfg(unix)]
#[test]
fn symlink_inputs_are_not_followed() {
    let temp = tempfile::tempdir().unwrap();
    let target = temp.path().join("raw");
    let link = temp.path().join("link");
    std::fs::write(&target, minimal()).unwrap();
    std::os::unix::fs::symlink(&target, &link).unwrap();
    assert_eq!(
        worker().decode(&link, &request()),
        Err(ReplayFailure::InputIo)
    );
}
#[test]
fn reparse_is_byte_deterministic() {
    let input = container(&[server(42, Some(0.02))], false);
    let a = decode(&input, &request()).unwrap();
    let b = decode(&input, &request()).unwrap();
    assert_eq!(
        serde_json::to_vec(&a).unwrap(),
        serde_json::to_vec(&b).unwrap()
    );
}
#[test]
fn extraction_selection_creates_a_new_generation() {
    let first = decode(&minimal(), &request()).unwrap();
    let mut r = request();
    r.selection.sample_every_ticks += 1;
    let second = decode(&minimal(), &r).unwrap();
    assert_ne!(first.generation_id, second.generation_id);
    assert_ne!(
        first.observations[0].observation_id,
        second.observations[0].observation_id
    );
}
#[test]
fn identical_raw_from_two_feeds_retains_provenance_but_not_duplicate_observation_ids() {
    let a = decode(&minimal(), &request()).unwrap();
    let mut r = request();
    r.source.source_id = "second-feed".into();
    r.source.source_revision = "different-source-revision".into();
    let b = decode(&minimal(), &r).unwrap();
    assert_eq!(a.generation_id, b.generation_id);
    assert_eq!(a.observations, b.observations);
    assert_ne!(a.artifact.source, b.artifact.source);
}
#[test]
fn upstream_packet_buffer_panic_is_contained_without_partial_success() {
    let mut bits = Bits::default();
    bits.ubitvar(SvcMessages::SvcServerInfo as u32);
    bits.uvarint(2 * 1024 * 1024 + 1);
    let bytes = container(&[packet_bits(5, bits.bytes)], false);
    assert_eq!(decode(&bytes, &request()), Err(ReplayFailure::ParserPanic));
}
#[test]
fn schema_contract_rejects_unknown_request_fields_and_missing_authorization() {
    let mut value = serde_json::to_value(request()).unwrap();
    value["surprise"] = true.into();
    assert!(serde_json::from_value::<ReplayRequest>(value).is_err());
    let mut value = serde_json::to_value(request()).unwrap();
    value["source"]["rights"]
        .as_object_mut()
        .unwrap()
        .remove("authorization_ref");
    assert!(serde_json::from_value::<ReplayRequest>(value).is_err());
}
proptest! {
    #![proptest_config(ProptestConfig { cases: 32, failure_persistence: None, ..ProptestConfig::default() })]
    #[test]
    fn arbitrary_untrusted_bytes_do_not_crash_the_supervisor(bytes in prop::collection::vec(any::<u8>(), 0..256)) {
        let outcome = decode(&bytes, &request());
        prop_assert!(outcome.is_err());
    }
    #[test]
    fn synthetic_interval_roundtrips_without_assuming_sixty_hz(interval in 0.001f32..0.2f32, tick in 0i32..1_000_000i32) {
        let report = decode(&container(&[server(tick, Some(interval))], false), &request()).unwrap();
        prop_assert_eq!(&report.observations[1].time.tick_interval_seconds, &Observed::known(interval));
        prop_assert!(matches!(report.observations[1].time.game_time_seconds, Observed::Unknown { .. }), "game clock must remain unknown");
    }
}
