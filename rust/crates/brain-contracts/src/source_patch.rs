//! Source patch constraints shared by generic retrieval and reviewed domain projections.
use super::{OriginArtifact, Versioned, ORIGIN_METADATA_KEY};
use crate::value::{Observed, UnknownReason};
use std::collections::BTreeMap;

/// Check every known source patch without deriving game validity from a corpus,
/// index, request or current head. Callers must separately authorize/bind the source.
///
/// The canonical origin remains authoritative, including explicit Unknown. A
/// legacy patch is a constraint, not permission to upgrade canonical Unknown.
/// Legacy Wiki metadata spells missing values as JSON `null`; this is not a pin.
/// A reviewed typed projection may establish unknown validity independently, but
/// may never override a known contradictory source patch.
pub fn patch_validity_for(
    metadata: &BTreeMap<String, String>,
    requested_patch: Option<&str>,
) -> Result<Observed<String>, String> {
    let legacy = metadata
        .get("patch")
        .filter(|value| value.as_str() != "null");
    let canonical = metadata
        .get(ORIGIN_METADATA_KEY)
        .map(|encoded| -> Result<Observed<String>, String> {
            let origin: Versioned<OriginArtifact> =
                serde_json::from_str(encoded).map_err(|error| error.to_string())?;
            origin.data.validate()?;
            Ok(origin.data.validity.patch)
        })
        .transpose()?;
    if let Some(Observed::Known { value }) = &canonical {
        if legacy.is_some_and(|patch| patch != value) {
            return Err("conflicting canonical and legacy source patches".into());
        }
    }
    if let Some(requested) = requested_patch {
        if legacy.is_some_and(|patch| patch != requested)
            || matches!(&canonical, Some(Observed::Known { value }) if value != requested)
        {
            return Err("source patch does not match requested patch".into());
        }
    }
    Ok(canonical.unwrap_or_else(|| match metadata.get("patch") {
        Some(value) if value == "null" => Observed::unknown(UnknownReason::ExplicitNull),
        Some(value) => Observed::known(value.clone()),
        None => Observed::unknown(UnknownReason::NotPresent),
    }))
}
