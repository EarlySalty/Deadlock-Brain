use super::LocalPgReader;
use brain_contracts::{
    response_audit::{ResponseAuditPort, ResponseDeviation, ResponseDisposition},
    PortError,
};

impl LocalPgReader {
    pub fn check_response_audit(&self) -> Result<(), PortError> {
        let mut client = self.pool.acquire()?;
        let shape = client.query_one(crate::schema::RESPONSE_AUDIT_SHAPE_PROBE, &[])?;
        if !shape.try_get::<_, bool>(0).map_err(super::error)? {
            return Err(super::invalid("incompatible response audit schema"));
        }
        let row = client.query_one("SELECT to_regclass('brain.response_deviations_v1') IS NOT NULL AND has_table_privilege(current_user, 'brain.response_deviations_v1', 'INSERT') AND has_column_privilege(current_user, 'brain.response_deviations_v1', 'audit_id', 'SELECT')", &[])?;
        if row.try_get::<_, bool>(0).map_err(super::error)? {
            Ok(())
        } else {
            Err(PortError::PermissionDenied(
                "local response audit unavailable".into(),
            ))
        }
    }
}

impl ResponseAuditPort for LocalPgReader {
    fn append(&self, record: &ResponseDeviation) -> Result<(), PortError> {
        record.validate()?;
        let check = serde_json::to_value(&record.check)
            .map_err(|_| super::invalid("invalid response audit check"))?;
        let sources = serde_json::to_value(&record.source_ids)
            .map_err(|_| super::invalid("invalid response audit sources"))?;
        let evidence = serde_json::to_value(&record.evidence_ids)
            .map_err(|_| super::invalid("invalid response audit evidence"))?;
        let disposition = match record.disposition {
            ResponseDisposition::Rejected => "rejected",
            ResponseDisposition::UncheckedReturned => "unchecked_returned",
        };
        let mut client = self.pool.acquire()?;
        client.query_one("INSERT INTO brain.response_deviations_v1(request_id, model, check_json, raw_output, source_ids, evidence_ids, disposition, identifiers_redacted,raw_output_complete) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) RETURNING audit_id", &[&record.request_id, &record.model, &check, &record.raw_output, &sources, &evidence, &disposition, &record.identifiers_redacted, &record.raw_output_complete])?;
        Ok(())
    }
}
