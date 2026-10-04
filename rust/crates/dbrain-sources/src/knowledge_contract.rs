use chrono::DateTime;
use serde::de::{DeserializeSeed, Error as _, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::io::{BufRead, Cursor, Read, Seek, SeekFrom};

pub const KNOWLEDGE_CONTRACT_VERSION: &str = "wiki-spielwissen-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeSourceKind {
    Wiki,
    GameFile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeEvidenceStatus {
    ExtractedValue,
    SourceStatement,
    Hypothesis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KnowledgeLicense {
    pub name: String,
    #[serde(deserialize_with = "required_nullable")]
    pub url: Option<String>,
    pub attribution: String,
    pub redistribution_allowed: bool,
}

impl KnowledgeLicense {
    pub fn publication_permitted_by_declaration(&self) -> bool {
        self.redistribution_allowed
            && valid_text(&self.name)
            && !self.name.trim().eq_ignore_ascii_case("unverified")
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KnowledgeFact {
    pub fact_id: String,
    pub subject: String,
    pub predicate: String,
    #[serde(deserialize_with = "required_value")]
    pub value: Value,
    #[serde(deserialize_with = "required_nullable")]
    pub unit: Option<String>,
    pub evidence_status: KnowledgeEvidenceStatus,
    #[serde(deserialize_with = "required_nullable")]
    pub source_span: Option<String>,
    pub qualifiers: Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KnowledgeDocument {
    pub contract_version: String,
    pub source_kind: KnowledgeSourceKind,
    pub source_id: String,
    pub document_id: String,
    pub source_locator: String,
    pub title: String,
    pub language: String,
    pub revision: String,
    pub observed_at: String,
    pub content_sha256: String,
    pub content: String,
    pub evidence_status: KnowledgeEvidenceStatus,
    pub license: KnowledgeLicense,
    pub metadata: Map<String, Value>,
    pub facts: Vec<KnowledgeFact>,
}

impl KnowledgeDocument {
    pub fn revision_is_unknown(&self) -> bool {
        self.revision == format!("unknown:{}", self.content_sha256)
    }

    pub fn validate(&self, line: usize) -> Result<(), KnowledgeValidationError> {
        if self.contract_version != KNOWLEDGE_CONTRACT_VERSION {
            return Err(issue(
                line,
                "contract_version",
                "Unbekannte Vertragsversion",
            ));
        }
        for (field, value) in [
            ("source_id", &self.source_id),
            ("document_id", &self.document_id),
            ("source_locator", &self.source_locator),
            ("title", &self.title),
            ("language", &self.language),
            ("revision", &self.revision),
            ("license.name", &self.license.name),
            ("license.attribution", &self.license.attribution),
        ] {
            if !valid_text(value) {
                return Err(issue(
                    line,
                    field,
                    "Nichtleerer Text ohne Steuerzeichen erforderlich",
                ));
            }
        }
        if !is_sha256(&self.content_sha256) {
            return Err(issue(
                line,
                "content_sha256",
                "64 kleingeschriebene SHA-256-Hexzeichen erforderlich",
            ));
        }
        if sha256_content(&self.content) != self.content_sha256 {
            return Err(issue(
                line,
                "content_sha256",
                "Hash stimmt nicht mit den exakten UTF-8-Inhaltsbytes überein",
            ));
        }
        if self.revision.to_ascii_lowercase().starts_with("unknown") && !self.revision_is_unknown()
        {
            return Err(issue(
                line,
                "revision",
                "Unbekannte Revision muss unknown:<content_sha256> lauten",
            ));
        }
        let timestamp = DateTime::parse_from_rfc3339(&self.observed_at).map_err(|_| {
            issue(
                line,
                "observed_at",
                "Gültiger UTC-Zeitpunkt nach RFC3339 erforderlich",
            )
        })?;
        let bytes = self.observed_at.as_bytes();
        if bytes
            .get(10)
            .copied()
            .is_none_or(|b| !matches!(b, b'T' | b't'))
            || timestamp.offset().local_minus_utc() != 0
            || !(self.observed_at.ends_with('Z')
                || self.observed_at.ends_with('z')
                || self.observed_at.ends_with("+00:00"))
        {
            return Err(issue(
                line,
                "observed_at",
                "UTC mit Z oder +00:00 erforderlich; -00:00 belegt keine Zeitzone",
            ));
        }
        validate_document_identity(self, line)?;
        if let Some(url) = &self.license.url {
            if !is_web_url(url) {
                return Err(issue(
                    line,
                    "license.url",
                    "HTTP(S)-URL ohne Zugangsdaten oder null erforderlich",
                ));
            }
        }
        let mut fact_ids = BTreeSet::new();
        for (index, fact) in self.facts.iter().enumerate() {
            for (field, value) in [
                ("fact_id", &fact.fact_id),
                ("subject", &fact.subject),
                ("predicate", &fact.predicate),
            ] {
                if !valid_text(value) {
                    return Err(issue(
                        line,
                        format!("facts[{index}].{field}"),
                        "Nichtleerer Text ohne Steuerzeichen erforderlich",
                    ));
                }
            }
            if !fact_ids.insert(&fact.fact_id) {
                return Err(issue(
                    line,
                    format!("facts[{index}].fact_id"),
                    "Fact-ID ist innerhalb des Dokuments mehrfach vorhanden",
                ));
            }
            for (field, value) in [("unit", &fact.unit), ("source_span", &fact.source_span)] {
                if value.as_ref().is_some_and(|value| !valid_text(value)) {
                    return Err(issue(
                        line,
                        format!("facts[{index}].{field}"),
                        "Nichtleerer Text ohne Steuerzeichen oder null erforderlich",
                    ));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("Zeile {line}, Feld {field}: {message}")]
pub struct KnowledgeValidationError {
    pub line: usize,
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeValidationErrors {
    pub errors: Vec<KnowledgeValidationError>,
}

impl fmt::Display for KnowledgeValidationErrors {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, error) in self.errors.iter().enumerate() {
            if index != 0 {
                writeln!(formatter)?;
            }
            write!(formatter, "{error}")?;
        }
        Ok(())
    }
}

impl std::error::Error for KnowledgeValidationErrors {}

#[derive(Debug, Clone, PartialEq)]
pub struct LocatedKnowledgeDocument {
    pub line: usize,
    pub document: KnowledgeDocument,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnowledgeConflictKind {
    ContentMismatch,
    RepresentationMismatch,
    SourceIdentityMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeInputConflict {
    pub first_line: usize,
    pub line: usize,
    pub document_id: String,
    pub revision: String,
    pub kind: KnowledgeConflictKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeInputDuplicate {
    pub first_line: usize,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValidatedKnowledgeInput {
    documents: Vec<LocatedKnowledgeDocument>,
    duplicates: Vec<KnowledgeInputDuplicate>,
    conflicts: Vec<KnowledgeInputConflict>,
}

impl ValidatedKnowledgeInput {
    pub fn documents(&self) -> &[LocatedKnowledgeDocument] {
        &self.documents
    }

    pub fn duplicates(&self) -> &[KnowledgeInputDuplicate] {
        &self.duplicates
    }

    pub fn conflicts(&self) -> &[KnowledgeInputConflict] {
        &self.conflicts
    }

    pub fn unknown_revision_count(&self) -> usize {
        self.documents
            .iter()
            .filter(|record| record.document.revision_is_unknown())
            .count()
    }
}

pub fn sha256_content(content: &str) -> String {
    format!("{:x}", Sha256::digest(content.as_bytes()))
}

pub fn validate_knowledge_jsonl_str(
    input: &str,
) -> Result<ValidatedKnowledgeInput, KnowledgeValidationErrors> {
    validate_knowledge_jsonl(Cursor::new(input.as_bytes()))
}

pub fn validate_knowledge_jsonl(
    mut input: impl BufRead,
) -> Result<ValidatedKnowledgeInput, KnowledgeValidationErrors> {
    let mut documents = Vec::new();
    let mut errors = Vec::new();
    let mut line = 0;
    let mut bytes = Vec::new();
    loop {
        bytes.clear();
        match input.read_until(b'\n', &mut bytes) {
            Ok(0) => break,
            Ok(_) => line += 1,
            Err(error) => {
                errors.push(issue(
                    line + 1,
                    "$",
                    format!("Eingabe konnte nicht gelesen werden: {error}"),
                ));
                break;
            }
        }
        let parsed = std::str::from_utf8(&bytes)
            .map_err(|error| issue(line, "$", format!("Ungültiges UTF-8: {error}")))
            .and_then(|text| parse_document(text, line));
        match parsed {
            Ok(document) => documents.push(LocatedKnowledgeDocument { line, document }),
            Err(error) => errors.push(error),
        }
    }
    if !errors.is_empty() {
        return Err(KnowledgeValidationErrors { errors });
    }
    classify_documents(documents).map_err(|error| KnowledgeValidationErrors {
        errors: vec![error],
    })
}

#[derive(Debug, Clone, Copy)]
pub struct KnowledgeStreamLimits {
    pub max_line_bytes: usize,
    pub max_input_bytes: u64,
    pub max_document_lines: usize,
    pub max_index_bytes: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct KnowledgeLineRange {
    pub start: u64,
    pub end_exclusive: u64,
    pub line: usize,
}

#[derive(Debug)]
pub struct ValidatedKnowledgeStream {
    pub document_lines: usize,
    pub input_bytes: u64,
    pub index_bytes: usize,
    pub duplicate_lines: Vec<KnowledgeInputDuplicate>,
    pub conflicts: Vec<KnowledgeInputConflict>,
}

#[derive(Default)]
struct KnowledgeClassifier<P> {
    identities: BTreeMap<String, (usize, KnowledgeSourceKind, String)>,
    revisions: BTreeMap<(String, String), P>,
    duplicates: Vec<KnowledgeInputDuplicate>,
    conflicts: Vec<KnowledgeInputConflict>,
    index_bytes: usize,
}

impl<P> KnowledgeClassifier<P> {
    fn observe<'a>(
        &mut self,
        record: &LocatedKnowledgeDocument,
        position: P,
        maximum: usize,
        mut read_first: impl FnMut(
            &P,
        ) -> Result<
            Cow<'a, LocatedKnowledgeDocument>,
            KnowledgeValidationError,
        >,
    ) -> Result<(), KnowledgeValidationError> {
        let document = &record.document;
        let revision_identity = if document.source_kind == KnowledgeSourceKind::Wiki
            && document.revision.bytes().all(|byte| byte.is_ascii_digit())
        {
            document.revision.trim_start_matches('0')
        } else {
            document.revision.as_str()
        };
        let cost = document
            .document_id
            .len()
            .saturating_mul(3)
            .saturating_add(document.source_id.len())
            .saturating_add(document.revision.len().saturating_mul(3))
            .saturating_add(512);
        self.index_bytes = self
            .index_bytes
            .checked_add(cost)
            .filter(|bytes| *bytes <= maximum)
            .ok_or_else(|| {
                issue(
                    record.line,
                    "$",
                    "Begrenzter Identitäts-/Revisionsindex ist voll",
                )
            })?;
        if let Some((first_line, source_kind, source_id)) =
            self.identities.get(&document.document_id)
        {
            if source_id != &document.source_id || *source_kind != document.source_kind {
                self.conflicts.push(KnowledgeInputConflict {
                    first_line: *first_line,
                    line: record.line,
                    document_id: document.document_id.clone(),
                    revision: document.revision.clone(),
                    kind: KnowledgeConflictKind::SourceIdentityMismatch,
                });
            }
        } else {
            self.identities.insert(
                document.document_id.clone(),
                (
                    record.line,
                    document.source_kind,
                    document.source_id.clone(),
                ),
            );
        }
        let key = (document.document_id.clone(), revision_identity.to_owned());
        if let Some(first_position) = self.revisions.get(&key) {
            let first = read_first(first_position)?;
            if first.document.content_sha256 != document.content_sha256 {
                self.conflicts.push(input_conflict(
                    &first,
                    record,
                    KnowledgeConflictKind::ContentMismatch,
                ));
            } else if same_document_representation(document, &first.document) {
                self.duplicates.push(KnowledgeInputDuplicate {
                    first_line: first.line,
                    line: record.line,
                });
            } else {
                self.conflicts.push(input_conflict(
                    &first,
                    record,
                    KnowledgeConflictKind::RepresentationMismatch,
                ));
            }
        } else {
            self.revisions.insert(key, position);
        }
        Ok(())
    }
}

fn same_document_representation(a: &KnowledgeDocument, b: &KnowledgeDocument) -> bool {
    a.contract_version == b.contract_version
        && a.source_kind == b.source_kind
        && a.source_id == b.source_id
        && a.document_id == b.document_id
        && a.source_locator == b.source_locator
        && a.title == b.title
        && a.language == b.language
        && a.revision == b.revision
        && a.content_sha256 == b.content_sha256
        && a.content == b.content
        && a.evidence_status == b.evidence_status
        && a.license == b.license
        && a.metadata == b.metadata
        && a.facts == b.facts
}

fn classify_documents(
    documents: Vec<LocatedKnowledgeDocument>,
) -> Result<ValidatedKnowledgeInput, KnowledgeValidationError> {
    let mut classifier = KnowledgeClassifier::<usize>::default();
    for (index, record) in documents.iter().enumerate() {
        classifier.observe(record, index, usize::MAX, |first| {
            Ok(Cow::Borrowed(&documents[*first]))
        })?;
    }
    Ok(ValidatedKnowledgeInput {
        documents,
        duplicates: classifier.duplicates,
        conflicts: classifier.conflicts,
    })
}

fn read_knowledge_line(
    input: &mut impl BufRead,
    bytes: &mut Vec<u8>,
    maximum: usize,
    line: usize,
) -> Result<usize, KnowledgeValidationError> {
    bytes.clear();
    let length = input
        .take(maximum as u64 + 1)
        .read_until(b'\n', bytes)
        .map_err(|_| issue(line, "$", "Eingabe konnte nicht gelesen werden"))?;
    if length > maximum {
        return Err(issue(
            line,
            "$",
            "Dokumentzeile überschreitet die begrenzte Singletongröße",
        ));
    }
    Ok(length)
}

fn parse_knowledge_line(
    bytes: &[u8],
    line: usize,
) -> Result<LocatedKnowledgeDocument, KnowledgeValidationError> {
    let text = std::str::from_utf8(bytes).map_err(|_| issue(line, "$", "Ungültiges UTF-8"))?;
    Ok(LocatedKnowledgeDocument {
        line,
        document: parse_document(text, line)?,
    })
}

pub fn validate_knowledge_jsonl_seekable(
    mut input: impl BufRead + Seek,
    limits: KnowledgeStreamLimits,
    mut visit: impl FnMut(
        &LocatedKnowledgeDocument,
        KnowledgeLineRange,
        &[u8],
    ) -> Result<(), KnowledgeValidationError>,
) -> Result<ValidatedKnowledgeStream, KnowledgeValidationError> {
    if limits.max_line_bytes == 0
        || limits.max_line_bytes == usize::MAX
        || limits.max_input_bytes == 0
        || limits.max_document_lines == 0
        || limits.max_index_bytes == 0
    {
        return Err(issue(
            0,
            "$",
            "Streaminggrenzen müssen endlich und größer als null sein",
        ));
    }
    input
        .seek(SeekFrom::Start(0))
        .map_err(|_| issue(0, "$", "Eingabesicherung ist nicht seekfähig"))?;
    let mut classifier = KnowledgeClassifier::<KnowledgeLineRange>::default();
    let mut bytes = Vec::new();
    let mut reread = Vec::new();
    let mut lines = 0usize;
    let mut start = 0u64;
    loop {
        let line = lines + 1;
        let length = read_knowledge_line(&mut input, &mut bytes, limits.max_line_bytes, line)?;
        if length == 0 {
            break;
        }
        if lines == limits.max_document_lines {
            return Err(issue(
                line,
                "$",
                "Maximale Anzahl historischer Dokumentzeilen überschritten",
            ));
        }
        let end = start
            .checked_add(length as u64)
            .filter(|end| *end <= limits.max_input_bytes)
            .ok_or_else(|| {
                issue(
                    line,
                    "$",
                    "Eingabesicherung überschreitet die Gesamtbytegrenze",
                )
            })?;
        let record = parse_knowledge_line(&bytes, line)?;
        let range = KnowledgeLineRange {
            start,
            end_exclusive: end,
            line,
        };
        classifier.observe(&record, range, limits.max_index_bytes, |first| {
            input
                .seek(SeekFrom::Start(first.start))
                .map_err(|_| issue(line, "$", "Originalzeile kann nicht nachgelesen werden"))?;
            let length =
                read_knowledge_line(&mut input, &mut reread, limits.max_line_bytes, first.line)?;
            if length as u64 != first.end_exclusive - first.start {
                return Err(issue(
                    line,
                    "$",
                    "Nachgelesene Originalzeile hat eine andere Länge",
                ));
            }
            let original = parse_knowledge_line(&reread, first.line)?;
            input.seek(SeekFrom::Start(end)).map_err(|_| {
                issue(
                    line,
                    "$",
                    "Streamingposition kann nicht wiederhergestellt werden",
                )
            })?;
            Ok(Cow::Owned(original))
        })?;
        visit(&record, range, &bytes)?;
        lines += 1;
        start = end;
    }
    Ok(ValidatedKnowledgeStream {
        document_lines: lines,
        input_bytes: start,
        index_bytes: classifier.index_bytes,
        duplicate_lines: classifier.duplicates,
        conflicts: classifier.conflicts,
    })
}

fn input_conflict(
    first: &LocatedKnowledgeDocument,
    current: &LocatedKnowledgeDocument,
    kind: KnowledgeConflictKind,
) -> KnowledgeInputConflict {
    KnowledgeInputConflict {
        first_line: first.line,
        line: current.line,
        document_id: current.document.document_id.clone(),
        revision: current.document.revision.clone(),
        kind,
    }
}

pub(crate) fn parse_unique_json(text: &str) -> Result<Value, serde_json::Error> {
    let mut deserializer = serde_json::Deserializer::from_str(text);
    let value = UniqueJsonSeed { path: "$".into() }.deserialize(&mut deserializer)?;
    deserializer.end()?;
    drop(value);
    serde_json::from_str(text)
}

fn parse_document(text: &str, line: usize) -> Result<KnowledgeDocument, KnowledgeValidationError> {
    let value = parse_unique_json(text)
        .map_err(|error| issue(line, "$", format!("Ungültiges JSON: {error}")))?;
    validate_shape(&value, line)?;
    drop(value);
    let document: KnowledgeDocument = serde_json::from_str(text)
        .map_err(|error| issue(line, "$", format!("Ungültiger Vertragstyp: {error}")))?;
    document.validate(line)?;
    Ok(document)
}

fn validate_shape(value: &Value, line: usize) -> Result<(), KnowledgeValidationError> {
    let object = object_at(value, line, "$")?;
    for field in [
        "contract_version",
        "source_kind",
        "source_id",
        "document_id",
        "source_locator",
        "title",
        "language",
        "revision",
        "observed_at",
        "content_sha256",
        "content",
        "evidence_status",
    ] {
        require_kind(object, field, "", line, |value| value.is_string(), "Text")?;
    }
    require_enum(object, "source_kind", "", line, &["wiki", "game_file"])?;
    require_enum(
        object,
        "evidence_status",
        "",
        line,
        &["extracted_value", "source_statement", "hypothesis"],
    )?;
    require_kind(
        object,
        "metadata",
        "",
        line,
        |value| value.is_object(),
        "Objekt",
    )?;
    let license = object_at(required(object, "license", "", line)?, line, "license")?;
    for field in ["name", "attribution"] {
        require_kind(
            license,
            field,
            "license.",
            line,
            |value| value.is_string(),
            "Text",
        )?;
    }
    require_kind(
        license,
        "url",
        "license.",
        line,
        |value| value.is_null() || value.is_string(),
        "Text oder null",
    )?;
    require_kind(
        license,
        "redistribution_allowed",
        "license.",
        line,
        |value| value.is_boolean(),
        "Boolescher Wert",
    )?;
    reject_unknown(
        license,
        &["name", "url", "attribution", "redistribution_allowed"],
        "license.",
        line,
    )?;
    let facts = required(object, "facts", "", line)?
        .as_array()
        .ok_or_else(|| issue(line, "facts", "Liste erforderlich"))?;
    for (index, value) in facts.iter().enumerate() {
        let prefix = format!("facts[{index}].");
        let fact = object_at(value, line, format!("facts[{index}]"))?;
        for field in ["fact_id", "subject", "predicate", "evidence_status"] {
            require_kind(
                fact,
                field,
                &prefix,
                line,
                |value| value.is_string(),
                "Text",
            )?;
        }
        required(fact, "value", &prefix, line)?;
        for field in ["unit", "source_span"] {
            require_kind(
                fact,
                field,
                &prefix,
                line,
                |value| value.is_null() || value.is_string(),
                "Text oder null",
            )?;
        }
        require_enum(
            fact,
            "evidence_status",
            &prefix,
            line,
            &["extracted_value", "source_statement", "hypothesis"],
        )?;
        require_kind(
            fact,
            "qualifiers",
            &prefix,
            line,
            |value| value.is_object(),
            "Objekt",
        )?;
        reject_unknown(
            fact,
            &[
                "fact_id",
                "subject",
                "predicate",
                "value",
                "unit",
                "evidence_status",
                "source_span",
                "qualifiers",
            ],
            &prefix,
            line,
        )?;
    }
    reject_unknown(
        object,
        &[
            "contract_version",
            "source_kind",
            "source_id",
            "document_id",
            "source_locator",
            "title",
            "language",
            "revision",
            "observed_at",
            "content_sha256",
            "content",
            "evidence_status",
            "license",
            "metadata",
            "facts",
        ],
        "",
        line,
    )
}

fn required<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    prefix: &str,
    line: usize,
) -> Result<&'a Value, KnowledgeValidationError> {
    object
        .get(field)
        .ok_or_else(|| issue(line, format!("{prefix}{field}"), "Pflichtfeld fehlt"))
}

fn require_kind(
    object: &Map<String, Value>,
    field: &str,
    prefix: &str,
    line: usize,
    check: impl FnOnce(&Value) -> bool,
    expected: &str,
) -> Result<(), KnowledgeValidationError> {
    if check(required(object, field, prefix, line)?) {
        Ok(())
    } else {
        Err(issue(
            line,
            format!("{prefix}{field}"),
            format!("{expected} erforderlich"),
        ))
    }
}

fn require_enum(
    object: &Map<String, Value>,
    field: &str,
    prefix: &str,
    line: usize,
    allowed: &[&str],
) -> Result<(), KnowledgeValidationError> {
    if required(object, field, prefix, line)?
        .as_str()
        .is_some_and(|value| allowed.contains(&value))
    {
        Ok(())
    } else {
        Err(issue(
            line,
            format!("{prefix}{field}"),
            "Unzulässiger Vertragswert",
        ))
    }
}

fn reject_unknown(
    object: &Map<String, Value>,
    fields: &[&str],
    prefix: &str,
    line: usize,
) -> Result<(), KnowledgeValidationError> {
    if let Some(field) = object
        .keys()
        .find(|field| !fields.contains(&field.as_str()))
    {
        return Err(issue(
            line,
            format!("{prefix}{field}"),
            "Unbekanntes Vertragsfeld; Zusatzdaten gehören in metadata oder qualifiers",
        ));
    }
    Ok(())
}

fn object_at(
    value: &Value,
    line: usize,
    field: impl Into<String>,
) -> Result<&Map<String, Value>, KnowledgeValidationError> {
    value
        .as_object()
        .ok_or_else(|| issue(line, field, "Objekt erforderlich"))
}

fn validate_document_identity(
    document: &KnowledgeDocument,
    line: usize,
) -> Result<(), KnowledgeValidationError> {
    match document.source_kind {
        KnowledgeSourceKind::Wiki => {
            if document
                .source_id
                .chars()
                .any(|character| character.is_whitespace() || matches!(character, ':' | '/' | '\\'))
            {
                return Err(issue(
                    line,
                    "source_id",
                    "Wiki-Quellen-ID muss ein einzelnes ID-Segment sein",
                ));
            }
            if !is_web_url(&document.source_locator) {
                return Err(issue(
                    line,
                    "source_locator",
                    "Wiki-Quelle muss eine HTTP(S)-URL ohne Zugangsdaten sein",
                ));
            }
            let prefix = format!("wiki:{}:", document.source_id);
            let suffix = document.document_id.strip_prefix(&prefix).ok_or_else(|| {
                issue(
                    line,
                    "document_id",
                    "Wiki-ID passt nicht zu Quellenart oder Quellen-ID",
                )
            })?;
            let valid = if let Some(page_id) = suffix.strip_prefix("page:") {
                positive_decimal_id(page_id)
            } else if let Some(hash) = suffix.strip_prefix("url:") {
                is_sha256(hash) && hash == sha256_content(&document.source_locator)
            } else {
                false
            };
            if !valid {
                return Err(issue(
                    line,
                    "document_id",
                    "wiki:<source_id>:page:<page_id> oder url:<URL-SHA-256> erforderlich",
                ));
            }
        }
        KnowledgeSourceKind::GameFile => {
            let (app_id, path) = document
                .document_id
                .strip_prefix("game:")
                .and_then(|suffix| suffix.split_once(':'))
                .ok_or_else(|| {
                    issue(
                        line,
                        "document_id",
                        "game:<app_id>:<relativer Depotpfad> erforderlich",
                    )
                })?;
            if !positive_decimal_id(app_id) || !normalized_relative_path(path) {
                return Err(issue(
                    line,
                    "document_id",
                    "Positive App-ID und normalisierter relativer Depotpfad erforderlich",
                ));
            }
            if !document.source_locator.starts_with("steam:")
                && !normalized_relative_path(&document.source_locator)
            {
                return Err(issue(
                    line,
                    "source_locator",
                    "Steam-Quellenkennung oder normalisierter relativer Depotpfad erforderlich",
                ));
            }
        }
    }
    Ok(())
}

fn positive_decimal_id(value: &str) -> bool {
    !value.is_empty() && !value.starts_with('0') && value.bytes().all(|byte| byte.is_ascii_digit())
}

fn normalized_relative_path(value: &str) -> bool {
    valid_text(value)
        && !value.contains('\\')
        && !value.contains(':')
        && value
            .split('/')
            .all(|part| !part.is_empty() && part != "." && part != "..")
}

fn is_web_url(value: &str) -> bool {
    valid_text(value)
        && reqwest::Url::parse(value).is_ok_and(|url| {
            matches!(url.scheme(), "http" | "https")
                && url.host_str().is_some()
                && url.username().is_empty()
                && url.password().is_none()
        })
}

fn valid_text(value: &str) -> bool {
    !value.trim().is_empty() && !value.chars().any(char::is_control)
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn issue(
    line: usize,
    field: impl Into<String>,
    message: impl Into<String>,
) -> KnowledgeValidationError {
    KnowledgeValidationError {
        line,
        field: field.into(),
        message: message.into(),
    }
}

fn required_nullable<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    Option::<String>::deserialize(deserializer)
}

fn required_value<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Value, D::Error> {
    Value::deserialize(deserializer)
}

struct UniqueJsonSeed {
    path: String,
}

impl<'de> DeserializeSeed<'de> for UniqueJsonSeed {
    type Value = Value;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Value, D::Error> {
        deserializer.deserialize_any(self)
    }
}

impl<'de> Visitor<'de> for UniqueJsonSeed {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("JSON ohne doppelte Objektschlüssel")
    }

    fn visit_bool<E: serde::de::Error>(self, value: bool) -> Result<Value, E> {
        Ok(Value::Bool(value))
    }

    fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Value, E> {
        Ok(Value::Number(value.into()))
    }

    fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Value, E> {
        Ok(Value::Number(value.into()))
    }

    fn visit_f64<E: serde::de::Error>(self, value: f64) -> Result<Value, E> {
        serde_json::Number::from_f64(value)
            .map(Value::Number)
            .ok_or_else(|| E::custom("Nicht-endliche JSON-Zahl"))
    }

    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Value, E> {
        Ok(Value::String(value.into()))
    }

    fn visit_string<E: serde::de::Error>(self, value: String) -> Result<Value, E> {
        Ok(Value::String(value))
    }

    fn visit_unit<E: serde::de::Error>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<Value, A::Error> {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element_seed(UniqueJsonSeed {
            path: format!("{}[{}]", self.path, values.len()),
        })? {
            values.push(value);
        }
        Ok(Value::Array(values))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Value, A::Error> {
        let mut values = Map::new();
        while let Some(key) = map.next_key::<String>()? {
            if values.contains_key(&key) {
                return Err(A::Error::custom(format!(
                    "Doppelter Objektschlüssel in {}.{key}",
                    self.path
                )));
            }
            let value = map.next_value_seed(UniqueJsonSeed {
                path: format!("{}.{key}", self.path),
            })?;
            values.insert(key, value);
        }
        Ok(Value::Object(values))
    }
}
