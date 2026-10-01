#![allow(dead_code)]
// Original synthetic fixtures. No recorded match data, accounts, player names, or downloaded demos.
use dbrain_replay::*;
use prost::Message;
use std::path::PathBuf;
use valveprotos::common::{
    CDemoFileHeader, CDemoPacket, CsvcMsgServerInfo, EDemoCommands, SvcMessages,
};

pub fn request() -> ReplayRequest {
    ReplayRequest {
        source: ReplaySource {
            source_id: "synthetic-test".into(),
            source_revision: "authored-v1".into(),
            raw_object_ref: "private:test-object".into(),
            retrieved_at_unix_ms: None,
            expected_sha256: None,
            match_reference: None,
            rights: ReplayRights {
                authorization_ref: "synthetic-authorship".into(),
                scope: "test-only".into(),
                local_processing_allowed: true,
                raw_retention_allowed: true,
                external_egress_allowed: false,
            },
        },
        budget: DecodeBudget {
            cpu_seconds: 5,
            wall_time_ms: 10_000,
            ..DecodeBudget::default()
        },
        selection: ReplaySelection::default(),
    }
}
pub fn worker() -> WorkerDecoder {
    WorkerDecoder::new(PathBuf::from(env!("CARGO_BIN_EXE_dbrain-replay-worker")))
}
pub fn decode(bytes: &[u8], request: &ReplayRequest) -> Result<ReplayReport, ReplayFailure> {
    let temp = tempfile::tempdir().unwrap();
    let input = temp.path().join("synthetic.raw");
    std::fs::write(&input, bytes).unwrap();
    worker().decode(&input, request)
}
pub fn varint(mut n: u32) -> Vec<u8> {
    let mut out = Vec::new();
    loop {
        let byte = (n & 127) as u8;
        n >>= 7;
        out.push(byte | if n == 0 { 0 } else { 128 });
        if n == 0 {
            return out;
        }
    }
}
pub fn command(cmd: EDemoCommands, tick: i32, body: &[u8], compressed: bool) -> Vec<u8> {
    let data = if compressed {
        snap::raw::Encoder::new().compress_vec(body).unwrap()
    } else {
        body.to_vec()
    };
    stored_command(cmd as u32 | if compressed { 64 } else { 0 }, tick, &data)
}
pub fn stored_command(cmd: u32, tick: i32, body: &[u8]) -> Vec<u8> {
    let mut out = varint(cmd);
    out.extend(varint(tick as u32));
    out.extend(varint(body.len() as u32));
    out.extend(body);
    out
}
pub fn header(compressed: bool) -> Vec<u8> {
    let value = CDemoFileHeader {
        demo_file_stamp: "PBDEMS2".into(),
        game_directory: Some("citadel".into()),
        patch_version: Some(7),
        build_num: Some(123),
        server_name: Some("PRIVATE-SYNTHETIC-SERVER".into()),
        client_name: Some("PRIVATE-SYNTHETIC-PLAYER".into()),
        ..Default::default()
    };
    command(
        EDemoCommands::DemFileHeader,
        -1,
        &value.encode_to_vec(),
        compressed,
    )
}
pub fn container(middle: &[Vec<u8>], compressed_header: bool) -> Vec<u8> {
    let mut out = b"PBDEMS2\0".to_vec();
    out.extend([0; 8]);
    out.extend(header(compressed_header));
    for cmd in middle {
        out.extend(cmd);
    }
    out.extend(command(EDemoCommands::DemStop, 100, &[], false));
    out
}
pub fn minimal() -> Vec<u8> {
    container(&[], false)
}
#[derive(Default)]
pub struct Bits {
    pub bytes: Vec<u8>,
    pub position: usize,
}
impl Bits {
    pub fn bits(&mut self, n: u64, count: usize) {
        for bit in 0..count {
            if self.position / 8 == self.bytes.len() {
                self.bytes.push(0);
            }
            self.bytes[self.position / 8] |= (((n >> bit) & 1) as u8) << (self.position % 8);
            self.position += 1;
        }
    }
    pub fn bytes(&mut self, value: &[u8]) {
        for byte in value {
            self.bits(u64::from(*byte), 8);
        }
    }
    pub fn uvarint(&mut self, n: u32) {
        self.bytes(&varint(n));
    }
    pub fn ubitvar(&mut self, n: u32) {
        let (tag, count) = if n < 16 {
            (0, 0)
        } else if n < 256 {
            (16, 4)
        } else if n < 4096 {
            (32, 8)
        } else {
            (48, 28)
        };
        self.bits(u64::from((n & 15) | tag), 6);
        if count > 0 {
            self.bits(u64::from(n >> 4), count);
        }
    }
}
pub fn packet(tick: i32, messages: &[(u32, Vec<u8>)]) -> Vec<u8> {
    let mut bits = Bits::default();
    for (kind, data) in messages {
        bits.ubitvar(*kind);
        bits.uvarint(data.len() as u32);
        bits.bytes(data);
    }
    packet_bits(tick, bits.bytes)
}
pub fn packet_bits(tick: i32, bytes: Vec<u8>) -> Vec<u8> {
    command(
        EDemoCommands::DemPacket,
        tick,
        &CDemoPacket { data: Some(bytes) }.encode_to_vec(),
        false,
    )
}
pub fn server(tick: i32, interval: Option<f32>) -> Vec<u8> {
    packet(
        tick,
        &[(
            SvcMessages::SvcServerInfo as u32,
            CsvcMsgServerInfo {
                tick_interval: interval,
                ..Default::default()
            }
            .encode_to_vec(),
        )],
    )
}
