#[path = "../../dbrain-wiki/tests/support/c5.rs"]
mod fixture;
use brain_contracts::{
    domain::{DomainObject, DomainStorePort, StoredDomainObject, Validity},
    source::origin_from_record,
    AuthorizedContext, Budget, DocumentRevision, Principal, SourceVisibility,
};
use brain_storage::{DomainReader, LocalPgReader, PgStore};
use dbrain_s12_wiki_probe::knowledge::{self, ProjectionReview, WikiIr};
use dbrain_sources::wiki_runtime::{plan_release, ReleaseRequest, ScratchWikiStore};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};
fn request(ir: &WikiIr, id: &str) -> ReleaseRequest {
    let mut pins = BTreeMap::<String, DocumentRevision>::new();
    for source in ir.sources() {
        let r = DocumentRevision {
            source_id: source.source_id.clone(),
            logical_id: source.logical_id.clone(),
            revision: source.revision,
            content_hash: source.content_hash.clone(),
        };
        if pins
            .get(&r.logical_id)
            .is_none_or(|old| old.revision < r.revision)
        {
            pins.insert(r.logical_id.clone(), r);
        }
    }
    ReleaseRequest {
        release_id: id.into(),
        knowledge_version: "c5-fixture-v1".into(),
        validity: Validity {
            patch: "synthetic-p1".into(),
            mode: "review".into(),
        },
        created_at_epoch: fixture::options().retrieved_at,
        review: Some(ProjectionReview {
            decision_ref: "synthetic field and closure review".into(),
            approved_fields: ir
                .fields()
                .iter()
                .map(|f| (f.id.clone(), f.source_revision.clone()))
                .collect(),
            source_revisions: pins,
        }),
    }
}
// The production synchronous reader runs at a blocking-worker boundary, not
// inside an async executor's current thread. Exercise that same boundary here,
// including teardown: pooled postgres::Client values retain their own Tokio
// runtime and must be dropped outside the async test runtime too.
struct BlockingDomainReader(Option<DomainReader<LocalPgReader>>);
impl BlockingDomainReader {
    fn new(reader: LocalPgReader) -> Self {
        Self(Some(DomainReader::new(reader)))
    }

    fn read_domain(
        &self,
        context: &AuthorizedContext,
        validity: &Validity,
    ) -> Result<brain_contracts::domain::DomainSnapshot, brain_contracts::PortError> {
        let reader = self.0.as_ref().expect("blocking domain reader available");
        std::thread::scope(|scope| {
            scope
                .spawn(|| reader.read_domain(context, validity))
                .join()
                .unwrap()
        })
    }
}
impl Drop for BlockingDomainReader {
    fn drop(&mut self) {
        let Some(reader) = self.0.take() else {
            return;
        };
        std::thread::scope(|scope| {
            scope.spawn(move || drop(reader)).join().unwrap();
        });
    }
}
fn context(release: &str) -> AuthorizedContext {
    AuthorizedContext {
        principal: Principal {
            actor_id: "c5-reviewer".into(),
            channel: "test".into(),
            scopes: ["wiki.review".into()].into(),
            provider_egress: BTreeSet::new(),
        },
        conversation_id: "c5-test".into(),
        knowledge_release: release.into(),
        deadline_ms: 5000,
        budget: Budget::default(),
    }
}
#[test]
fn shared_ir_fact_release_plan_is_loss_aware_and_review_pinned() {
    let ir = knowledge::extract(&fixture::bytes(101, 201), &fixture::mapping()).unwrap();
    let req = request(&ir, "c5-plan");
    let plan = plan_release(&ir, &req).unwrap();
    assert_eq!(plan.raw_records.len(), 8);
    assert_eq!(plan.ir_records.len(), 4);
    assert_eq!(plan.fact_records.len(), 3);
    assert_eq!(plan.withheld_fields.len(), 2);
    for record in &plan.fact_records {
        let origin = origin_from_record(record).unwrap();
        assert!(!origin.policy.publication_allowed);
        assert!(!origin.policy.provider_egress_allowed);
        assert!(origin.policy.raw_retention_allowed);
        let object: StoredDomainObject = serde_json::from_str(&record.content).unwrap();
        let DomainObject::NumericFact(fact) = object.object else {
            panic!()
        };
        assert!(fact.verified);
        assert_eq!(fact.validity.patch, "synthetic-p1");
        assert!(record.metadata.contains_key("wiki_field"));
    }
    let mut stale = req.clone();
    for pin in stale.review.as_mut().unwrap().approved_fields.values_mut() {
        pin.revision -= 1;
    }
    assert!(plan_release(&ir, &stale).unwrap().fact_records.is_empty());
    let mut incomplete = req.clone();
    incomplete
        .review
        .as_mut()
        .unwrap()
        .source_revisions
        .retain(|id, _| !id.ends_with(":2"));
    assert_eq!(
        plan_release(&ir, &incomplete).unwrap().fact_records.len(),
        1
    );
    let mut absent = req;
    absent.review = None;
    assert!(plan_release(&ir, &absent).unwrap().fact_records.is_empty());
}
#[test]
fn lua_programs_and_template_expressions_are_not_executed_or_promoted() {
    for (model, content) in [
        ("Scribunto", "return os.execute('touch /tmp/not-executed')"),
        ("wikitext", "{{#invoke:Stats|execute}}"),
    ] {
        let mut cap = serde_json::to_value(fixture::capture(101, 201)).unwrap();
        let page = cap["pages"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|p| p["page_id"] == 1)
            .unwrap();
        let slot = &mut page["response"]["query"]["pages"][0]["revisions"][0]["slots"]["main"];
        slot["contentmodel"] = json!(model);
        slot["content"] = json!(content);
        let ir =
            knowledge::extract(&serde_json::to_vec(&cap).unwrap(), &fixture::mapping()).unwrap();
        let plan = plan_release(&ir, &request(&ir, "c5-untrusted")).unwrap();
        assert!(plan
            .fact_records
            .iter()
            .all(|r| !r.logical_id.starts_with("c5_synthetic_wiki:page:1:")));
        assert!(ir
            .fields()
            .iter()
            .filter(|f| f.source_revision.logical_id.ends_with(":1"))
            .all(|f| !f.unknowns.is_empty()));
    }
}
#[test]
fn raw_retention_and_scope_are_mandatory_for_store_plan() {
    let mut cap = fixture::capture(101, 201);
    cap.policy.raw_retention_allowed = false;
    let ir = knowledge::extract(&serde_json::to_vec(&cap).unwrap(), &fixture::mapping()).unwrap();
    assert!(plan_release(&ir, &request(&ir, "c5-denied")).is_err());
}

#[test]
fn transitive_template_changes_leave_unrelated_ir_and_facts_unchanged() {
    let before = knowledge::extract(&fixture::bytes(101, 201), &fixture::mapping()).unwrap();
    let after = knowledge::extract(&fixture::bytes(101, 202), &fixture::mapping()).unwrap();
    let a = plan_release(&before, &request(&before, "a")).unwrap();
    let b = plan_release(&after, &request(&after, "b")).unwrap();
    for id in [1, 2, 3, 4] {
        let prefix = format!("c5_synthetic_wiki:page:{id}:ir:");
        let old = a
            .ir_records
            .iter()
            .find(|r| r.logical_id.starts_with(&prefix))
            .unwrap();
        let new = b
            .ir_records
            .iter()
            .find(|r| r.logical_id.starts_with(&prefix))
            .unwrap();
        assert_eq!(old.logical_id == new.logical_id, id == 4);
    }
    let unrelated = |p: &dbrain_sources::wiki_runtime::WikiReleasePlan| {
        p.fact_records
            .iter()
            .find(|r| r.logical_id.starts_with("c5_synthetic_wiki:page:4:fact:"))
            .unwrap()
            .logical_id
            .clone()
    };
    assert_eq!(unrelated(&a), unrelated(&b));
}

#[test]
fn quarantined_template_blocks_dependent_facts_without_executing_source() {
    let mut cap = serde_json::to_value(fixture::capture(101, 201)).unwrap();
    let template = cap["pages"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|p| p["page_id"] == 2)
        .unwrap();
    let slot = &mut template["response"]["query"]["pages"][0]["revisions"][0]["slots"]["main"];
    slot["contentmodel"] = json!("Scribunto");
    slot["content"] = json!("return require('untrusted').calculate()");
    let ir = knowledge::extract(&serde_json::to_vec(&cap).unwrap(), &fixture::mapping()).unwrap();
    let plan = plan_release(&ir, &request(&ir, "untrusted-template")).unwrap();
    assert_eq!(plan.fact_records.len(), 1);
    assert!(plan.fact_records[0]
        .logical_id
        .starts_with("c5_synthetic_wiki:page:4:"));
}

#[tokio::test]
async fn scratch_guard_rejects_arbitrary_paths_before_a_database_connection() {
    let dir = tempfile::tempdir().unwrap();
    assert!(
        ScratchWikiStore::connect(dir.path(), &dir.path().join("raw"))
            .await
            .is_err()
    );
    assert!(!dir.path().join("raw").exists());
}

#[tokio::test]
#[ignore = "dedicated Unix-socket scratch cluster; run scripts/test_wiki_runtime.sh"]
async fn scratch_raw_ir_facts_release_delta_reparse_and_acl() {
    let socket = std::env::var("BRAIN_C5_TEST_SOCKET").expect("use the scratch test wrapper");
    let out = std::env::var("BRAIN_C5_TEST_OUTPUT").expect("scratch output");
    let store = ScratchWikiStore::connect(Path::new(&socket), &Path::new(&out).join("raw"))
        .await
        .unwrap();
    store.migrate().await.unwrap();
    let mapping = fixture::mapping();
    let bytes = fixture::bytes(101, 201);
    let ir = knowledge::extract(&bytes, &mapping).unwrap();
    let req = request(&ir, "c5-db-r1");
    let first = store.stage(&ir, &req).await.unwrap();
    assert_eq!(first["fact_records"], 3);
    assert_eq!(first["raw_staging"]["raw_revisions"], 8);
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM brain.source_documents")
        .fetch_one(store.pool())
        .await
        .unwrap();
    assert_eq!(count, 8);
    let metadata: Value =
        sqlx::query_scalar("SELECT metadata FROM brain.source_documents ORDER BY id LIMIT 1")
            .fetch_one(store.pool())
            .await
            .unwrap();
    assert_eq!(metadata["source_record"]["visibility"], "internal");
    assert!(metadata["source_record"].get("content").is_none());
    let reader = LocalPgReader::new(&socket, 55441, "brain_wiki_c5", "brain_wiki_c5").unwrap();
    let domain = BlockingDomainReader::new(reader);
    let snapshot = domain
        .read_domain(&context("c5-db-r1"), &req.validity)
        .unwrap();
    assert_eq!(snapshot.facts.len(), 3);
    let mut denied = context("c5-db-r1");
    denied.principal.scopes.clear();
    assert!(domain
        .read_domain(&denied, &req.validity)
        .unwrap()
        .facts
        .is_empty());
    // Later retrieval of the SAME revisions preserves first-observation raw/IR.
    let mut same = fixture::capture(101, 201);
    same.retrieved_at += 60;
    let same = knowledge::extract(&serde_json::to_vec(&same).unwrap(), &mapping).unwrap();
    let second = store.stage(&same, &req).await.unwrap();
    assert_eq!(second["inserted_records"], 0);
    let unchanged = knowledge::compare_ir(&ir, &same).unwrap();
    assert!(unchanged.reparse_pages.is_empty());
    let changed = knowledge::extract(&fixture::bytes(102, 201), &mapping).unwrap();
    let changed_req = request(&changed, "c5-db-r2");
    store.stage(&changed, &changed_req).await.unwrap();
    let latest = domain
        .read_domain(&context("c5-db-r2"), &req.validity)
        .unwrap();
    assert!(latest
        .facts
        .iter()
        .any(|f| f.subject_id == "hero:pilot" && f.quantity.value == 650.0));
    assert!(domain
        .read_domain(&context("c5-db-r1"), &req.validity)
        .unwrap()
        .facts
        .iter()
        .any(|f| f.subject_id == "hero:pilot" && f.quantity.value == 600.0));
    let rows:i64=sqlx::query_scalar("SELECT count(*) FROM brain.source_record_revisions WHERE logical_id='c5_synthetic_wiki:page:1'").fetch_one(store.pool()).await.unwrap();
    assert_eq!(rows, 3);
    // Reparse with a new reviewed mapping generation does not recapture or rewrite raw.
    let mut reparsed_mapping = mapping;
    reparsed_mapping.version = "c5-mapping-v2".into();
    let reparsed = knowledge::extract(&fixture::bytes(102, 201), &reparsed_mapping).unwrap();
    store
        .stage(&reparsed, &request(&reparsed, "c5-db-reparse"))
        .await
        .unwrap();
    let rows_after:i64=sqlx::query_scalar("SELECT count(*) FROM brain.source_record_revisions WHERE logical_id='c5_synthetic_wiki:page:1'").fetch_one(store.pool()).await.unwrap();
    assert_eq!(rows_after, rows);
    // Source rights remain in both stores and immutable raw files are hash checked.
    let artifacts: Vec<(String, String)> =
        sqlx::query_as("SELECT raw_path,content_hash FROM brain.source_documents")
            .fetch_all(store.pool())
            .await
            .unwrap();
    for (path, hash) in artifacts {
        assert_eq!(
            dbrain_s12_wiki_probe::sha256(&std::fs::read(path).unwrap()),
            hash
        );
    }
    // Dependency ACLs are checked too: a hidden Template invalidates its Hero
    // fact even while the Hero's own raw revision remains visible.
    let mut dependency = changed
        .sources()
        .iter()
        .find(|s| s.logical_id.ends_with(":2") && s.revision == 201)
        .unwrap()
        .clone();
    let mut dependency_origin = origin_from_record(&dependency).unwrap();
    dependency.revision = 202;
    dependency.visibility = SourceVisibility::Private;
    dependency.allowed_scopes = ["wiki.denied".into()].into();
    dependency_origin.source_revision = brain_contracts::source::SourceRevision::Wiki {
        page_id: 2,
        revision_id: 202,
    };
    dependency_origin.policy.visibility = dependency.visibility;
    dependency_origin.policy.allowed_scopes = dependency.allowed_scopes.clone();
    dependency_origin.bind_record(&mut dependency).unwrap();
    PgStore::new(store.pool().clone())
        .apply(&dependency)
        .await
        .unwrap();
    let dependent_denial = domain
        .read_domain(&context("c5-db-r1"), &req.validity)
        .unwrap();
    assert_eq!(dependent_denial.facts.len(), 1);
    assert_eq!(dependent_denial.facts[0].subject_id, "hero:other");

    // Current ACL revocation hides derived facts even in OLD releases.
    let mut raw = changed
        .sources()
        .iter()
        .find(|s| s.logical_id.ends_with(":1") && s.revision == 102)
        .unwrap()
        .clone();
    raw.revision = 103;
    raw.visibility = SourceVisibility::Private;
    raw.allowed_scopes = ["wiki.denied".into()].into();
    let mut origin = origin_from_record(
        changed
            .sources()
            .iter()
            .find(|s| s.logical_id.ends_with(":1") && s.revision == 102)
            .unwrap(),
    )
    .unwrap();
    origin.source_revision = brain_contracts::source::SourceRevision::Wiki {
        page_id: 1,
        revision_id: 103,
    };
    origin.policy.visibility = raw.visibility;
    origin.policy.allowed_scopes = raw.allowed_scopes.clone();
    origin.bind_record(&mut raw).unwrap();
    PgStore::new(store.pool().clone())
        .apply(&raw)
        .await
        .unwrap();
    assert!(domain
        .read_domain(&context("c5-db-r1"), &req.validity)
        .unwrap()
        .facts
        .iter()
        .all(|f| f.subject_id != "hero:pilot"));
    std::fs::write(Path::new(&out).join("db-receipt.json"),serde_json::to_vec_pretty(&json!({"first":first,"unchanged":second,"checked":"Raw -> common IR -> PgStore -> DomainStorePort / CorpusRelease; revision/delta/reparse/ACL"})).unwrap()).unwrap();
    // Export deterministic synthetic inputs for the CLI smoke test, not production fixtures.
    assert!(store
        .stage(&changed, &request(&changed, "c5-stale-after-revocation"))
        .await
        .is_err());
    let mut cli_capture = fixture::capture(101, 201);
    cli_capture.source_key = "c5_cli_synthetic".into();
    let mut cli_mapping = fixture::mapping();
    cli_mapping.source_key = cli_capture.source_key.clone();
    let cli_bytes = serde_json::to_vec(&cli_capture).unwrap();
    let cli_ir = knowledge::extract(&cli_bytes, &cli_mapping).unwrap();
    std::fs::write(Path::new(&out).join("fixture.capture.json"), cli_bytes).unwrap();
    std::fs::write(
        Path::new(&out).join("fixture.mapping.json"),
        serde_json::to_vec_pretty(&cli_mapping).unwrap(),
    )
    .unwrap();
    std::fs::write(
        Path::new(&out).join("fixture.release.json"),
        serde_json::to_vec_pretty(&request(&cli_ir, "c5-cli-r1")).unwrap(),
    )
    .unwrap();
}

#[tokio::test]
async fn normal_store_stage_is_idempotent_versioned_and_needs_no_second_store() {
    use brain_contracts::SnapshotReadPort;
    use brain_storage::MemoryRepository;
    use dbrain_sources::wiki_runtime::stage_into_store;
    let store = MemoryRepository::default();
    let mapping = fixture::mapping();
    let ir = knowledge::extract(&fixture::bytes(101, 201), &mapping).unwrap();
    let req = request(&ir, "c5-core-r1");
    let first = stage_into_store(&store, &ir, &req, "wiki-core")
        .await
        .unwrap();
    assert_eq!(first["committed_records"], 15);
    assert_eq!(first["fact_records"], 3);
    assert_eq!(first["second_store"], false);
    assert!(first["batches"].as_u64().unwrap() >= 2);
    let snapshot = store.read_snapshot("c5-core-r1").unwrap();
    let facts = snapshot
        .revisions
        .iter()
        .filter(|r| r.metadata.contains_key("wiki_field"))
        .count();
    assert_eq!(facts, 3);
    let again = stage_into_store(&store, &ir, &req, "wiki-core")
        .await
        .unwrap();
    assert_eq!(again["committed_records"], 0);
    assert_eq!(again["unchanged_records"], 15);
    let changed = knowledge::extract(&fixture::bytes(102, 201), &mapping).unwrap();
    let changed_req = request(&changed, "c5-core-r2");
    let delta = stage_into_store(&store, &changed, &changed_req, "wiki-core")
        .await
        .unwrap();
    assert!(delta["committed_records"].as_u64().unwrap() > 0);
    assert!(delta["unchanged_records"].as_u64().unwrap() > 0);
    assert!(store.read_snapshot("c5-core-r2").is_ok());
    assert!(
        stage_into_store(&store, &ir, &request(&ir, "c5-core-stale"), "wiki-core")
            .await
            .is_err()
    );
}
