use super::*;

#[test]
fn encrypted_browser_state_roundtrip_concurrency_revocation_and_account_isolation() {
    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct TestConfig {
        socket: std::path::PathBuf,
        user: String,
        database: String,
    }
    // Passwortlose lokale Peer-Verbindung; die normale Testconfig enthält
    // ausschließlich Metadaten und öffnet keine produktive Datenbank.
    let config: TestConfig =
        serde_json::from_str(include_str!("../tests/database-config.json")).unwrap();
    assert!(config.socket.is_absolute());
    assert!(config.database.starts_with("token_db_"));
    let mut options = postgres::Config::new();
    options
        .host_path(&config.socket)
        .user(&config.user)
        .dbname(&config.database)
        .connect_timeout(std::time::Duration::from_secs(10));
    let mut admin = options.connect(NoTls).unwrap();
    let database = format!("token_db_browser_{}", std::process::id());
    admin
        .batch_execute(&format!("CREATE DATABASE {database}"))
        .unwrap();
    let mut local = options.clone();
    local.dbname(&database);
    let mut client = local.connect(NoTls).unwrap();
    client.batch_execute("CREATE SCHEMA core").unwrap();
    client.batch_execute(include_str!("../../../../../Deadlock-Bots/rust/crates/dl-central-db/migrations/20260930220300_browser_credentials.sql")).unwrap();
    let cipher = FieldCipher::from_hex_key(&"11".repeat(32), "v1").unwrap();
    let state =
        json!({"cookies":[{"name":"synthetic_cookie","value":"synthetic_secret"}],"origins":[]});
    assert_eq!(
        load(&mut client, &cipher, "account-a").unwrap()["revision"],
        -1
    );
    save(&mut client, &cipher, "account-a", -1, &state).unwrap();
    let first = load(&mut client, &cipher, "account-a").unwrap();
    assert_eq!(first["revision"], 0);
    assert_eq!(first["state"], state);
    assert_eq!(
        load(&mut client, &cipher, "account-b").unwrap()["revision"],
        -1
    );
    assert!(save(&mut client, &cipher, "account-a", -1, &state).is_err());
    // Replay the actual helper get/put codec at the exact stored-state limit.
    let mut boundary = json!({"cookies":[],"origins":[],"padding":""});
    let overhead = serde_json::to_vec(&boundary).unwrap().len();
    boundary["padding"] = json!("x".repeat(LIMIT - overhead));
    assert_eq!(serde_json::to_vec(&boundary).unwrap().len(), LIMIT);
    save(&mut client, &cipher, "boundary-account", -1, &boundary).unwrap();
    let envelope = load(&mut client, &cipher, "boundary-account").unwrap();
    let encoded = encode_envelope(&envelope).unwrap();
    assert!(encoded.len() > LIMIT);
    let (revision, replay) = decode_envelope(encoded.as_slice()).unwrap();
    save(&mut client, &cipher, "boundary-account", revision, &replay).unwrap();
    assert_eq!(
        load(&mut client, &cipher, "boundary-account").unwrap()["revision"],
        1
    );
    let mut oversized = boundary.clone();
    oversized["padding"] = json!("x".repeat(LIMIT - overhead + 1));
    assert!(decode_envelope(
        encode_envelope(&json!({"revision":1,"state":oversized}))
            .unwrap()
            .as_slice()
    )
    .is_err());
    assert!(save(&mut client, &cipher, "boundary-account", 1, &oversized).is_err());
    assert_eq!(
        load(&mut client, &cipher, "boundary-account").unwrap()["revision"],
        1
    );
    let blob: Vec<u8> = client
        .query_one(
            "SELECT state_enc FROM core.browser_credentials WHERE account_id='account-a'",
            &[],
        )
        .unwrap()
        .get(0);
    assert!(!blob
        .windows(b"synthetic_secret".len())
        .any(|v| v == b"synthetic_secret"));
    assert!(cipher.decrypt_field(&blob, &aad("account-b")).is_err());
    save(&mut client, &cipher, "account-a", 0, &state).unwrap();
    assert!(save(&mut client, &cipher, "account-a", 0, &state).is_err());
    assert_eq!(
        load(&mut client, &cipher, "account-a").unwrap()["revision"],
        1
    );
    let wrong = FieldCipher::from_hex_key(&"22".repeat(32), "v1").unwrap();
    assert!(load(&mut client, &wrong, "account-a").is_err());
    client
        .execute(
            "UPDATE core.browser_credentials SET revoked_at=now() WHERE account_id='account-a'",
            &[],
        )
        .unwrap();
    assert!(load(&mut client, &cipher, "account-a").is_err());
    assert!(save(&mut client, &cipher, "account-a", 1, &state).is_err());
    assert!(save(&mut client, &cipher, "account-a", -1, &state).is_err());
    drop(client);
    admin
        .batch_execute(&format!("DROP DATABASE {database}"))
        .unwrap();
}
