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
