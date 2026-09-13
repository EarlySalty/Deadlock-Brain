use crate::{ItemModel, ReasonerError, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InventoryRules {
    pub max_slots: usize,
    pub upgrade_components: BTreeMap<i64, Vec<i64>>,
    pub resale_fraction: f64,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Inventory {
    pub held_ids: BTreeSet<i64>,
    pub spent_souls: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PurchaseTransition {
    pub before: Inventory,
    pub after: Inventory,
    pub purchased_id: i64,
    pub consumed_ids: Vec<i64>,
    pub sold_ids: Vec<i64>,
    pub purchase_cost: i64,
    pub sale_return: i64,
    pub net_cost: i64,
}
impl Inventory {
    pub fn preview_purchase(
        &self,
        item: &ItemModel,
        catalog: &[ItemModel],
        rules: &InventoryRules,
        sale_ids: &[i64],
    ) -> Result<PurchaseTransition> {
        let error = |message: &str| ReasonerError::Data(message.into());
        if !item.shopable
            || item.disabled
            || item.cost <= 0
            || self.held_ids.contains(&item.item_id)
        {
            return Err(error("Item ist nicht kaufbar oder bereits im Inventar"));
        }
        if rules.max_slots == 0
            || !rules.resale_fraction.is_finite()
            || !(0.0..=1.0).contains(&rules.resale_fraction)
        {
            return Err(error("Ungültige Inventarregeln"));
        }
        let sold: BTreeSet<_> = sale_ids.iter().copied().collect();
        if sold.len() != sale_ids.len() || !sold.is_subset(&self.held_ids) {
            return Err(error(
                "Verkauf muss eindeutige tatsächlich gehaltene Items betreffen",
            ));
        }
        let price = |id: &i64| {
            catalog
                .iter()
                .find(|i| i.item_id == *id)
                .map(|i| i.cost)
                .filter(|v| *v > 0)
                .ok_or_else(|| error("Preis eines gehaltenen Items fehlt"))
        };
        let mut after = self.clone();
        let mut sale_return = 0i64;
        for id in &sold {
            sale_return = sale_return
                .checked_add((price(id)? as f64 * rules.resale_fraction).floor() as i64)
                .ok_or_else(|| error("Seelenbetrag zu groß"))?;
            after.held_ids.remove(id);
        }
        let mut consumed_ids = Vec::new();
        let mut component_credit = 0i64;
        let mut seen = BTreeSet::new();
        for id in rules
            .upgrade_components
            .get(&item.item_id)
            .into_iter()
            .flatten()
        {
            if !seen.insert(*id) || *id == item.item_id {
                return Err(error("Ungültige Upgrade-Komponenten"));
            }
            if sold.contains(id) {
                return Err(error(
                    "Upgrade-Komponente darf nicht gleichzeitig verkauft werden",
                ));
            }
            if after.held_ids.remove(id) {
                component_credit = component_credit
                    .checked_add(price(id)?)
                    .ok_or_else(|| error("Seelenbetrag zu groß"))?;
                consumed_ids.push(*id);
            }
        }
        if component_credit > item.cost {
            return Err(error("Komponentenpreis übersteigt Upgrade-Gesamtpreis"));
        }
        after.held_ids.insert(item.item_id);
        if after.held_ids.len() > rules.max_slots {
            return Err(error("Nicht genügend freie Inventarplätze"));
        }
        let purchase_cost = item.cost - component_credit;
        let net_cost = purchase_cost - sale_return;
        after.spent_souls = after
            .spent_souls
            .checked_add(net_cost)
            .ok_or_else(|| error("Seelenbetrag zu groß"))?;
        Ok(PurchaseTransition {
            before: self.clone(),
            after,
            purchased_id: item.item_id,
            consumed_ids,
            sold_ids: sold.into_iter().collect(),
            purchase_cost,
            sale_return,
            net_cost,
        })
    }
    pub fn apply_transition(&mut self, transition: &PurchaseTransition) -> Result<()> {
        if *self != transition.before {
            return Err(ReasonerError::Data(
                "Inventar hat sich seit der Kaufprüfung geändert".into(),
            ));
        }
        *self = transition.after.clone();
        Ok(())
    }
    pub fn held_items(&self, catalog: &[ItemModel]) -> Result<Vec<ItemModel>> {
        self.held_ids
            .iter()
            .map(|id| {
                catalog
                    .iter()
                    .find(|i| i.item_id == *id)
                    .cloned()
                    .ok_or_else(|| {
                        ReasonerError::Data(format!("Inventaritem {id} fehlt im Katalog"))
                    })
            })
            .collect()
    }
}

impl InventoryRules {
    pub fn from_catalog(catalog: &[ItemModel]) -> Result<Self> {
        let mut upgrade_components = BTreeMap::new();
        for item in catalog.iter().filter(|i| i.shopable && !i.disabled) {
            let mut components = Vec::new();
            for name in &item.component_items {
                let component =
                    catalog
                        .iter()
                        .find(|i| i.class_name == *name)
                        .ok_or_else(|| {
                            ReasonerError::Data(format!(
                                "Upgrade-Komponente {name} für {} fehlt im Snapshot",
                                item.name
                            ))
                        })?;
                components.push(component.item_id);
            }
            if !components.is_empty() {
                upgrade_components.insert(item.item_id, components);
            }
        }
        Ok(Self {
            max_slots: 12,
            upgrade_components,
            resale_fraction: 0.5,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn item(id: i64, cost: i64) -> ItemModel {
        serde_json::from_value(serde_json::json!({"item_id":id,"name":format!("Item {id}"),"slot":"Weapon","tier":1,"cost":cost,"is_active":false,"shopable":true,"disabled":false,"damage_axis":"Weapon","defense_kind":[],"properties":{},"passive_properties":{},"condition":"None","proc_cooldown":null,"imbueable":false})).unwrap()
    }
    #[test]
    fn upgrade_consumes_component_and_charges_only_difference() {
        let component = item(1, 800);
        let upgrade = item(2, 3200);
        let catalog = vec![component.clone(), upgrade.clone()];
        let rules = InventoryRules {
            max_slots: 1,
            upgrade_components: BTreeMap::from([(2, vec![1])]),
            resale_fraction: 0.5,
        };
        let mut inventory = Inventory::default();
        inventory
            .apply_transition(
                &inventory
                    .preview_purchase(&component, &catalog, &rules, &[])
                    .unwrap(),
            )
            .unwrap();
        let transition = inventory
            .preview_purchase(&upgrade, &catalog, &rules, &[])
            .unwrap();
        assert_eq!(transition.purchase_cost, 2400);
        assert_eq!(transition.consumed_ids, vec![1]);
        assert_eq!(transition.after.spent_souls, 3200);
        assert_eq!(transition.after.held_ids, BTreeSet::from([2]));
        assert!(inventory
            .preview_purchase(&upgrade, &catalog, &rules, &[1])
            .is_err());
    }
    #[test]
    fn sale_and_purchase_are_atomic_and_missing_slot_is_rejected() {
        let first = item(1, 800);
        let second = item(2, 1600);
        let catalog = vec![first.clone(), second.clone()];
        let rules = InventoryRules {
            max_slots: 1,
            upgrade_components: BTreeMap::new(),
            resale_fraction: 0.5,
        };
        let mut inv = Inventory::default();
        inv.apply_transition(&inv.preview_purchase(&first, &catalog, &rules, &[]).unwrap())
            .unwrap();
        assert!(inv
            .preview_purchase(&second, &catalog, &rules, &[])
            .is_err());
        let transition = inv
            .preview_purchase(&second, &catalog, &rules, &[1])
            .unwrap();
        assert_eq!(transition.net_cost, 1200);
        assert_eq!(inv.held_ids, BTreeSet::from([1]));
        inv.apply_transition(&transition).unwrap();
        assert!(inv.apply_transition(&transition).is_err());
        assert_eq!(inv.held_ids, BTreeSet::from([2]));
    }
}
