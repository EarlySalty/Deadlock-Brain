BEGIN;

CREATE INDEX fact_receipt_peer ON canon.fact(receipt_id,entity_id,stat_key);
CREATE INDEX alias_receipt_peer ON canon.alias(receipt_id,entity_id,alias_norm,lang);
CREATE INDEX relation_receipt_peer ON canon.relation(receipt_id,from_entity,kind,to_entity);
CREATE INDEX entity_identity_receipt_peer ON canon.entity_identity(receipt_id,entity_id);

CREATE FUNCTION serve.receipt_is_published(p_receipt bigint) RETURNS boolean
LANGUAGE plpgsql STABLE STRICT SECURITY DEFINER SET search_path=pg_catalog,pg_temp AS $publication$
BEGIN
    IF NOT EXISTS(SELECT 1 FROM canon.receipt_quarantine WHERE cause='audited')
        AND NOT EXISTS(SELECT 1 FROM ops.receipt_audit_rejection) THEN
        RETURN EXISTS(
            SELECT 1 FROM canon.receipt r
            JOIN ops.ingest_run run ON run.id=r.ingest_run_id AND run.status='succeeded'
            WHERE r.id=p_receipt
            AND NOT EXISTS(SELECT 1 FROM canon.receipt_quarantine q WHERE q.receipt_id=r.id)
        );
    END IF;
    RETURN EXISTS(
        SELECT 1 FROM canon.receipt r
        JOIN ops.ingest_run run ON run.id=r.ingest_run_id AND run.status='succeeded'
        WHERE r.id=p_receipt
        AND NOT EXISTS(SELECT 1 FROM canon.receipt_quarantine q WHERE q.receipt_id=r.id)
        AND NOT EXISTS(SELECT 1 FROM ops.receipt_audit_rejection rejected WHERE rejected.raw_object_id=r.raw_object_id)
        AND NOT EXISTS(
            SELECT 1 FROM (
                SELECT c.receipt_id FROM canon.fact older JOIN canon.fact c USING(entity_id,stat_key)
                    WHERE older.receipt_id=r.id
                UNION ALL
                SELECT c.receipt_id FROM canon.alias older JOIN canon.alias c USING(entity_id,alias_norm,lang)
                    WHERE older.receipt_id=r.id
                UNION ALL
                SELECT c.receipt_id FROM canon.relation older JOIN canon.relation c USING(from_entity,kind,to_entity)
                    WHERE older.receipt_id=r.id
                UNION ALL
                SELECT c.receipt_id FROM canon.entity_identity older JOIN canon.entity_identity c USING(entity_id)
                    WHERE older.receipt_id=r.id
            ) peer
            JOIN canon.receipt bad ON bad.id=peer.receipt_id AND (r.ingest_run_id,r.id)<(bad.ingest_run_id,bad.id)
            JOIN ops.ingest_run published ON published.id=bad.ingest_run_id AND published.status='succeeded'
            WHERE EXISTS(SELECT 1 FROM canon.receipt_quarantine q WHERE q.receipt_id=bad.id AND q.cause='audited')
                OR EXISTS(SELECT 1 FROM ops.receipt_audit_rejection rejected WHERE rejected.raw_object_id=bad.raw_object_id)
        )
    );
END
$publication$;
REVOKE ALL ON FUNCTION serve.receipt_is_published(bigint) FROM PUBLIC,brain_ingest,brain_tool,brain_service;
GRANT EXECUTE ON FUNCTION serve.receipt_is_published(bigint) TO brain_tool,brain_service;

CREATE OR REPLACE VIEW serve.published_receipt AS
SELECT r.id,r.raw_object_id,r.ingest_run_id,r.extractor,r.locator_kind,r.locator,r.value_sha256,
    o.source,o.revision,o.revision AS source_revision,o.path,o.content_sha256
FROM canon.receipt r JOIN ops.ingest_run run ON run.id=r.ingest_run_id AND run.status='succeeded'
JOIN raw.object o ON o.id=r.raw_object_id
WHERE NOT EXISTS(SELECT 1 FROM canon.receipt_quarantine q WHERE q.receipt_id=r.id)
AND NOT EXISTS(SELECT 1 FROM ops.receipt_audit_rejection rejected WHERE rejected.raw_object_id=r.raw_object_id)
AND (
    (NOT EXISTS(SELECT 1 FROM canon.receipt_quarantine WHERE cause='audited')
        AND NOT EXISTS(SELECT 1 FROM ops.receipt_audit_rejection))
    OR serve.receipt_is_published(r.id)
);

COMMIT;
