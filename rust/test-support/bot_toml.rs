#![allow(dead_code)]
use serde_json::Value;

fn remove_nulls(value: &mut Value) {
    match value {
        Value::Object(table) => {
            table.retain(|_, value| !value.is_null());
            for value in table.values_mut() {
                remove_nulls(value);
            }
        }
        Value::Array(values) => {
            for value in values {
                remove_nulls(value);
            }
        }
        _ => {}
    }
}

pub fn section(value: &Value, path: &[&str]) -> Vec<u8> {
    let mut root = value.clone();
    remove_nulls(&mut root);
    for key in path.iter().rev() {
        let mut table = serde_json::Map::new();
        table.insert((*key).to_string(), root);
        root = Value::Object(table);
    }
    toml::to_string(&root).unwrap().into_bytes()
}

pub fn merge(path: &std::path::Path, value: &Value, keys: &[&str]) -> Vec<u8> {
    let mut root = std::fs::read_to_string(path)
        .ok()
        .map(|text| toml::from_str::<toml::Value>(&text).unwrap())
        .unwrap_or_else(|| toml::Value::Table(Default::default()));
    let mut new = value.clone();
    remove_nulls(&mut new);
    let mut cursor = &mut root;
    for key in &keys[..keys.len() - 1] {
        cursor = cursor
            .as_table_mut()
            .unwrap()
            .entry(key.to_string())
            .or_insert_with(|| toml::Value::Table(Default::default()));
    }
    let key = keys[keys.len() - 1];
    let mut new = toml::Value::try_from(new).unwrap();
    if let Some(runtime) = cursor.get(key).and_then(|old| old.get("runtime")) {
        new.as_table_mut()
            .unwrap()
            .insert("runtime".into(), runtime.clone());
    }
    cursor.as_table_mut().unwrap().insert(key.into(), new);
    toml::to_string(&root).unwrap().into_bytes()
}
