#[derive(Debug, Clone)]
pub struct RawObject {
    pub id: i64,
    pub source: String,
    pub revision: String,
    pub path: String,
    pub content_sha256: [u8; 32],
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Receipt {
    pub locator_kind: String,
    pub locator: String,
    pub value_sha256: [u8; 32],
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub kind: String,
    pub key: String,
    pub receipt: Receipt,
}

#[derive(Debug, Clone)]
pub struct Alias {
    pub entity_key: String,
    pub alias: String,
    pub lang: String,
    pub receipt: Receipt,
}

#[derive(Debug, Clone)]
pub enum FactValue {
    Number(f64),
    Text(String),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct Fact {
    pub entity_key: String,
    pub stat_key: String,
    pub value: FactValue,
    pub receipt: Receipt,
}

#[derive(Debug, Clone)]
pub struct Relation {
    pub from_key: String,
    pub kind: String,
    pub to_key: String,
    pub receipt: Receipt,
}

#[derive(Debug, Clone, Default)]
pub struct FactSet {
    pub entities: Vec<Entity>,
    pub aliases: Vec<Alias>,
    pub facts: Vec<Fact>,
    pub relations: Vec<Relation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rejection {
    pub code: &'static str,
    pub detail: String,
}

impl Rejection {
    pub fn new(code: &'static str, detail: impl Into<String>) -> Self {
        let mut detail = detail.into().replace('\0', "�");
        let mut end = detail.len().min(super::limits::DETAIL_BYTES);
        while !detail.is_char_boundary(end) {
            end -= 1;
        }
        detail.truncate(end);
        Self { code, detail }
    }
}
