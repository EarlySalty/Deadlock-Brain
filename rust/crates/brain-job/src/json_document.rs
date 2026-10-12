use super::{json_text, kv3::Leaf};
use serde::de::{Error, MapAccess, Visitor};
use serde_json::value::RawValue;
use std::{collections::BTreeSet, fmt};

pub(super) fn parse(input: &str) -> Result<Vec<Leaf>, serde_json::Error> {
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let leaves = serde::Deserializer::deserialize_map(&mut deserializer, Document)?;
    deserializer.end()?;
    Ok(leaves)
}

struct Document;

impl<'de> Visitor<'de> for Document {
    type Value = Vec<Leaf>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a source record map")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut keys = BTreeSet::new();
        let mut leaves = Vec::new();
        let mut remaining = 32 * 1024 * 1024usize;
        while let Some((key, value)) = map.next_entry::<String, &RawValue>()? {
            if key.len() > 512 || keys.len() >= 100000 || !keys.insert(key.clone()) {
                return Err(M::Error::custom("json_root_key"));
            }
            if value.get().len() > 2 * 1024 * 1024 {
                return Err(M::Error::custom("json_member_limit"));
            }
            let escaped = serde_json::to_string(&key).map_err(M::Error::custom)?;
            let wrapped = format!("{{{escaped}:{value}}}");
            let parsed = json_text::parse(&wrapped).map_err(M::Error::custom)?;
            for leaf in parsed {
                let bytes = leaf
                    .pointer
                    .len()
                    .checked_add(leaf.lexeme.len())
                    .and_then(|n| n.checked_add(leaf.value.to_string().len()))
                    .and_then(|n| n.checked_mul(2))
                    .and_then(|n| n.checked_add(512))
                    .ok_or_else(|| M::Error::custom("json_leaf_budget"))?;
                remaining = remaining
                    .checked_sub(bytes)
                    .ok_or_else(|| M::Error::custom("json_leaf_budget"))?;
                leaves.push(leaf);
            }
        }
        Ok(leaves)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_record_maps_preserve_full_locators_and_validate_all_members() {
        let leaves = parse(r#"{"a/b":{"x~y":26.0},"other":{"text":"value"}}"#).unwrap();
        assert_eq!(leaves[0].pointer, "/a~1b/x~0y");
        assert_eq!(leaves[0].lexeme, "26.0");
        assert!(parse(r#"{"a":{"x":1},"a":{"x":2}}"#).is_err());
        assert!(parse(r#"{"a":{"x":1,"x":2}}"#).is_err());
        assert!(parse(r#"{"a":{"x":1},"other":{"x":invalid}}"#).is_err());
        assert!(parse(r#"{"a":{"x":1}} garbage"#).is_err());
    }
}
