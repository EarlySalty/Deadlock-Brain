use super::kv3::{self, Leaf};
use std::collections::BTreeMap;

pub(super) fn parse(input: &str) -> Result<Vec<Leaf>, &'static str> {
    let body = if input.starts_with("<!--") {
        let end = input.find("-->").ok_or("kv3_header")?;
        &input[end + 3..]
    } else {
        input
    };
    let mut scan = Scanner {
        input: body,
        index: 0,
    };
    scan.space()?;
    scan.expect(b'{')?;
    let mut keys = BTreeMap::<String, usize>::new();
    let mut leaves = Vec::new();
    let mut remaining = 48 * 1024 * 1024usize;
    loop {
        scan.space()?;
        if scan.byte() == Some(b'}') {
            scan.index += 1;
            scan.space()?;
            if scan.index != body.len() {
                return Err("kv3_trailing");
            }
            return Ok(leaves);
        }
        let start = scan.index;
        let key = scan.key()?;
        if key.len() > 512 || keys.len() >= 10000 {
            return Err("kv3_root_key");
        }
        let occurrence = keys.entry(key.clone()).or_default();
        let observed = *occurrence;
        *occurrence += 1;
        scan.space()?;
        scan.expect(b'=')?;
        scan.space()?;
        scan.value()?;
        let member = body.get(start..scan.index).ok_or("kv3_boundary")?;
        if member.len() > 2 * 1024 * 1024 {
            return Err("kv3_member_limit");
        }
        let wrapped = format!("{{{member}}}");
        let parsed = kv3::parse(&wrapped)?;
        let type_pointer = format!(
            "/{}/0/m_eAbilityType/0",
            key.replace('~', "~0").replace('/', "~1")
        );
        let ability = parsed.iter().any(|leaf| {
            leaf.pointer == type_pointer
                && leaf
                    .value
                    .as_str()
                    .is_some_and(super::supported_ability_type)
        });
        for mut leaf in parsed {
            if observed > 0 {
                let escaped = key.replace('~', "~0").replace('/', "~1");
                let prefix = format!("/{escaped}/0");
                let tail = leaf
                    .pointer
                    .strip_prefix(&prefix)
                    .ok_or("kv3_root_pointer")?;
                leaf.pointer = format!("/{escaped}/{observed}{tail}");
            }
            let relevant = leaf.pointer.contains("/m_mapStartingStats/")
                || leaf.pointer.contains("/m_eHeroDevelopmentState/")
                || (ability
                    && (leaf.pointer.contains("/m_mapAbilityProperties/")
                        || leaf.pointer.contains("/m_vecAbilityUpgrades/")))
                || leaf.pointer == type_pointer;
            if relevant {
                let bytes = leaf
                    .pointer
                    .len()
                    .checked_add(leaf.lexeme.len())
                    .and_then(|n| n.checked_add(leaf.value.to_string().len()))
                    .and_then(|n| n.checked_add(leaf.flags.iter().map(String::len).sum()))
                    .and_then(|n| n.checked_mul(2))
                    .and_then(|n| n.checked_add(512))
                    .ok_or("kv3_leaf_budget")?;
                remaining = remaining.checked_sub(bytes).ok_or("kv3_leaf_budget")?;
                leaves.push(leaf);
            }
        }
        scan.space()?;
        if scan.byte() == Some(b',') {
            scan.index += 1;
        }
    }
}

struct Scanner<'a> {
    input: &'a str,
    index: usize,
}

impl Scanner<'_> {
    fn byte(&self) -> Option<u8> {
        self.input.as_bytes().get(self.index).copied()
    }

    fn expect(&mut self, byte: u8) -> Result<(), &'static str> {
        if self.byte() != Some(byte) {
            return Err("kv3_structure");
        }
        self.index += 1;
        Ok(())
    }

    fn space(&mut self) -> Result<(), &'static str> {
        loop {
            while self.byte().is_some_and(|b| b.is_ascii_whitespace()) {
                self.index += 1;
            }
            let rest = self.input.get(self.index..).ok_or("kv3_boundary")?;
            if rest.starts_with("//") {
                self.index += rest.find('\n').unwrap_or(rest.len());
            } else if let Some(comment) = rest.strip_prefix("/*") {
                self.index += comment.find("*/").ok_or("kv3_comment")? + 4;
            } else {
                return Ok(());
            }
        }
    }

    fn string(&mut self) -> Result<(), &'static str> {
        let rest = self.input.get(self.index..).ok_or("kv3_boundary")?;
        if let Some(body) = rest.strip_prefix("\"\"\"") {
            self.index += body.find("\"\"\"").ok_or("kv3_string")? + 6;
            return Ok(());
        }
        self.expect(b'"')?;
        loop {
            match self.byte() {
                Some(b'\\') => {
                    self.index += 2;
                }
                Some(b'"') => {
                    self.index += 1;
                    return Ok(());
                }
                Some(_) => {
                    self.index += 1;
                }
                None => return Err("kv3_string"),
            }
        }
    }

    fn key(&mut self) -> Result<String, &'static str> {
        let start = self.index;
        if self.byte() == Some(b'"') {
            self.string()?;
            return serde_json::from_str(&self.input[start..self.index]).map_err(|_| "kv3_key");
        }
        while self.byte().is_some_and(|b| {
            !b.is_ascii_whitespace()
                && !matches!(b, b'=' | b'{' | b'}' | b'[' | b']' | b',' | b':' | b'"')
        }) {
            self.index += 1;
        }
        if self.index == start {
            return Err("kv3_key");
        }
        self.input
            .get(start..self.index)
            .map(str::to_owned)
            .ok_or("kv3_boundary")
    }

    fn value(&mut self) -> Result<(), &'static str> {
        let start = self.index;
        let mut stack = Vec::new();
        loop {
            self.space()?;
            match self.byte() {
                Some(b'"') => self.string()?,
                Some(b'{') | Some(b'[') => {
                    if stack.len() >= 128 {
                        return Err("kv3_depth");
                    }
                    stack.push(self.byte().ok_or("kv3_structure")?);
                    self.index += 1;
                }
                Some(b'}') | Some(b']') => {
                    let Some(open) = stack.pop() else {
                        return Err("kv3_value");
                    };
                    let close = self.byte().ok_or("kv3_structure")?;
                    if !matches!((open, close), (b'{', b'}') | (b'[', b']')) {
                        return Err("kv3_structure");
                    }
                    self.index += 1;
                    if stack.is_empty() {
                        return Ok(());
                    }
                }
                Some(_) if !stack.is_empty() => {
                    self.index += 1;
                }
                Some(_) => {
                    while self
                        .byte()
                        .is_some_and(|b| !b.is_ascii_whitespace() && !matches!(b, b',' | b'}'))
                    {
                        self.index += 1;
                    }
                    self.space()?;
                    if self.byte() == Some(b'{') || self.byte() == Some(b'[') {
                        continue;
                    }
                    return if self.index > start {
                        Ok(())
                    } else {
                        Err("kv3_value")
                    };
                }
                None => return Err("kv3_value"),
            }
            if stack.is_empty() {
                return Ok(());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_members_keep_full_locators_and_validate_skipped_members() {
        let leaves = parse(r#"<!-- kv3 encoding:text --> { other = { text = "braces } [" } hero_baba = { m_mapStartingStats = { EMaxHealth = 780.0 } } }"#).unwrap();
        assert_eq!(leaves.len(), 1);
        assert_eq!(
            leaves[0].pointer,
            "/hero_baba/0/m_mapStartingStats/0/EMaxHealth/0"
        );
        assert!(parse("{ other = { value = invalid } }").is_err());
        let repeated = parse("{ hero_baba = { m_eHeroDevelopmentState = \"EHeroDevState_Release\" } hero_baba = { m_eHeroDevelopmentState = \"EHeroDevState_HeroLabs\" } }").unwrap();
        assert_eq!(
            repeated[1].pointer,
            "/hero_baba/1/m_eHeroDevelopmentState/0"
        );
        assert!(parse("{ other = { value = 1 ] }").is_err());
    }
}
