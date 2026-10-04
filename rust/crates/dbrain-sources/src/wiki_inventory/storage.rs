use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{BufWriter, Read, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::{json, Value};

use crate::{Result, SourcesError};

use super::{
    normalize::{sha256, source_id_for_origin, source_origin_from_url},
    Checkpoint, WikiInventoryReport, CONTRACT_VERSION, SOURCE_ID,
};

pub(super) struct WikiSpool {
    root: PathBuf,
    records_dir: PathBuf,
    max_total_bytes: usize,
    stored_bytes: Cell<usize>,
    bound_origin: RefCell<Option<String>>,
    _lock: File,
}

impl WikiSpool {
    pub(super) fn open(root: &Path, max_total_bytes: usize) -> Result<Self> {
        if max_total_bytes == 0 {
            return Err(SourcesError::invalid_input("Wiki-Gesamtgrenze fehlt"));
        }
        create_dir_all_durable(root)?;
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(root.join("inventory.lock"))?;
        lock.try_lock().map_err(|error| {
            SourcesError::invalid_input(format!(
                "Das Wiki-Inventar wird bereits bearbeitet: {error}"
            ))
        })?;
        let records_dir = root.join("documents");
        create_dir_all_durable(&records_dir)?;
        let mut stored_bytes = 0usize;
        for directory in [
            &records_dir,
            &root.join("provenance"),
            &root.join("conflicts"),
        ] {
            if !directory.exists() {
                continue;
            }
            for entry in fs::read_dir(directory)? {
                let entry = entry?;
                if entry
                    .path()
                    .extension()
                    .is_some_and(|extension| extension == "json")
                {
                    if !entry.file_type()?.is_file() {
                        return Err(SourcesError::invalid_input("Ungültige Wiki-Spooldatei"));
                    }
                    let bytes = usize::try_from(entry.metadata()?.len())
                        .map_err(|_| SourcesError::invalid_input("Wiki-Spooldatei ist zu groß"))?;
                    stored_bytes = stored_bytes
                        .checked_add(bytes)
                        .and_then(|size| size.checked_add(1))
                        .ok_or_else(|| {
                            SourcesError::invalid_input(
                                "Wiki-Spoolgröße überschreitet die Plattformgrenze",
                            )
                        })?;
                    if stored_bytes > max_total_bytes {
                        return Err(SourcesError::invalid_input(
                            "Bestehender Wiki-Spool überschreitet das Größenlimit",
                        ));
                    }
                }
            }
        }
        Ok(Self {
            root: root.into(),
            records_dir,
            max_total_bytes,
            stored_bytes: Cell::new(stored_bytes),
            bound_origin: RefCell::new(None),
            _lock: lock,
        })
    }

    /// Resolve old orphaned records and check every source before any durable effect.
    /// A missing checkpoint is not evidence for the official source.
    pub(super) fn bind_source(
        &self,
        state: &mut Checkpoint,
        requested_origin: Option<&str>,
    ) -> Result<()> {
        let mut origin: Option<String> = None;
        let mut accept = |candidate: &str| -> Result<()> {
            source_id_for_origin(candidate)?;
            if origin
                .as_deref()
                .is_some_and(|previous| previous != candidate)
            {
                return Err(SourcesError::invalid_input(
                    "Verschiedene Wiki-Quellen benötigen getrennte Inventarverzeichnisse",
                ));
            }
            origin = Some(candidate.into());
            Ok(())
        };
        let binding_path = self.root.join("source.json");
        let binding = self.read_optional_json(&binding_path)?;
        if let Some(binding) = &binding {
            let bound = binding["source_origin"]
                .as_str()
                .ok_or_else(|| SourcesError::invalid_input("Wiki-Quellenbindung ohne Herkunft"))?;
            if binding["contract_version"] != CONTRACT_VERSION
                || binding["source_id"] != source_id_for_origin(bound)?
            {
                return Err(SourcesError::invalid_input("Ungültige Wiki-Quellenbindung"));
            }
            accept(bound)?;
        }
        if let Some(context) = &state.context {
            context.validate()?;
            accept(&context.source_origin)?;
        }
        // Include conflicts: older interrupted runs may have no checkpoint or marker.
        for directory in [&self.records_dir, &self.root.join("conflicts")] {
            if !directory.exists() {
                continue;
            }
            for entry in fs::read_dir(directory)? {
                let entry = entry?;
                if entry
                    .path()
                    .extension()
                    .is_some_and(|extension| extension == "json")
                {
                    let document = self.read_optional_json(&entry.path())?.ok_or_else(|| {
                        SourcesError::invalid_input("Wiki-Spooldatei ist verschwunden")
                    })?;
                    validate_document(&document)?;
                    accept(document_origin(&document)?)?;
                    if directory != &self.records_dir {
                        let path = entry.path();
                        let key =
                            path.file_stem()
                                .and_then(|name| name.to_str())
                                .ok_or_else(|| {
                                    SourcesError::invalid_input("Ungültiger Wiki-Konfliktschlüssel")
                                })?;
                        self.read_provenance(key, &document)?;
                    }
                }
            }
        }
        if let Some(requested) = requested_origin {
            accept(requested)?;
        }
        if let Some(origin) = origin {
            let source = source_id_for_origin(&origin)?;
            // Check only actual checkpoint evidence, never Checkpoint::default().
            if self.root.join("checkpoint.json").exists() && state.source_id != source {
                return Err(SourcesError::invalid_input(
                    "Wiki-Checkpoint und Spoolquelle widersprechen sich",
                ));
            }
            if !state
                .pages
                .keys()
                .all(|id| id.starts_with(&format!("wiki:{source}:")))
            {
                return Err(SourcesError::invalid_input(
                    "Wiki-Inventar und Spoolquelle widersprechen sich",
                ));
            }
            if binding.is_none() {
                write_immutable(
                    &binding_path,
                    &serde_json::to_vec(&json!({
                        "contract_version": CONTRACT_VERSION,
                        "source_id": source,
                        "source_origin": origin,
                    }))?,
                )?;
            }
            state.source_id = source.into();
            *self.bound_origin.borrow_mut() = Some(origin);
        }
        Ok(())
    }

    fn read_optional_json(&self, path: &Path) -> Result<Option<Value>> {
        match fs::symlink_metadata(path) {
            Ok(metadata) => {
                if !metadata.is_file() || metadata.len() > self.max_total_bytes as u64 {
                    return Err(SourcesError::invalid_input(
                        "Wiki-Spooldatei überschreitet das Größenlimit oder ist ungültig",
                    ));
                }
                let value = serde_json::from_slice(&read_checkpoint_bytes(
                    File::open(path)?,
                    self.max_total_bytes,
                )?)?;
                sync_parent(path)?;
                Ok(Some(value))
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub(super) fn read_checkpoint(&self) -> Result<Checkpoint> {
        let path = self.root.join("checkpoint.json");
        let state: Checkpoint = match fs::symlink_metadata(&path) {
            Ok(metadata) => {
                if !metadata.is_file() {
                    return Err(SourcesError::invalid_input(
                        "Ungültige Wiki-Checkpointdatei",
                    ));
                }
                let file = File::open(&path)?;
                let metadata = file.metadata()?;
                if !metadata.is_file() || metadata.len() > self.max_total_bytes as u64 {
                    return Err(SourcesError::invalid_input(
                        "Wiki-Checkpoint überschreitet das Größenlimit",
                    ));
                }
                // A file may grow after metadata was checked; bound the actual read too.
                serde_json::from_slice(&read_checkpoint_bytes(file, self.max_total_bytes)?)?
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Checkpoint::default(),
            Err(error) => return Err(error.into()),
        };
        let expected_source = if let Some(context) = &state.context {
            context.validate()?;
            context.source_id()?
        } else {
            SOURCE_ID
        };
        if state.version != CONTRACT_VERSION
            || state.source_id != expected_source
            || state.pages.iter().any(|(id, page)| {
                id != &page.document_id || !id.starts_with(&format!("wiki:{expected_source}:"))
            })
        {
            return Err(SourcesError::invalid_input(
                "Wiki-Checkpoint hat einen anderen Quellenvertrag",
            ));
        }
        Ok(state)
    }

    pub(super) fn write_checkpoint(&self, state: &Checkpoint) -> Result<()> {
        let bound = self.bound_origin.borrow();
        let origin = bound.as_deref().ok_or_else(|| {
            SourcesError::invariant("Wiki-Spool ist nicht an eine Quelle gebunden")
        })?;
        if state.source_id != source_id_for_origin(origin)?
            || state
                .context
                .as_ref()
                .is_some_and(|context| context.source_origin != origin)
        {
            return Err(SourcesError::invalid_input(
                "Wiki-Checkpoint gehört zu einer anderen Spoolquelle",
            ));
        }
        let bytes = serde_json::to_vec(state)?;
        if bytes.len() > self.max_total_bytes {
            return Err(SourcesError::invalid_input(
                "Wiki-Checkpoint überschreitet das Größenlimit",
            ));
        }
        write_atomic(&self.root.join("checkpoint.json"), &bytes)
    }

    pub(super) fn persist_document(&self, document: &Value) -> Result<()> {
        validate_document(document)?;
        if self.bound_origin.borrow().as_deref() != Some(document_origin(document)?) {
            return Err(SourcesError::invalid_input(
                "Wiki-Dokument gehört nicht zur gebundenen Spoolquelle",
            ));
        }
        let id = document["document_id"]
            .as_str()
            .ok_or_else(|| SourcesError::invariant("Wiki-Dokument-ID fehlt"))?;
        let revision = document["revision"]
            .as_str()
            .ok_or_else(|| SourcesError::invariant("Wiki-Revision fehlt"))?;
        let key = sha256(&serde_json::to_vec(&(id, revision))?);
        let target = self.records_dir.join(format!("{key}.json"));
        match fs::read(&target) {
            Ok(bytes) => {
                let previous: Value = serde_json::from_slice(&bytes)?;
                validate_document(&previous)?;
                sync_parent(&target)?;
                if previous["document_id"] != document["document_id"]
                    || previous["revision"] != document["revision"]
                {
                    return Err(SourcesError::invariant("Wiki-Spoolschlüssel kollidiert"));
                }
                if previous["content_sha256"] != document["content_sha256"]
                    || previous["content"] != document["content"]
                {
                    let conflict_dir = self.root.join("conflicts");
                    let conflict_key = format!(
                        "{key}-{}",
                        document["content_sha256"].as_str().unwrap_or_default()
                    );
                    let conflict = conflict_dir.join(format!("{conflict_key}.json"));
                    if let Some(saved) = self.read_optional_json(&conflict)? {
                        validate_document(&saved)?;
                        if saved["document_id"] != document["document_id"]
                            || saved["revision"] != document["revision"]
                            || saved["content_sha256"] != document["content_sha256"]
                            || saved["content"] != document["content"]
                            || document_origin(&saved)? != document_origin(document)?
                        {
                            return Err(SourcesError::invariant(
                                "Wiki-Konfliktschlüssel kollidiert",
                            ));
                        }
                        self.persist_provenance(&conflict_key, &saved, document)?;
                    } else {
                        let bytes = serde_json::to_vec(document)?;
                        let next_size = self
                            .stored_bytes
                            .get()
                            .checked_add(bytes.len())
                            .and_then(|size| size.checked_add(1))
                            .ok_or_else(|| {
                                SourcesError::invalid_input(
                                    "Wiki-Spoolgröße überschreitet die Plattformgrenze",
                                )
                            })?;
                        if next_size > self.max_total_bytes {
                            return Err(SourcesError::invalid_input(
                                "Wiki-Gesamtgrenze erreicht; Konflikt nicht gespeichert, Inventar bleibt unvollständig",
                            ));
                        }
                        write_immutable_with_install(&conflict, &bytes, || {
                            self.stored_bytes.set(next_size);
                        })?;
                    }
                    return Err(SourcesError::invalid_input(
                        "Gleiche Wiki-ID und Revision mit anderem Inhalt; Konflikt erhalten",
                    ));
                }
                return self.persist_provenance(&key, &previous, document);
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
        let bytes = serde_json::to_vec(document)?;
        let next_size = self
            .stored_bytes
            .get()
            .checked_add(bytes.len())
            .and_then(|size| size.checked_add(1))
            .ok_or_else(|| {
                SourcesError::invalid_input("Wiki-Spoolgröße überschreitet die Plattformgrenze")
            })?;
        if next_size > self.max_total_bytes {
            return Err(SourcesError::invalid_input(
                "Wiki-Gesamtgrenze erreicht; Inventar bleibt unvollständig",
            ));
        }
        write_immutable_with_install(&target, &bytes, || {
            self.stored_bytes.set(next_size);
        })?;
        Ok(())
    }

    fn provenance_path(&self, key: &str) -> PathBuf {
        self.root.join("provenance").join(format!("{key}.json"))
    }

    fn read_provenance(&self, key: &str, document: &Value) -> Result<Vec<Value>> {
        let Some(value) = self.read_optional_json(&self.provenance_path(key))? else {
            return Ok(Vec::new());
        };
        let entries = value
            .as_array()
            .ok_or_else(|| SourcesError::invalid_input("Ungültige Wiki-Herkunftsergänzung"))?;
        for entry in entries {
            let assertion = &entry["assertion"];
            if entry["document_id"] != document["document_id"]
                || entry["revision"] != document["revision"]
                || entry["content_sha256"] != document["content_sha256"]
                || entry["assertion_sha256"] != sha256(&serde_json::to_vec(assertion)?)
                || !assertion.is_object()
                || entry["observed_at"].as_str().is_none()
            {
                return Err(SourcesError::invalid_input(
                    "Wiki-Herkunftsergänzung gehört nicht zur Originalrevision",
                ));
            }
            super::normalize::validate_observed_at(entry["observed_at"].as_str().unwrap())?;
        }
        Ok(entries.clone())
    }

    fn persist_provenance(&self, key: &str, previous: &Value, incoming: &Value) -> Result<()> {
        let assertion = provenance_assertion(incoming);
        let mut entries = self.read_provenance(key, previous)?;
        if assertion == provenance_assertion(previous) {
            return Ok(());
        }
        if entries.iter().any(|entry| entry["assertion"] == assertion) {
            return Ok(());
        }
        entries.push(json!({
            "document_id": previous["document_id"],
            "revision": previous["revision"],
            "content_sha256": previous["content_sha256"],
            "assertion_sha256": sha256(&serde_json::to_vec(&assertion)?),
            "assertion": assertion,
            "observed_at": incoming["observed_at"],
        }));
        let target = self.provenance_path(key);
        let previous_size = match fs::metadata(&target) {
            Ok(metadata) => usize::try_from(metadata.len())
                .map_err(|_| SourcesError::invalid_input("Wiki-Herkunftsdatei ist zu groß"))?
                .checked_add(1)
                .ok_or_else(|| {
                    SourcesError::invalid_input("Wiki-Spoolgröße überschreitet die Plattformgrenze")
                })?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => 0,
            Err(error) => return Err(error.into()),
        };
        let bytes = serde_json::to_vec(&entries)?;
        let next = self
            .stored_bytes
            .get()
            .checked_sub(previous_size)
            .and_then(|size| size.checked_add(bytes.len()))
            .and_then(|size| size.checked_add(1))
            .ok_or_else(|| {
                SourcesError::invalid_input("Wiki-Spoolgröße überschreitet die Plattformgrenze")
            })?;
        if next > self.max_total_bytes {
            return Err(SourcesError::invalid_input(
                "Wiki-Gesamtgrenze für Herkunftsergänzung erreicht",
            ));
        }
        write_atomic_with_install(&target, &bytes, || {
            self.stored_bytes.set(next);
        })?;
        Ok(())
    }

    fn enrich_document(&self, key: &str, document: &mut Value) -> Result<()> {
        let entries = self.read_provenance(key, document)?;
        if entries.is_empty() {
            return Ok(());
        }
        let original = provenance_assertion(document);
        let assertions = std::iter::once(&original)
            .chain(entries.iter().map(|entry| &entry["assertion"]))
            .collect::<Vec<_>>();
        let distinct = |field: &str| {
            let mut values = Vec::new();
            for assertion in &assertions {
                let value = &assertion[field];
                if !value.is_null() && !values.contains(value) {
                    values.push(value.clone());
                }
            }
            values
        };
        let authors = distinct("revision_author");
        let contributors = distinct("revision_contributor");
        let conflicts = authors.len() > 1 || contributors.len() > 1;
        // Never select a winner for contradictory evidence or replace an existing value.
        if !conflicts {
            if document["metadata"]["revision_author"].is_null() && authors.len() == 1 {
                document["metadata"]["revision_author"] = authors[0].clone();
            }
            if document["metadata"]["revision_contributor"].is_null() && contributors.len() == 1 {
                document["metadata"]["revision_contributor"] = contributors[0].clone();
            }
        }
        let mut attributions = Vec::new();
        for assertion in &assertions {
            if let Some(attribution) = assertion["license"]["attribution"].as_str() {
                if !attributions.contains(&attribution) {
                    attributions.push(attribution);
                }
            }
        }
        // Only attribution is extended. Original license name, URL and rights stay unchanged.
        document["license"]["attribution"] = json!(attributions.join("; additional provenance: "));
        document["metadata"]["provenance_original"] = original;
        document["metadata"]["provenance_supplements"] = json!(entries);
        document["metadata"]["provenance_conflicts"] = json!({
            "revision_author": authors.len() > 1,
            "revision_contributor": contributors.len() > 1,
            "automatic_selection": false,
        });
        Ok(())
    }

    pub(super) fn publish(&self, state: &Checkpoint) -> Result<WikiInventoryReport> {
        let mut record_paths = Vec::new();
        for entry in fs::read_dir(&self.records_dir)? {
            let entry = entry?;
            if entry
                .path()
                .extension()
                .is_some_and(|extension| extension == "json")
            {
                if !entry.file_type()?.is_file() {
                    return Err(SourcesError::invalid_input("Ungültige Wiki-Spooldatei"));
                }
                record_paths.push(entry.path());
            }
        }
        record_paths.sort();
        let target = self.root.join("documents.jsonl");
        let temporary = temporary_path(&target)?;
        let mut writer = BufWriter::new(
            OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&temporary)?,
        );
        let mut unknown_revisions = 0;
        let mut total_bytes = 0usize;
        let mut document_count = 0;
        let publication = (|| -> Result<()> {
            for path in record_paths {
                let bytes = fs::read(&path)?;
                let mut document: Value = serde_json::from_slice(&bytes)?;
                validate_document(&document)?;
                sync_parent(&path)?;
                if document["source_id"] != state.source_id
                    || self.bound_origin.borrow().as_deref() != Some(document_origin(&document)?)
                {
                    return Err(SourcesError::invalid_input(
                        "Wiki-Spool enthält verschiedene Quellen",
                    ));
                }
                let key = path
                    .file_stem()
                    .and_then(|name| name.to_str())
                    .ok_or_else(|| SourcesError::invalid_input("Ungültiger Wiki-Spoolschlüssel"))?;
                self.enrich_document(key, &mut document)?;
                let bytes = serde_json::to_vec(&document)?;
                total_bytes = total_bytes
                    .checked_add(bytes.len())
                    .and_then(|size| size.checked_add(1))
                    .ok_or_else(|| {
                        SourcesError::invalid_input(
                            "Wiki-JSONL-Größe überschreitet die Plattformgrenze",
                        )
                    })?;
                if total_bytes > self.max_total_bytes {
                    return Err(SourcesError::invalid_input(
                        "Wiki-JSONL überschreitet das Größenlimit",
                    ));
                }
                unknown_revisions += usize::from(
                    document["revision"]
                        .as_str()
                        .is_some_and(|revision| revision.starts_with("unknown:")),
                );
                document_count += 1;
                writer.write_all(&bytes)?;
                writer.write_all(b"\n")?;
            }
            writer.flush()?;
            writer.get_ref().sync_all()?;
            fs::rename(&temporary, &target)?;
            sync_parent(&target)?;
            Ok(())
        })();
        if publication.is_err() {
            let _ = fs::remove_file(&temporary);
        }
        publication?;
        let mut namespace_counts = BTreeMap::new();
        for page in state.pages.values() {
            if let Some(namespace) = page.namespace_id {
                *namespace_counts.entry(namespace).or_insert(0) += 1;
            }
        }
        let expected = state
            .context
            .as_ref()
            .map(|context| {
                context
                    .namespaces
                    .keys()
                    .copied()
                    .filter(|id| *id >= 0)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let completed = state
            .namespaces
            .iter()
            .filter_map(|(id, namespace)| namespace.complete.then_some(*id))
            .collect::<Vec<_>>();
        let complete = state.scope
            == "all_real_namespaces_latest_revision_text_without_media_binaries"
            && !expected.is_empty()
            && expected == completed
            && state.access_block.is_none();
        let report = WikiInventoryReport {
            contract_version: CONTRACT_VERSION.into(),
            source_id: state.source_id.clone(),
            inventory_complete: complete,
            content_complete: complete
                && state.gaps.is_empty()
                && state.pages.values().all(|page| page.content_available),
            namespace_counts,
            namespaces_completed: completed,
            namespaces_expected: expected,
            inventory_pages: state.pages.len(),
            documents: document_count,
            unknown_revisions,
            gaps: state.gaps.clone(),
            access_block: state.access_block.clone(),
            documents_path: target,
            inventory_path: self.root.join("inventory.json"),
            checkpoint_path: self.root.join("checkpoint.json"),
            scope: state.scope.clone(),
        };
        write_atomic(
            &report.inventory_path,
            &serde_json::to_vec(&json!({
                "report": report,
                "pages": state.pages.values().collect::<Vec<_>>(),
                "source_context": state.context,
                "source_snapshot_atomic": false,
                "rendered_dependency_revisions_pinned": false,
                "media_binary_files_collected": false,
                "coverage_denominator": if complete { "actual_namespace_page_inventory" } else { "unavailable_for_full_source" },
                "statistics_are_not_namespace_inventory": true,
            }))?,
        )?;
        Ok(report)
    }
}

pub(super) fn read_checkpoint_bytes(reader: impl Read, maximum: usize) -> Result<Vec<u8>> {
    let limit = u64::try_from(maximum)
        .ok()
        .and_then(|limit| limit.checked_add(1))
        .ok_or_else(|| {
            SourcesError::invalid_input("Wiki-Checkpointgrenze überschreitet die Plattformgrenze")
        })?;
    let mut bytes = Vec::new();
    reader.take(limit).read_to_end(&mut bytes)?;
    if bytes.len() > maximum {
        return Err(SourcesError::invalid_input(
            "Wiki-Checkpoint überschreitet das Größenlimit",
        ));
    }
    Ok(bytes)
}

fn provenance_assertion(document: &Value) -> Value {
    let metadata = &document["metadata"];
    json!({
        "revision_author": metadata["revision_author"],
        "revision_contributor": metadata["revision_contributor"],
        "attribution_url": metadata["attribution_url"],
        "license": document["license"],
        "source_locator": document["source_locator"],
        "source_origin": metadata["source_origin"],
        "source_siteinfo_sha256": metadata["source_siteinfo_sha256"],
        "source_siteinfo_hash_representation": metadata["source_siteinfo_hash_representation"],
        "source_capture_sha256": metadata["source_capture_sha256"],
        "source_fetched_at": metadata["source_fetched_at"],
        "license_observed_at": metadata["license_observed_at"],
        "provenance_capture_format": metadata["provenance_capture_format"],
    })
}

fn document_origin(document: &Value) -> Result<&str> {
    let locator = document["source_locator"]
        .as_str()
        .ok_or_else(|| SourcesError::invalid_input("Wiki-Dokument ohne Quellenadresse"))?;
    source_origin_from_url(locator.split(['?', '#']).next().unwrap_or(locator))
}

fn validate_document(document: &Value) -> Result<()> {
    let locator = document["source_locator"]
        .as_str()
        .ok_or_else(|| SourcesError::invalid_input("Wiki-Dokument ohne Quellenadresse"))?;
    // Locators contain a query; validate the origin independently of its page parameters.
    let address = locator.split(['?', '#']).next().unwrap_or(locator);
    let expected_source = source_id_for_origin(source_origin_from_url(address)?)?;
    let content = document
        .get("content")
        .and_then(Value::as_str)
        .ok_or_else(|| SourcesError::invalid_input("Wiki-Dokument ohne Quellentext"))?;
    if document["contract_version"] != CONTRACT_VERSION
        || document["source_kind"] != "wiki"
        || document["source_id"] != expected_source
        || !document["document_id"]
            .as_str()
            .is_some_and(|id| id.starts_with(&format!("wiki:{expected_source}:")))
        || !document["revision"]
            .as_str()
            .is_some_and(|revision| !revision.is_empty())
        || document["content_sha256"] != sha256(content.as_bytes())
        || !document["metadata"].is_object()
        || document["metadata"]["source_origin"]
            .as_str()
            .is_some_and(|origin| origin != source_origin_from_url(address).unwrap_or_default())
        || !document["facts"].is_array()
        || !document["license"].is_object()
    {
        return Err(SourcesError::invalid_input(
            "Ungültiges Wiki-Vertragsdokument im Spool",
        ));
    }
    Ok(())
}

pub(super) fn write_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    write_atomic_with_install(path, bytes, || {})
}

fn write_atomic_with_install(path: &Path, bytes: &[u8], installed: impl FnOnce()) -> Result<()> {
    if let Some(parent) = path.parent() {
        create_dir_all_durable(parent)?;
    }
    let temporary = temporary_path(path)?;
    let outcome = (|| -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)?;
        file.write_all(bytes)?;
        file.sync_all()?;
        fs::rename(&temporary, path)?;
        installed();
        sync_parent(path)
    })();
    if outcome.is_err() {
        let _ = fs::remove_file(temporary);
    }
    outcome
}

fn write_immutable(path: &Path, bytes: &[u8]) -> Result<()> {
    write_immutable_with_install(path, bytes, || {})
}

fn write_immutable_with_install(path: &Path, bytes: &[u8], installed: impl FnOnce()) -> Result<()> {
    if let Some(parent) = path.parent() {
        create_dir_all_durable(parent)?;
    }
    let temporary = temporary_path(path)?;
    let outcome = (|| -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)?;
        file.write_all(bytes)?;
        file.sync_all()?;
        fs::hard_link(&temporary, path)?;
        installed();
        fs::remove_file(&temporary)?;
        sync_parent(path)
    })();
    if outcome.is_err() {
        let _ = fs::remove_file(temporary);
    }
    outcome
}

fn create_dir_all_durable(path: &Path) -> Result<()> {
    create_dir_all_with_sync(path, |directory| {
        File::open(directory)?.sync_all()?;
        Ok(())
    })
}

// Sync every ancestor, including an already existing directory left by a failed
// earlier attempt. Syncing only the leaf does not persist its entry in its parent.
pub(super) fn create_dir_all_with_sync(
    path: &Path,
    mut sync: impl FnMut(&Path) -> Result<()>,
) -> Result<()> {
    let absolute = std::path::absolute(path)?;
    fs::create_dir_all(&absolute)?;
    for directory in absolute.ancestors() {
        sync(directory)?;
    }
    Ok(())
}

fn temporary_path(path: &Path) -> Result<PathBuf> {
    let name = path
        .file_name()
        .ok_or_else(|| SourcesError::invalid_input("Wiki-Dateiname fehlt"))?
        .to_string_lossy();
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    Ok(path.with_file_name(format!(".{name}.{}.{}.tmp", std::process::id(), nonce)))
}

#[cfg(test)]
struct ParentSyncFailures {
    path: PathBuf,
    skip: usize,
    remaining: usize,
    attempts: usize,
}

#[cfg(test)]
std::thread_local! {
    static PARENT_SYNC_FAILURES: RefCell<Option<ParentSyncFailures>> = const { RefCell::new(None) };
}

#[cfg(test)]
pub(super) fn with_parent_sync_failures<T>(
    path: &Path,
    failures: usize,
    action: impl FnOnce() -> T,
) -> (T, usize) {
    with_parent_sync_failures_after(path, 0, failures, action)
}

#[cfg(test)]
pub(super) fn with_parent_sync_failures_after<T>(
    path: &Path,
    skip: usize,
    failures: usize,
    action: impl FnOnce() -> T,
) -> (T, usize) {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            PARENT_SYNC_FAILURES.with(|slot| *slot.borrow_mut() = None);
        }
    }
    PARENT_SYNC_FAILURES.with(|slot| {
        let mut slot = slot.borrow_mut();
        assert!(slot.is_none());
        *slot = Some(ParentSyncFailures {
            path: path.into(),
            skip,
            remaining: failures,
            attempts: 0,
        });
    });
    let reset = Reset;
    let result = action();
    let attempts = PARENT_SYNC_FAILURES.with(|slot| slot.borrow().as_ref().unwrap().attempts);
    drop(reset);
    (result, attempts)
}

fn sync_parent(path: &Path) -> Result<()> {
    #[cfg(test)]
    PARENT_SYNC_FAILURES.with(|slot| -> Result<()> {
        let mut slot = slot.borrow_mut();
        if let Some(failure) = slot.as_mut() {
            if failure.path == path {
                failure.attempts += 1;
                if failure.skip > 0 {
                    failure.skip -= 1;
                } else if failure.remaining > 0 {
                    failure.remaining -= 1;
                    return Err(std::io::Error::other(
                        "Injizierter abschließender Elternsyncfehler",
                    )
                    .into());
                }
            }
        }
        Ok(())
    })?;
    if let Some(parent) = path.parent() {
        File::open(parent)?.sync_all()?;
    }
    Ok(())
}
