use super::{BuildObservation, MechanicAxis};
use crate::{ConditionKind, ItemModel};
use std::collections::{BTreeMap, BTreeSet};

/// Descriptive mechanical features; these never multiply an item score.
pub fn item_axes(item: &ItemModel) -> BTreeSet<MechanicAxis> {
    use MechanicAxis::*;
    let mut axes = BTreeSet::new();
    for (property, kind) in &item.property_damage_types {
        if item
            .properties
            .get(property)
            .is_none_or(|value| *value == 0.0)
        {
            continue;
        }
        match kind {
            crate::DamageType::Weapon => {
                axes.insert(Weapon);
            }
            crate::DamageType::Spirit => {
                axes.insert(Spirit);
            }
            crate::DamageType::Hybrid => {
                axes.insert(Weapon);
                axes.insert(Spirit);
            }
            crate::DamageType::None => {}
        }
    }
    if matches!(item.condition, ConditionKind::MeleeBound)
        || matches!(&item.condition, ConditionKind::ActionBound { action } if action == "melee")
    {
        axes.insert(Melee);
    }
    for (key, value) in item.properties.iter().chain(&item.passive_properties) {
        if !value.is_finite() || *value == 0.0 {
            continue;
        }
        let key = key.to_ascii_lowercase();
        if key.contains("melee") {
            axes.insert(Melee);
        }
        if key.contains("firerate")
            || key.contains("clipsize")
            || key.contains("baseattackdamage")
            || key.contains("headshot")
            || key.contains("bulletdamage")
        {
            axes.insert(Weapon);
        }
        if [
            "techpower",
            "spiritpower",
            "bonusspirit",
            "bonusspiritpower",
            "spirit",
        ]
        .contains(&key.as_str())
            || key.contains("explosiondamage")
            || key.contains("techdamage")
        {
            axes.insert(Spirit);
        }
        if key.contains("cooldown") {
            axes.insert(Cooldown);
        }
        if key.contains("duration") {
            axes.insert(Duration);
        }
        if key.contains("charge") {
            axes.insert(Charges);
        }
        if key.contains("movespeed")
            || key.contains("sprintspeed")
            || key.contains("stamina")
            || key.contains("dash")
        {
            axes.insert(Mobility);
        }
        if key.contains("lifesteal") || key.contains("regeneration") || key.contains("heal") {
            axes.insert(Sustain);
        }
        if key.contains("ally")
            || key.contains("rescue")
            || key.contains("healamp")
            || key.contains("healingamplification")
        {
            axes.insert(Support);
        }
        if key.contains("health")
            || key.contains("shield")
            || key.contains("barrier")
            || key.contains("resist")
        {
            axes.insert(Defense);
        }
        if key.contains("slow")
            || key.contains("silence")
            || key.contains("stun")
            || key.contains("root")
            || key.contains("disarm")
        {
            axes.insert(Control);
        }
        if key.contains("radius") || key.contains("range") {
            axes.insert(Range);
        }
    }
    if item.is_active {
        axes.insert(Active);
    }
    axes
}

#[derive(Clone, Debug, Default)]
pub(super) struct Features {
    pub roots: BTreeMap<i64, f64>,
    pub axes: BTreeMap<MechanicAxis, f64>,
    pub imbues: BTreeMap<(i64, i64), f64>,
    pub focus: BTreeMap<i64, f64>,
}

pub(super) fn upgrade_roots(items: &[ItemModel]) -> BTreeMap<i64, i64> {
    let classes: BTreeMap<_, _> = items
        .iter()
        .map(|i| (i.class_name.as_str(), i.item_id))
        .collect();
    let parents: BTreeMap<_, _> = items
        .iter()
        .filter_map(|i| {
            // Multi-component recipes do not make all parents interchangeable.
            (i.component_items.len() == 1)
                .then(|| {
                    classes
                        .get(i.component_items[0].as_str())
                        .map(|p| (i.item_id, *p))
                })
                .flatten()
        })
        .collect();
    items
        .iter()
        .map(|i| {
            let mut id = i.item_id;
            let mut seen = BTreeSet::new();
            while seen.insert(id) {
                match parents.get(&id) {
                    Some(parent) => id = *parent,
                    None => break,
                }
            }
            if parents.contains_key(&id) {
                id = i.item_id;
            }
            (i.item_id, id)
        })
        .collect()
}

pub(super) fn features(
    o: &BuildObservation,
    catalog: &BTreeMap<i64, &ItemModel>,
    roots: &BTreeMap<i64, i64>,
) -> Features {
    let mut f = Features::default();
    for id in &o.items {
        let Some(item) = catalog.get(id) else {
            continue;
        };
        let root = *roots.get(id).unwrap_or(id);
        let weight: f64 = if item.tier >= 3 { 1.5 } else { 1.0 };
        f.roots
            .entry(root)
            .and_modify(|w| *w = w.max(weight))
            .or_insert(weight);
        for axis in item_axes(item) {
            *f.axes.entry(axis).or_default() += 1.0;
        }
        if let Some(target) = o.imbues.get(id) {
            f.imbues.insert((root, *target), 1.0);
        }
    }
    if !o.items.is_empty() {
        for v in f.axes.values_mut() {
            *v /= o.items.len() as f64;
        }
    }
    // Unlock permutations alone do not identify a playstyle. First completed T3
    // is a stronger signal; incomplete sequences retain an unknown focus.
    let mut ranks = BTreeMap::<i64, usize>::new();
    for step in &o.skill_order {
        let count = ranks.entry(step.ability_id).or_default();
        *count += 1;
        if *count == 4 {
            f.focus.insert(step.ability_id, 1.0);
            break;
        }
    }
    f
}

fn soft_jaccard<K: Ord>(a: &BTreeMap<K, f64>, b: &BTreeMap<K, f64>) -> Option<f64> {
    if a.is_empty() || b.is_empty() {
        return None;
    }
    let intersection: f64 = a
        .iter()
        .map(|(k, v)| v.min(*b.get(k).unwrap_or(&0.0)))
        .sum();
    let union = a.values().sum::<f64>() + b.values().sum::<f64>() - intersection;
    (union > 0.0).then_some(intersection / union)
}

fn reliable_target(f: &Features, root: i64) -> Option<i64> {
    let values: Vec<_> = f.imbues.iter().filter(|((r, _), _)| *r == root).collect();
    let total: f64 = values.iter().map(|(_, v)| **v).sum();
    let (key, weight) = values
        .into_iter()
        .max_by(|a, b| a.1.total_cmp(b.1).then_with(|| b.0.cmp(a.0)))?;
    (total >= 0.25 && *weight / total >= 0.70).then_some(key.1)
}

pub(super) fn similarity(a: &Features, b: &Features) -> f64 {
    let Some(items) = soft_jaccard(&a.roots, &b.roots) else {
        return 0.0;
    };
    similarity_with_items(a, b, items)
}

fn similarity_with_items(a: &Features, b: &Features, items: f64) -> f64 {
    let mut sum = 0.70 * items;
    let mut weight = 0.70;
    if let Some(v) = soft_jaccard(&a.axes, &b.axes) {
        sum += 0.15 * v;
        weight += 0.15;
    }
    if let Some(v) = soft_jaccard(&a.imbues, &b.imbues) {
        sum += 0.10 * v;
        weight += 0.10;
    }
    if let Some(v) = soft_jaccard(&a.focus, &b.focus) {
        sum += 0.05 * v;
        weight += 0.05;
    }
    let mut value = sum / weight;
    let conflict = a
        .imbues
        .keys()
        .map(|(r, _)| *r)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .any(
            |r| matches!((reliable_target(a,r), reliable_target(b,r)), (Some(x),Some(y)) if x != y),
        );
    if conflict {
        value *= 0.55;
    }
    if soft_jaccard(&a.focus, &b.focus).is_some_and(|v| v < 0.25) {
        value *= 0.80;
    }
    value
}

pub(super) fn centroid(members: &[usize], values: &[Features]) -> Features {
    let mut f = Features::default();
    let n = members.len() as f64;
    for i in members {
        for (k, v) in &values[*i].roots {
            *f.roots.entry(*k).or_default() += v / n;
        }
        for (k, v) in &values[*i].axes {
            *f.axes.entry(*k).or_default() += v / n;
        }
        for (k, v) in &values[*i].imbues {
            *f.imbues.entry(*k).or_default() += v / n;
        }
        for (k, v) in &values[*i].focus {
            *f.focus.entry(*k).or_default() += v / n;
        }
    }
    f
}

/// Rare situational tails must not create a second playstyle. A core is the
/// existing 70% population staple definition, applied to upgrade families.
pub(super) fn core_centroid(members: &[usize], values: &[Features]) -> Features {
    let mut center = centroid(members, values);
    let mut counts = BTreeMap::<i64, usize>::new();
    for i in members {
        for id in values[*i].roots.keys() {
            *counts.entry(*id).or_default() += 1;
        }
    }
    center.roots.retain(|id, _| {
        counts[id] as f64 / members.len() as f64 >= dbrain_population::STAPLE_THRESHOLD
    });
    center
}

pub(super) fn merge_similarity(
    a: &Features,
    b: &Features,
    core_a: &Features,
    core_b: &Features,
) -> f64 {
    let full = similarity(a, b);
    if core_a.roots.len() < 3 || core_b.roots.len() < 3 {
        return full;
    }
    let intersection: f64 = core_a
        .roots
        .iter()
        .map(|(id, v)| v.min(*core_b.roots.get(id).unwrap_or(&0.0)))
        .sum();
    let smaller = core_a
        .roots
        .values()
        .sum::<f64>()
        .min(core_b.roots.values().sum::<f64>());
    if smaller <= 0.0 {
        return full;
    }
    // Containment allows a shorter core with one flex extension to remain the
    // same family. Mechanical and reliable binding conflicts are still applied.
    full.max(similarity_with_items(
        core_a,
        core_b,
        intersection / smaller,
    ))
}

pub(super) fn family_label(axes: &BTreeMap<MechanicAxis, f64>) -> String {
    use MechanicAxis::*;
    let get = |axis| axes.get(&axis).copied().unwrap_or(0.0);
    if get(Melee) >= 0.18 {
        return "Melee / Brawler".into();
    }
    if get(Support) >= 0.12 && get(Weapon) < 0.25 {
        return "Utility / Support".into();
    }
    if get(Weapon) >= 0.35 && get(Spirit) >= 0.20 {
        return "Gun / Spirit Hybrid".into();
    }
    if get(Weapon) >= 0.35 {
        return "Gun Carry".into();
    }
    if get(Spirit) >= 0.25 && get(Control) >= 0.15 {
        return "Spirit / Control".into();
    }
    if get(Spirit) >= 0.25 {
        return "Spirit / Ability".into();
    }
    if get(Defense) >= 0.45 {
        return "Tank / Frontline".into();
    }
    if get(Sustain) >= 0.30 {
        return "Sustain Brawler".into();
    }
    "Hybrid – mechanische Rolle nicht eindeutig".into()
}
