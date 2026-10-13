use super::{invalid, Rejection};
use serde_json::value::RawValue;
use std::collections::BTreeMap;

struct Node {
    start: u32,
    end: u32,
    next: u32,
}

pub(super) struct Index<'a> {
    text: &'a str,
    nodes: Vec<Node>,
    keys: Vec<(u32, String, u32)>,
    arrays: BTreeMap<u32, Vec<u32>>,
}

impl<'a> Index<'a> {
    pub(super) fn parse(bytes: &'a [u8]) -> Result<Option<Self>, Rejection> {
        let Ok(raw) = serde_json::from_slice::<&RawValue>(bytes) else {
            return Ok(None);
        };
        let mut index = Self {
            text: raw.get(),
            nodes: Vec::new(),
            keys: Vec::new(),
            arrays: BTreeMap::new(),
        };
        u32::try_from(index.text.len()).map_err(|_| invalid())?;
        if index.scan().is_err() {
            return Ok(None);
        }
        index
            .keys
            .sort_unstable_by(|a, b| (a.0, &a.1).cmp(&(b.0, &b.1)));
        if index
            .keys
            .windows(2)
            .any(|pair| pair[0].0 == pair[1].0 && pair[0].1 == pair[1].1)
        {
            return Err(Rejection::new(
                "locator_collision",
                "Rohobjekt enthält mehrdeutige JSON-Schlüssel",
            ));
        }
        Ok(Some(index))
    }

    fn space(&self, offset: &mut usize) {
        while self
            .text
            .as_bytes()
            .get(*offset)
            .is_some_and(|b| matches!(b, b' ' | b'\t' | b'\r' | b'\n'))
        {
            *offset += 1;
        }
    }

    fn string(&self, offset: &mut usize) -> Result<(), Rejection> {
        *offset += 1;
        loop {
            match self.text.as_bytes().get(*offset) {
                Some(b'\\') => *offset += 2,
                Some(b'"') => {
                    *offset += 1;
                    return Ok(());
                }
                Some(_) => *offset += 1,
                None => return Err(invalid()),
            }
        }
    }

    fn key(&mut self, object: u32, offset: &mut usize) -> Result<(), Rejection> {
        self.space(offset);
        let start = *offset;
        self.string(offset)?;
        let key = serde_json::from_str(&self.text[start..*offset]).map_err(|_| invalid())?;
        self.keys.push((object, key, self.nodes.len() as u32));
        self.space(offset);
        *offset += 1;
        Ok(())
    }

    fn finish(&mut self, node: u32, offset: usize) {
        let next = self.nodes.len() as u32;
        self.nodes[node as usize].end = offset as u32;
        self.nodes[node as usize].next = next;
    }

    fn scan(&mut self) -> Result<(), Rejection> {
        let mut offset = 0;
        let mut stack = Vec::<u32>::new();
        loop {
            self.space(&mut offset);
            let node = self.nodes.len() as u32;
            self.nodes.push(Node {
                start: offset as u32,
                end: 0,
                next: 0,
            });
            match self.text.as_bytes().get(offset) {
                Some(open @ (b'{' | b'[')) => {
                    let object = *open == b'{';
                    offset += 1;
                    self.space(&mut offset);
                    if self.text.as_bytes().get(offset) == Some(if object { &b'}' } else { &b']' })
                    {
                        offset += 1;
                    } else {
                        stack.push(node);
                        if object {
                            self.key(node, &mut offset)?;
                        }
                        continue;
                    }
                }
                Some(b'"') => self.string(&mut offset)?,
                Some(_) => {
                    while self.text.as_bytes().get(offset).is_some_and(|b| {
                        !matches!(b, b' ' | b'\t' | b'\r' | b'\n' | b',' | b']' | b'}')
                    }) {
                        offset += 1;
                    }
                }
                None => return Err(invalid()),
            }
            self.finish(node, offset);
            loop {
                let Some(&parent) = stack.last() else {
                    return Ok(());
                };
                self.space(&mut offset);
                let object =
                    self.text.as_bytes()[self.nodes[parent as usize].start as usize] == b'{';
                if self.text.as_bytes().get(offset) == Some(if object { &b'}' } else { &b']' }) {
                    offset += 1;
                    stack.pop();
                    self.finish(parent, offset);
                    continue;
                }
                offset += 1;
                if object {
                    self.key(parent, &mut offset)?;
                }
                break;
            }
        }
    }

    pub(super) fn value(&mut self, pointer: &str) -> Result<&'a [u8], Rejection> {
        let mut node = 0_u32;
        if !pointer.is_empty() {
            for token in pointer.strip_prefix('/').ok_or_else(invalid)?.split('/') {
                let key = token.replace("~1", "/").replace("~0", "~");
                let parent = &self.nodes[node as usize];
                node = match self.text.as_bytes()[parent.start as usize] {
                    b'{' => {
                        let index = self
                            .keys
                            .binary_search_by(|entry| {
                                (entry.0, entry.1.as_str()).cmp(&(node, key.as_str()))
                            })
                            .map_err(|_| invalid())?;
                        self.keys[index].2
                    }
                    b'[' => {
                        if key.len() > 1 && key.starts_with('0') {
                            return Err(invalid());
                        }
                        let index = key.parse::<usize>().map_err(|_| invalid())?;
                        let children = self.arrays.entry(node).or_insert_with(|| {
                            let mut children = Vec::new();
                            let mut child = node + 1;
                            while child < parent.next {
                                children.push(child);
                                child = self.nodes[child as usize].next;
                            }
                            children
                        });
                        *children.get(index).ok_or_else(invalid)?
                    }
                    _ => return Err(invalid()),
                };
            }
        }
        let value = &self.nodes[node as usize];
        Ok(&self.text.as_bytes()[value.start as usize..value.end as usize])
    }
}
