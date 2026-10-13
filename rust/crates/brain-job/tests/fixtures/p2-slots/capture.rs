pub fn read(name: &str) -> Vec<u8> {
    let expected = match name {
        "heroes-en.json.gz" => "5c163d19407aa683a6ebbae6e79d58885d71b3af3c0343e28cf0ec2e7fc75c31",
        "hero-data.json.gz" => "947c26790e067109f6cca4085649ed56e446bc5f4c9c571cd396290f802a3456",
        "items-en.json.gz" => "989ea40a2a7638db86bc484263a0ee74a65356e5a5139c2ebf45344f5e648efd",
        _ => panic!("unknown own capture"),
    };
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace crates")
        .join("brain-job/tests/fixtures/p2-slots")
        .join(name);
    let output = std::process::Command::new("gzip")
        .args(["-dc", "--"])
        .arg(path)
        .output()
        .expect("fixture decompression");
    assert!(output.status.success(), "complete fixture decompression");
    assert_eq!(
        brain_storage::entity_profile::derivation::sha256(&output.stdout),
        expected
    );
    output.stdout
}
