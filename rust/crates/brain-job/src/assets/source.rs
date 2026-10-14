use crate::{InputObject, InputObjects};
use anyhow::{anyhow, Context};
use dbrain_sources::assets_api::{BASE_URL, ENDPOINTS};
use deadlock_brain_core::http::{HttpClient, SourceHttpOptions};
use serde_json::value::RawValue;
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    io::Read,
    path::{Path, PathBuf},
};

struct Payload {
    kind: &'static str,
    language: &'static str,
    revision: String,
    bytes: Vec<u8>,
    ranges: Option<VecDeque<(usize, usize, usize)>>,
    collisions: BTreeSet<usize>,
    snapshot_pending: bool,
    parent_offset: usize,
}
struct Objects {
    cache: PathBuf,
    fetch: bool,
    heroes_pending: bool,
    languages: std::array::IntoIter<&'static str, 2>,
    payload: Option<Payload>,
}

pub fn load(cache: &Path, fetch: bool) -> InputObjects {
    Box::new(Objects {
        cache: cache.into(),
        fetch,
        heroes_pending: fetch || cache.join("heroes-en.json").is_file(),
        languages: ["en", "de"].into_iter(),
        payload: None,
    })
}
fn get(
    cache: PathBuf,
    kind: &'static str,
    language: &'static str,
    fetch: bool,
) -> anyhow::Result<Vec<u8>> {
    if !fetch {
        let limit = SourceHttpOptions::default().max_bytes;
        let mut bytes = Vec::new();
        let filename = if kind == "heroes" {
            format!("heroes-{language}.json")
        } else {
            format!("{language}.json")
        };
        std::fs::File::open(cache.join(filename))
            .context("Assets-Testquelle fehlt")?
            .take((limit + 1) as u64)
            .read_to_end(&mut bytes)?;
        anyhow::ensure!(
            bytes.len() <= limit,
            "Assets-Testquelle überschreitet die Transportgrenze"
        );
        return Ok(bytes);
    }
    std::thread::spawn(move || {
        let endpoint = ENDPOINTS
            .iter()
            .find(|(name, _)| *name == kind)
            .context("Assets-Endpunkt fehlt")?
            .1;
        let language = if language == "de" {
            "german"
        } else {
            "english"
        };
        let client = HttpClient::new("brain-job/assets", cache)?;
        let separator = if endpoint.contains('?') { '&' } else { '?' };
        let response = client.get_bounded(
            &format!("{BASE_URL}{endpoint}{separator}language={language}"),
            SourceHttpOptions::default(),
        )?;
        anyhow::ensure!(
            response.status == 200,
            "Assets-Abfrage fehlgeschlagen: HTTP {}",
            response.status
        );
        Ok(response.content)
    })
    .join()
    .map_err(|_| anyhow!("Assets-Transport fehlgeschlagen"))?
}
fn ranges(bytes: &[u8]) -> anyhow::Result<VecDeque<(usize, usize, usize)>> {
    let rows: Vec<&RawValue> = serde_json::from_slice(bytes)?;
    Ok(rows
        .into_iter()
        .enumerate()
        .map(|(index, raw)| {
            (
                index,
                raw.get().as_ptr() as usize - bytes.as_ptr() as usize,
                raw.get().len(),
            )
        })
        .collect())
}
fn collisions(kind: &str, bytes: &[u8], rows: &VecDeque<(usize, usize, usize)>) -> BTreeSet<usize> {
    #[derive(serde::Deserialize)]
    struct Identity {
        class_name: String,
    }
    let mut identities = BTreeMap::<String, Vec<usize>>::new();
    let mut ids = BTreeMap::<u64, Vec<usize>>::new();
    for &(index, offset, length) in rows {
        let bytes = &bytes[offset..offset + length];
        let class_name = if kind == "items" {
            let Ok(raw) = serde_json::from_slice::<&RawValue>(bytes) else {
                continue;
            };
            let Ok(Some((id, class_name, _))) = super::identity(raw) else {
                continue;
            };
            ids.entry(id).or_default().push(index);
            class_name
        } else if let Ok(value) = serde_json::from_slice::<Identity>(bytes) {
            value.class_name
        } else {
            continue;
        };
        identities
            .entry(crate::limits::normalize_key(&class_name))
            .or_default()
            .push(index);
    }
    identities
        .into_values()
        .chain(ids.into_values())
        .filter(|positions| positions.len() > 1)
        .flatten()
        .collect()
}

impl Iterator for Objects {
    type Item = anyhow::Result<InputObject>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(payload) = &mut self.payload {
                if payload.parent_offset < payload.bytes.len() || payload.parent_offset == 0 {
                    let offset = payload.parent_offset;
                    let end = (offset + crate::limits::RAW_BYTES).min(payload.bytes.len());
                    payload.parent_offset = end.max(1);
                    return Some(Ok(InputObject {
                        revision: payload.revision.clone(),
                        path: format!(
                            "{}/{}/{}/{offset}.response",
                            payload.kind,
                            payload.language,
                            payload
                                .revision
                                .strip_prefix("sha256:")
                                .unwrap_or(&payload.revision)
                        ),
                        bytes: payload.bytes[offset..end].to_vec(),
                    }));
                }
                if payload.ranges.is_none() {
                    match ranges(&payload.bytes) {
                        Ok(rows) => {
                            payload.collisions = collisions(payload.kind, &payload.bytes, &rows);
                            payload.ranges = Some(rows);
                        }
                        Err(_) => {
                            let oversized = payload.bytes.len() > crate::limits::RAW_BYTES;
                            let object = InputObject {
                                revision: payload.revision.clone(),
                                path: if oversized {
                                    format!(
                                        "{}/{}.invalid-response",
                                        payload.kind, payload.language
                                    )
                                } else {
                                    format!("{}/{}.json", payload.kind, payload.language)
                                },
                                bytes: if oversized {
                                    payload.bytes[..crate::limits::RAW_BYTES].to_vec()
                                } else {
                                    std::mem::take(&mut payload.bytes)
                                },
                            };
                            self.payload = None;
                            return Some(Ok(object));
                        }
                    }
                }
                if payload.kind == "heroes" && std::mem::take(&mut payload.snapshot_pending) {
                    return Some(Ok(InputObject {
                        revision: payload.revision.clone(),
                        path: "heroes/en/observed/complete.snapshot".into(),
                        bytes: payload.revision.as_bytes().to_vec(),
                    }));
                }
                if let Some((index, offset, length)) =
                    payload.ranges.as_mut().and_then(VecDeque::pop_front)
                {
                    let oversized = length > crate::limits::RAW_BYTES;
                    let end = offset + length.min(crate::limits::RAW_BYTES);
                    let mut bytes = payload.bytes[offset..end].to_vec();
                    if payload.kind == "heroes" && !oversized && bytes.last() == Some(&b'}') {
                        bytes.pop();
                        bytes.extend_from_slice(
                            format!(",\"__brain_response_revision\":\"{}\"}}", payload.revision)
                                .as_bytes(),
                        );
                    }
                    let oversized = oversized || bytes.len() > crate::limits::RAW_BYTES;
                    bytes.truncate(crate::limits::RAW_BYTES);
                    return Some(Ok(InputObject {
                        revision: payload.revision.clone(),
                        path: format!(
                            "{}/{}/{index}.{}",
                            payload.kind,
                            payload.language,
                            if oversized {
                                "oversized"
                            } else if payload.collisions.contains(&index) {
                                "collision"
                            } else {
                                "json"
                            }
                        ),
                        bytes,
                    }));
                }
                self.payload = None;
            }
            let (kind, language) = if let Some(language) = self.languages.next() {
                ("items", language)
            } else if std::mem::take(&mut self.heroes_pending) {
                ("heroes", "en")
            } else {
                return None;
            };
            match get(self.cache.clone(), kind, language, self.fetch) {
                Ok(bytes) => {
                    let revision = format!(
                        "sha256:{}",
                        brain_storage::entity_profile::derivation::sha256(&bytes)
                    );
                    self.payload = Some(Payload {
                        kind,
                        language,
                        revision,
                        bytes,
                        ranges: None,
                        collisions: BTreeSet::new(),
                        snapshot_pending: true,
                        parent_offset: 0,
                    });
                }
                Err(error) => return Some(Err(error)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn identical_archive_parts_remain_bound_to_their_complete_response_revision() {
        let first_part = |ending| {
            let mut bytes = vec![b' '; crate::limits::RAW_BYTES + 1];
            bytes[crate::limits::RAW_BYTES] = ending;
            let revision = format!(
                "sha256:{}",
                brain_storage::entity_profile::derivation::sha256(&bytes)
            );
            Objects {
                cache: PathBuf::new(),
                fetch: false,
                heroes_pending: false,
                languages: ["en", "de"].into_iter(),
                payload: Some(Payload {
                    kind: "items",
                    language: "en",
                    revision,
                    bytes,
                    ranges: None,
                    collisions: BTreeSet::new(),
                    snapshot_pending: true,
                    parent_offset: 0,
                }),
            }
            .next()
            .expect("part")
            .expect("archive")
        };
        let first = first_part(b'0');
        let second = first_part(b'1');
        assert!(first.bytes == second.bytes);
        assert_ne!(first.revision, second.revision);
        assert_ne!(first.path, second.path);
    }
    #[test]
    fn full_responses_are_archived_before_parsing_and_oversized_objects_do_not_hide_peers() {
        use crate::{assets::Assets, Extractor, Outcome, RawObject};
        use sha2::{Digest, Sha256};
        let bytes = serde_json::to_vec(&serde_json::json!([
            {"id":1,"class_name":"large","type":"ability","image":"x".repeat(crate::limits::RAW_BYTES)},
            {"id":2,"class_name":"valid","type":"ability"}
        ])).expect("public-shaped input");
        let mut objects = Objects {
            cache: PathBuf::new(),
            fetch: false,
            heroes_pending: false,
            languages: ["en", "de"].into_iter(),
            payload: Some(Payload {
                kind: "items",
                language: "en",
                revision: "fixture".into(),
                bytes: bytes.clone(),
                ranges: None,
                collisions: BTreeSet::new(),
                snapshot_pending: true,
                parent_offset: 0,
            }),
        };
        let mut archived = Vec::new();
        for _ in 0..2 {
            let part = objects.next().expect("part").expect("raw archive part");
            assert!(part.path.ends_with(".response"));
            assert!(part.bytes.len() <= crate::limits::RAW_BYTES);
            archived.extend(part.bytes);
            assert!(objects
                .payload
                .as_ref()
                .is_some_and(|payload| payload.ranges.is_none()));
        }
        assert!(archived == bytes);
        let raw = |object: InputObject| RawObject {
            id: 1,
            source: "assets".into(),
            revision: object.revision,
            path: object.path,
            content_sha256: Sha256::digest(&object.bytes).into(),
            bytes: object.bytes,
        };
        let oversized = raw(objects
            .next()
            .expect("large record")
            .expect("bounded rejection marker"));
        assert!(
            matches!(Assets.extract(&oversized),Outcome::Rejected(error) if error.code=="raw_too_large")
        );
        let valid = raw(objects
            .next()
            .expect("valid peer")
            .expect("exact object bytes"));
        assert!(matches!(Assets.extract(&valid), Outcome::Accepted(_)));
    }
    #[test]
    fn only_valid_hero_arrays_emit_complete_snapshot_markers() {
        for (bytes, valid) in [
            (b"[not-json".as_slice(), false),
            (b"{}".as_slice(), false),
            (b"[]".as_slice(), true),
        ] {
            let mut objects = Objects {
                cache: PathBuf::new(),
                fetch: false,
                heroes_pending: false,
                languages: ["en", "de"].into_iter(),
                payload: Some(Payload {
                    kind: "heroes",
                    language: "en",
                    revision: "fixture".into(),
                    bytes: bytes.to_vec(),
                    ranges: None,
                    collisions: BTreeSet::new(),
                    snapshot_pending: true,
                    parent_offset: 0,
                }),
            };
            let archive = objects.next().expect("archive").expect("object");
            assert!(archive.bytes == bytes);
            assert!(archive.path.ends_with(".response"));
            let next = objects.next().expect("validation outcome").expect("object");
            assert_eq!(next.path.ends_with("complete.snapshot"), valid);
            if !valid {
                assert!(objects.payload.is_none());
                assert!(next.bytes == bytes);
            }
        }
    }
    #[test]
    fn array_ranges_preserve_exact_object_bytes_and_reject_malformed_containers() {
        let bytes = br#" [ {"value":"a\/b"}, {"value":1.2300e+2} ] "#;
        let rows = ranges(bytes).expect("array");
        assert_eq!(rows.len(), 2);
        let values: Vec<_> = rows
            .into_iter()
            .map(|(_, start, len)| &bytes[start..start + len])
            .collect();
        assert_eq!(
            values,
            [br#"{"value":"a\/b"}"#.as_slice(), br#"{"value":1.2300e+2}"#]
        );
        for bytes in [br#"{ "id": 1 }"#.as_slice(), b"[not-json"] {
            assert!(ranges(bytes).is_err());
        }
    }
}
