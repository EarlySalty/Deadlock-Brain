mod common;
use common::*;
use dbrain_replay::*;
use prost::Message;
use valveprotos::common::*;

fn entry(stored: &[u8], varint: bool) -> Vec<u8> {
    let mut bits = Bits::default();
    bits.bits(1, 1); // index 0
    bits.bits(0, 1); // no key string
    bits.bits(1, 1); // user data present
    bits.bits(1, 1); // this entry is Snappy compressed
    if varint {
        bits.ubitvar(stored.len() as u32);
    } else {
        bits.bits(stored.len() as u64, 17);
    }
    bits.bytes(stored);
    bits.bytes
}
fn create(data: Vec<u8>, count: i32, varint: bool, compressed: bool) -> CsvcMsgCreateStringTable {
    let size = data.len() as i32;
    CsvcMsgCreateStringTable {
        name: Some("synthetic_userdata".into()),
        num_entries: Some(count),
        flags: Some(1),
        using_varint_bitcounts: Some(varint),
        data_compressed: Some(compressed),
        uncompressed_size: Some(size),
        string_data: Some(if compressed {
            snap::raw::Encoder::new().compress_vec(&data).unwrap()
        } else {
            data
        }),
        ..Default::default()
    }
}
fn wrapped(msg: &CsvcMsgCreateStringTable) -> Vec<u8> {
    container(
        &[packet(
            1,
            &[(
                SvcMessages::SvcCreateStringTable as u32,
                msg.encode_to_vec(),
            )],
        )],
        false,
    )
}
#[test]
fn per_entry_snappy_is_counted_in_total_decoded_work() {
    let packed = snap::raw::Encoder::new().compress_vec(&[0; 4096]).unwrap();
    let report = decode(
        &wrapped(&create(entry(&packed, false), 1, false, false)),
        &request(),
    )
    .unwrap();
    assert!(report.total_decoded_bytes >= 4096);
}
#[test]
fn per_entry_snappy_cannot_evade_the_total_budget() {
    let packed = snap::raw::Encoder::new().compress_vec(&[0; 4096]).unwrap();
    let mut r = request();
    r.budget.max_total_decoded_bytes = 512;
    assert_eq!(
        decode(
            &wrapped(&create(entry(&packed, false), 1, false, false)),
            &r
        ),
        Err(ReplayFailure::BudgetExceeded)
    );
}
#[test]
fn per_entry_declared_output_cannot_overrun_upstream_fixed_scratch() {
    let stored = varint(u32::MAX);
    assert_eq!(
        decode(
            &wrapped(&create(entry(&stored, false), 1, false, false)),
            &request()
        ),
        Err(ReplayFailure::BudgetExceeded)
    );
}
#[test]
fn update_stringtable_reuses_the_checked_compression_specification() {
    let initial = create(vec![], 0, false, false);
    let packed = snap::raw::Encoder::new().compress_vec(&[0; 4096]).unwrap();
    let update = CsvcMsgUpdateStringTable {
        table_id: Some(0),
        num_changed_entries: Some(1),
        string_data: Some(entry(&packed, false)),
    };
    let bytes = container(
        &[
            packet(
                1,
                &[(
                    SvcMessages::SvcCreateStringTable as u32,
                    initial.encode_to_vec(),
                )],
            ),
            packet(
                2,
                &[(
                    SvcMessages::SvcUpdateStringTable as u32,
                    update.encode_to_vec(),
                )],
            ),
        ],
        false,
    );
    let mut r = request();
    r.budget.max_total_decoded_bytes = 512;
    assert_eq!(decode(&bytes, &r), Err(ReplayFailure::BudgetExceeded));
}
#[test]
fn all_three_snappy_layers_and_varint_entry_lengths_decode() {
    let stored = snap::raw::Encoder::new().compress_vec(&[0; 4096]).unwrap();
    let msg = create(entry(&stored, true), 1, true, true).encode_to_vec();
    let mut net = Bits::default();
    net.ubitvar(SvcMessages::SvcCreateStringTable as u32);
    net.uvarint(msg.len() as u32);
    net.bytes(&msg);
    let bytes = container(
        &[command(
            EDemoCommands::DemPacket,
            1,
            &CDemoPacket {
                data: Some(net.bytes),
            }
            .encode_to_vec(),
            true,
        )],
        false,
    );
    let report = decode(&bytes, &request()).unwrap();
    assert!(report.total_decoded_bytes >= 4096);
    assert!(!report.real_replay_verified);
}
#[test]
fn unsupported_table_flags_are_quarantined() {
    let mut msg = create(vec![], 0, false, false);
    msg.flags = Some(2);
    assert_eq!(
        decode(&wrapped(&msg), &request()),
        Err(ReplayFailure::UnknownStructure)
    );
}
