//! Ausschließlich synthetische Daten in der bestehenden isolierten Peer-Testinfrastruktur.
use brain_contracts::{guide::*, RequestDeadline, SourceRecordV2, SourceVisibility};
use brain_storage::{LocalPgReader, PgStore};
use serde::Deserialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
    time::Duration,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TestConfig {
    socket: PathBuf,
    port: u16,
    user: String,
    database: String,
}
fn config() -> TestConfig {
    // Eine normale lokale Config, kein ENV- oder Secretpfad und kein Produktionstarget.
    let file =
        std::fs::File::open("tests/guide-pg.local.json").expect("isolierte Testconfig fehlt");
    let config: TestConfig = serde_json::from_reader(file).unwrap();
    assert!(config.socket.is_absolute() && config.socket.ends_with(".core-test-pg"));
    assert!(config.database.starts_with("guide_test_") && config.port > 0);
    assert!(config.user == "brain_core_test");
    config
}
fn turn(request: &str, surface: Surface) -> GuideTurn {
    GuideTurn {
        request_id: request.into(),
        guild_id: "100".into(),
        user_id: "200".into(),
        channel_id: "300".into(),
        message_id: "400".into(),
        thread_id: None,
        reply_to_message_id: None,
        conversation_id: None,
        bot_user_id: None,
        surface,
        addressed: if surface == Surface::Dm {
            Addressed::Dm
        } else {
            Addressed::Mention
        },
        event: Event::Message,
        content: "Synthetische Frage".into(),
        control: None,
        domain: None,
        human_helped: false,
    }
}
fn control_turn(reader: &LocalPgReader, original: &GuideTurn) -> GuideTurn {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let mut event = original.clone();
    event.request_id = format!("control:{}", NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed));
    reader.guide_claim(&event, &RequestDeadline::after(Duration::from_secs(5)), 100, true).unwrap().unwrap();
    event
}
#[tokio::test]
#[ignore = "benötigt isolierten Peer-Testcluster und tests/guide-pg.local.json"]
async fn profil_isolation_datenschutz_und_ablauf_im_echten_pg_pfad() {
    let config = config();
    let options = sqlx::postgres::PgConnectOptions::new_without_pgpass()
        .host(&config.socket.to_string_lossy())
        .port(config.port)
        .username(&config.user)
        .database(&config.database)
        .password("");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .unwrap();
    sqlx::raw_sql("CREATE SCHEMA IF NOT EXISTS core; CREATE TABLE IF NOT EXISTS core.user_privacy(user_id BIGINT PRIMARY KEY,opted_out BOOLEAN NOT NULL DEFAULT false,deleted_at TIMESTAMPTZ,reason TEXT,updated_at TIMESTAMPTZ DEFAULT now())").execute(&pool).await.unwrap();
    let store = PgStore::new(pool.clone());
    store.migrate_core().await.unwrap();
    store.migrate_guide().await.unwrap();
    assert!(store.import_guide_legacy(None).await.is_err());
    migration_checks(&store, &pool).await;
    pool.close().await;
    tokio::task::spawn_blocking(move || exercise(config))
        .await
        .unwrap();
}
#[test]
#[ignore = "benötigt den abgeschlossenen synthetischen Bot-Wiedereinwilligungstest"]
fn wiederholte_einwilligung_sperrt_alte_events_in_fehlender_guild() {
    let config = config();
    assert_eq!(config.database, "guide_test_bots");
    let reader =
        LocalPgReader::new(&config.socket, config.port, &config.user, &config.database).unwrap();
    let mut sql = postgres::Config::new();
    sql.host_path(&config.socket)
        .port(config.port)
        .user(&config.user)
        .dbname(&config.database)
        .password("");
    let mut sql = sql.connect(postgres::NoTls).unwrap();
    let first_min: i64 = sql
        .query_one(
            "SELECT first_min_event_id FROM core.guide_test_consent_boundary",
            &[],
        )
        .unwrap()
        .get(0);
    assert_eq!(
        sql.query_one(
            "SELECT count(*) FROM brain.guide_subjects WHERE guild_id='101' AND user_id='5'",
            &[]
        )
        .unwrap()
        .get::<_, i64>(0),
        0
    );
    let mut event = turn("old-consent-guild", Surface::Dm);
    event.guild_id = "101".into();
    event.user_id = "5".into();
    let deadline = || RequestDeadline::after(Duration::from_secs(5));
    for (index, kind, addressed) in [
        (0, Event::Message, Addressed::Dm),
        (1, Event::TourStart, Addressed::TourButton),
        (2, Event::Message, Addressed::Command),
    ] {
        event.request_id = format!("old-consent-guild-{index}");
        event.message_id = (first_min - 1).to_string();
        event.event = kind;
        event.addressed = addressed;
        assert!(reader
            .guide_claim(&event, &deadline(), 1300, true)
            .unwrap()
            .is_none());
    }
    let fresh_id: i64 = sql.query_one("SELECT ((floor(extract(epoch from clock_timestamp())*1000)::bigint+2)-1420070400000)*4194304",&[]).unwrap().get(0);
    event.request_id = "new-consent-guild".into();
    event.message_id = fresh_id.to_string();
    let fresh = reader
        .guide_claim(&event, &deadline(), 1300, true)
        .unwrap()
        .unwrap();
    assert!(!fresh.profile.deleted && !fresh.profile.globally_opted_out);
    assert!(!fresh.profile.memory_enabled && !fresh.profile.contact_enabled);
    assert!(fresh.profile.fields.is_empty() && fresh.history.is_empty());
}

fn exercise(config: TestConfig) {
    let reader =
        LocalPgReader::new(&config.socket, config.port, &config.user, &config.database).unwrap();
    let mut sql = postgres::Config::new();
    sql.host_path(&config.socket)
        .port(config.port)
        .user(&config.user)
        .dbname(&config.database)
        .password("");
    let mut sql = sql.connect(postgres::NoTls).unwrap();
    // Der Testcluster muss leer sein; keine fremden Fixtures still überschreiben.
    assert_eq!(
        sql.query_one("SELECT count(*) FROM brain.guide_subjects", &[])
            .unwrap()
            .get::<_, i64>(0),
        0
    );
    let deadline = || RequestDeadline::after(Duration::from_secs(5));
    let dm = turn("dm:1", Surface::Dm);
    let first = reader
        .guide_claim(&dm, &deadline(), 100, true)
        .unwrap()
        .unwrap();
    assert!(!first.profile.memory_enabled);
    assert!(reader
        .guide_claim(&dm, &deadline(), 100, true)
        .unwrap()
        .is_none());
    assert!(reader
        .guide_control(
            &control_turn(&reader, &dm),
            &ProfileControl::Memory { enabled: true },
            100,
            None,
            &deadline()
        )
        .is_err());
    reader
        .guide_control(
            &dm,
            &ProfileControl::Memory { enabled: true },
            100,
            Some(1000),
            &deadline(),
        )
        .unwrap();
    let profile = reader
        .guide_control(
            &dm,
            &ProfileControl::Correct {
                field: ProfileField::PlayTimes,
                value: "Abends".into(),
            },
            100,
            Some(1000),
            &deadline(),
        )
        .unwrap();
    assert!(profile.fields[&ProfileField::PlayTimes].explicitly_stated);
    let private = reader
        .guide_claim(&turn("dm:2", Surface::Dm), &deadline(), 101, true)
        .unwrap()
        .unwrap();
    assert_eq!(
        private.profile.fields[&ProfileField::PlayTimes].value,
        "Abends"
    );
    let no_retention = reader
        .guide_claim(&turn("dm:3", Surface::Dm), &deadline(), 101, false)
        .unwrap()
        .unwrap();
    assert!(no_retention.profile.fields.is_empty() && no_retention.history.is_empty());
    let public = reader
        .guide_claim(&turn("public:1", Surface::Public), &deadline(), 101, true)
        .unwrap()
        .unwrap();
    assert!(public.profile.fields.is_empty() && public.history.is_empty());
    let mut other = turn("dm:other", Surface::Dm);
    other.user_id = "201".into();
    assert!(reader
        .guide_claim(&other, &deadline(), 101, true)
        .unwrap()
        .unwrap()
        .profile
        .fields
        .is_empty());
    let epoch = private.profile.epoch;
    let history = vec![GuideHistory {
        role: "user".into(),
        content: "Privates synthetisches Merkmal".into(),
        message_id: "400".into(),
        expires_at: 1100,
    }];
    reader
        .guide_control(
            &dm,
            &ProfileControl::Memory { enabled: false },
            102,
            Some(1000),
            &deadline(),
        )
        .unwrap();
    assert!(!reader
        .guide_finish(&dm, epoch, None, &history, None, &deadline())
        .unwrap());
    assert!(reader
        .guide_claim(&turn("dm:4", Surface::Dm), &deadline(), 103, true)
        .unwrap()
        .unwrap()
        .history
        .is_empty());
    reader
        .guide_control(
            &dm,
            &ProfileControl::Memory { enabled: true },
            104,
            Some(1000),
            &deadline(),
        )
        .unwrap();
    reader
        .guide_control(
            &dm,
            &ProfileControl::Correct {
                field: ProfileField::CurrentGoals,
                value: "Mitspieler finden".into(),
            },
            104,
            Some(1000),
            &deadline(),
        )
        .unwrap();
    reader.guide_cleanup(1200, 86400, &deadline()).unwrap();
    assert!(reader
        .guide_claim(&turn("dm:5", Surface::Dm), &deadline(), 1201, true)
        .unwrap()
        .unwrap()
        .profile
        .fields
        .is_empty());
    let before = reader
        .guide_claim(&turn("dm:6", Surface::Dm), &deadline(), 1201, true)
        .unwrap()
        .unwrap();
    reader
        .guide_feedback_draft(&dm, 1201, 600, true, &deadline())
        .unwrap();
    let forgotten = reader
        .guide_control(&control_turn(&reader, &dm), &ProfileControl::Forget, 1202, Some(1000), &deadline())
        .unwrap();
    assert!(forgotten.deleted && !forgotten.memory_enabled && forgotten.fields.is_empty());
    assert!(!reader
        .guide_finish(&dm, before.profile.epoch, None, &history, None, &deadline())
        .unwrap());
    assert!(reader
        .guide_feedback_draft(&dm, 1203, 0, false, &deadline())
        .unwrap()
        .is_none());
    assert!(reader
        .guide_control(
            &dm,
            &ProfileControl::Memory { enabled: true },
            1203,
            Some(1000),
            &deadline()
        )
        .is_err());
    sql.execute(
        "INSERT INTO core.user_privacy(user_id,opted_out) VALUES(201,true)",
        &[],
    )
    .unwrap();
    other.request_id = "dm:other-optout".into();
    let blocked = reader
        .guide_claim(&other, &deadline(), 1204, true)
        .unwrap()
        .unwrap();
    assert!(blocked.profile.globally_opted_out && blocked.profile.fields.is_empty());
    assert!(reader
        .guide_control(
            &other,
            &ProfileControl::Memory { enabled: true },
            1204,
            Some(1000),
            &deadline()
        )
        .is_err());
    let mut feedback = turn("feedback:1", Surface::Dm);
    feedback.user_id = "202".into();
    let subject = reader
        .guide_claim(&feedback, &deadline(), 1300, true)
        .unwrap()
        .unwrap();
    assert!(reader
        .guide_finish(
            &feedback,
            subject.profile.epoch,
            None,
            &[],
            Some(("feedback-test", "500", "Ein synthetischer Serverwunsch")),
            &deadline()
        )
        .unwrap());
    let mut ack = ActionResult {
        request_id: "feedback:1".into(),
        guild_id: "100".into(),
        user_id: "202".into(),
        delivery_id: "feedback-test".into(),
        success: true,
        sent_message_id: None,
        reply_message_id: None,
    };
    assert!(reader.guide_action_result(&ack, &deadline()).is_err());
    ack.success = false;
    assert_eq!(
        reader.guide_action_result(&ack, &deadline()).unwrap(),
        Some(subject.profile.epoch)
    );
    ack.success = true;
    ack.sent_message_id = Some("501".into());
    assert!(reader
        .guide_action_result(&ack, &deadline())
        .unwrap()
        .is_none());
    assert_eq!(
        sql.query_one(
            "SELECT state,text FROM brain.guide_feedback_outbox WHERE delivery_id='feedback-test'",
            &[]
        )
        .unwrap()
        .get::<_, String>(0),
        "failed"
    );
    assert!(sql.query_one("SELECT text IS NULL FROM brain.guide_feedback_outbox WHERE delivery_id='feedback-test'",&[]).unwrap().get::<_,bool>(0));
    sql.execute("INSERT INTO brain.guide_feedback_outbox(guild_id,user_id,delivery_id,destination_channel_id,text,state,expires_at) VALUES('100','202','feedback-expired','500','Synthetisches Anliegen','pending',to_timestamp(1200))",&[]).unwrap();
    reader.guide_cleanup(1300, 86400, &deadline()).unwrap();
    let expired = sql.query_one("SELECT state,text IS NULL FROM brain.guide_feedback_outbox WHERE delivery_id='feedback-expired'",&[]).unwrap();
    assert_eq!(expired.get::<_, String>(0), "failed");
    assert!(expired.get::<_, bool>(1));
    ack.delivery_id = "feedback-expired".into();
    assert!(reader
        .guide_action_result(&ack, &deadline())
        .unwrap()
        .is_none());
    sql.execute("UPDATE brain.guide_subjects SET min_event_id=1000,epoch=epoch+1,deleted=false,globally_opted_out=false,profile_json='{}',history_json='[]' WHERE user_id='202'",&[]).unwrap();
    let mut fresh = feedback.clone();
    fresh.request_id = "old-after-optin".into();
    fresh.message_id = "999".into();
    assert!(reader
        .guide_claim(&fresh, &deadline(), 1300, true)
        .unwrap()
        .is_none());
    fresh.request_id = "new-after-optin".into();
    fresh.message_id = "1000".into();
    let restarted = reader
        .guide_claim(&fresh, &deadline(), 1300, true)
        .unwrap()
        .unwrap();
    assert!(restarted.profile.epoch > subject.profile.epoch);
    assert!(restarted.profile.fields.is_empty() && restarted.history.is_empty());
    assert!(!restarted.profile.memory_enabled);
    assert!(!reader
        .guide_finish(
            &feedback,
            subject.profile.epoch,
            None,
            &[],
            None,
            &deadline()
        )
        .unwrap());
    for (index, event, addressed) in [
        (0, Event::TourStart, Addressed::TourButton),
        (1, Event::Message, Addressed::Command),
        (2, Event::Message, Addressed::Dm),
    ] {
        fresh.request_id = format!("old-event-{index}");
        fresh.message_id = "999".into();
        fresh.event = event;
        fresh.addressed = addressed;
        assert!(reader
            .guide_claim(&fresh, &deadline(), 1300, true)
            .unwrap()
            .is_none());
        fresh.request_id = format!("new-event-{index}");
        fresh.message_id = "1001".into();
        assert!(reader
            .guide_claim(&fresh, &deadline(), 1300, true)
            .unwrap()
            .is_some());
    }
    sequence_checks(&reader, &mut sql);
    let mut snapshot = ServerSnapshot {
        guild_id: "100".into(),
        revision: 1,
        observed_at: 1300,
        channels: vec![ServerChannel {
            id: "500".into(),
            name: "Regeln".into(),
            kind: "text".into(),
            public_readable: true,
            deleted: false,
        }],
        roles: vec![],
        rules: vec![ServerRule {
            channel_id: "500".into(),
            message_id: "501".into(),
            text: "Synthetische öffentliche Regel".into(),
            updated_at: 1300,
            deleted: false,
        }],
    };
    let record = |snapshot: &ServerSnapshot| SourceRecordV2 {
        source_id: "guide-discord:100".into(),
        logical_id: "server".into(),
        revision: snapshot.revision,
        content_hash: format!("{:064x}", snapshot.revision),
        content: serde_json::to_string(snapshot).unwrap(),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::from(["bot.public".into()]),
        tombstone: false,
        valid_from: Some("1300".into()),
        valid_to: Some("1600".into()),
        metadata: BTreeMap::from([("source_class".into(), "server_documentation".into())]),
    };
    reader
        .guide_sync_server(&snapshot, &record(&snapshot), &deadline())
        .unwrap();
    snapshot.revision = 2;
    snapshot.channels[0].public_readable = false;
    snapshot.rules.clear();
    reader
        .guide_sync_server(&snapshot, &record(&snapshot), &deadline())
        .unwrap();
    assert_eq!(
        reader
            .guide_server_record("100", &deadline())
            .unwrap()
            .unwrap()
            .revision,
        2
    );
    assert_eq!(sql.query_one("SELECT count(*) FROM brain.source_record_revisions WHERE source_id='guide-discord:100'",&[]).unwrap().get::<_,i64>(0),1);
    snapshot.revision = 1;
    assert!(reader
        .guide_sync_server(&snapshot, &record(&snapshot), &deadline())
        .is_err());
}

fn sequence_checks(reader: &LocalPgReader, sql: &mut postgres::Client) {
    let deadline = || RequestDeadline::after(Duration::from_secs(5));
    let mut a = turn("sequence-a", Surface::Dm);
    a.user_id = "210".into();
    let first = reader.guide_claim(&a, &deadline(), 1300, true).unwrap().unwrap();
    let mut b = a.clone(); b.request_id = "sequence-b".into();
    let second = reader.guide_claim(&b, &deadline(), 1300, true).unwrap().unwrap();
    assert!(reader.guide_claim(&a, &deadline(), 1300, true).unwrap().is_none());
    let conv = GuideConversation { id: "sequence-conv".into(), channel_id: a.channel_id.clone(), thread_id: None, user_id: a.user_id.clone(), surface: Surface::Dm, last_user_message_id: a.message_id.clone(), last_bot_message_id: None, expires_at: 1900, closed: false };
    assert!(reader.guide_finish(&b, second.profile.epoch, Some(&conv), &[], None, &deadline()).unwrap());
    assert!(!reader.guide_finish(&a, first.profile.epoch, Some(&conv), &[], None, &deadline()).unwrap());
    let mut ack = ActionResult { request_id: "sequence-b:reply".into(), guild_id: a.guild_id.clone(), user_id: a.user_id.clone(), delivery_id: "reply:sequence-conv".into(), success: true, sent_message_id: None, reply_message_id: Some("777".into()) };
    reader.guide_action_result(&ack, &deadline()).unwrap();
    ack.request_id = "sequence-a:reply".into(); ack.reply_message_id = Some("778".into());
    reader.guide_action_result(&ack, &deadline()).unwrap();
    ack.request_id = "sequence-b:reply".into();
    reader.guide_action_result(&ack, &deadline()).unwrap();
    let saved: serde_json::Value = sql.query_one("SELECT state_json FROM brain.guide_conversations WHERE user_id='210'", &[]).unwrap().get(0);
    assert_eq!(saved["last_bot_message_id"], "777");
    let mut control = a.clone(); control.request_id = "sequence-control".into();
    reader.guide_claim(&control, &deadline(), 1300, true).unwrap().unwrap();
    b.request_id = "sequence-later-normal".into();
    let later = reader.guide_claim(&b, &deadline(), 1300, true).unwrap().unwrap();
    let off = reader.guide_control(&control, &ProfileControl::Memory { enabled: false }, 1300, None, &deadline()).unwrap();
    assert!(off.epoch > later.profile.epoch);
    assert!(!reader.guide_finish(&b, later.profile.epoch, None, &[], None, &deadline()).unwrap());
    assert!(reader.guide_control(&control, &ProfileControl::Memory { enabled: false }, 1300, None, &deadline()).is_err());
    control.request_id = "sequence-before-consent".into();
    reader.guide_claim(&control, &deadline(), 1300, true).unwrap().unwrap();
    sql.execute("UPDATE brain.guide_subjects SET epoch=epoch+1 WHERE user_id='210'", &[]).unwrap();
    assert!(reader.guide_control(&control, &ProfileControl::Forget, 1300, None, &deadline()).is_err());
}

async fn migration_checks(store: &PgStore, pool: &sqlx::PgPool) {
    for index in 0..257 {
        let record = SourceRecordV2 { source_id: format!("text-source-{index:04}"), logical_id: "document".into(), revision: 1, content_hash: "a".repeat(64), content: "Synthetischer Datensatz".into(), visibility: SourceVisibility::Public, allowed_scopes: BTreeSet::from(["bot.public".into()]), tombstone: false, valid_from: None, valid_to: None, metadata: BTreeMap::new() };
        store.apply(&record).await.unwrap();
    }
    for index in 0..65 {
        let release = brain_contracts::CorpusRelease { release_id: format!("text-release-{index:04}"), knowledge_version: "synthetisch".into(), patch: "synthetisch".into(), created_at_epoch: 1, source_revisions: BTreeMap::new() };
        store.publish_release(&release).await.unwrap();
    }
    store.migrate_core().await.unwrap();
    sqlx::raw_sql(r#"CREATE SCHEMA bot; CREATE TABLE bot.concierge_profiles(user_id BIGINT,guild_id BIGINT,opted_out BOOLEAN,forgot_at TIMESTAMPTZ,play_times TEXT,updated_at TIMESTAMPTZ); CREATE TABLE bot.concierge_conversations(user_id BIGINT,guild_id BIGINT,role TEXT,content TEXT,created_at TIMESTAMPTZ);
        INSERT INTO bot.concierge_profiles VALUES(220,100,false,NULL,'Alt',now()),(221,100,false,NULL,'Frisch',now()),(222,100,true,now(),'Gesperrt',now());
        INSERT INTO brain.guide_subjects(guild_id,user_id,memory_enabled,profile_json) VALUES('100','220',true,'{"play_times":{"value":"Korrigiert"}}');
        INSERT INTO core.user_privacy(user_id,opted_out,reason) VALUES(222,false,'user_opt_in');
        INSERT INTO brain.guide_subjects(guild_id,user_id) VALUES('100','222')"#)
        .execute(pool).await.unwrap();
    store.migrate_guide().await.unwrap();
    let reopened: bool = sqlx::query_scalar("SELECT NOT deleted AND NOT globally_opted_out FROM brain.guide_subjects WHERE user_id='222'").fetch_one(pool).await.unwrap();
    assert!(reopened);
    store.import_guide_legacy(Some(1000)).await.unwrap();
    store.import_guide_legacy(Some(1000)).await.unwrap();
    let unchanged: bool = sqlx::query_scalar("SELECT memory_enabled AND profile_json->'play_times'->>'value'='Korrigiert' AND history_json='[]'::jsonb FROM brain.guide_subjects WHERE user_id='220'").fetch_one(pool).await.unwrap();
    assert!(unchanged);
    let imported: bool = sqlx::query_scalar("SELECT NOT memory_enabled AND NOT contact_enabled AND epoch=1 AND NOT legacy_import_eligible AND profile_json->'play_times'->>'value'='Frisch' FROM brain.guide_subjects WHERE user_id='221'").fetch_one(pool).await.unwrap();
    assert!(imported);
    sqlx::raw_sql("DELETE FROM brain.guide_subjects WHERE user_id IN ('220','221','222'); DELETE FROM core.user_privacy WHERE user_id=222; DROP TABLE bot.concierge_profiles; DROP TABLE bot.concierge_conversations")
        .execute(pool).await.unwrap();
}
