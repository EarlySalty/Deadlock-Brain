use serde_json::{Map, Value};

pub fn tag_name(tag: &str) -> String {
    tag.trim_start_matches('/')
        .split(|c: char| c.is_whitespace() || c == '/')
        .next()
        .unwrap_or("")
        .to_ascii_lowercase()
}

const GAME_TAGS: [&str; 34] = [
    "a",
    "b",
    "br",
    "circle",
    "clippath",
    "defs",
    "div",
    "ellipse",
    "em",
    "font",
    "g",
    "i",
    "img",
    "label",
    "li",
    "line",
    "lineargradient",
    "p",
    "panel",
    "path",
    "polygon",
    "polyline",
    "rect",
    "script",
    "span",
    "stop",
    "strong",
    "style",
    "sub",
    "sup",
    "svg",
    "title",
    "u",
    "use",
];

pub fn game_tag(tag: &str) -> bool {
    tag.starts_with("!--") || GAME_TAGS.contains(&tag_name(tag).as_str())
}

pub fn decode(text: &str) -> String {
    text.replace("&nbsp;", " ")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&amp;", "&")
}

pub fn image_alt<'a>(tag: &'a str, after: &str) -> &'a str {
    if labelled_next(after) {
        return "";
    }
    attribute(tag, "alt")
        .filter(|alt| !alt.starts_with('_'))
        .unwrap_or("")
}

fn labelled_next(after: &str) -> bool {
    let after = after.trim_start();
    after.starts_with('<')
        && after[1..].split('>').next().is_some_and(|tag| {
            tag_name(tag) == "span"
                && attribute(tag, "class").is_some_and(|class| {
                    class
                        .split_whitespace()
                        .any(|name| name == "inline-attribute-label")
                })
        })
}

pub fn attribute<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let marker = format!("{name}=\"");
    let start = tag.find(&marker)? + marker.len();
    let end = tag[start..].find('"')?;
    Some(tag[start..start + end].trim())
}

pub fn plain(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(start) = rest.find('<') {
        out.push_str(&rest[..start]);
        let Some(end) = rest[start..].find('>') else {
            out.push_str(&rest[start..]);
            rest = "";
            break;
        };
        let tag = &rest[start + 1..start + end];
        if !game_tag(tag) {
            out.push('<');
            rest = &rest[start + 1..];
            continue;
        }
        match tag_name(tag).as_str() {
            "br" => out.push('\n'),
            "img" => out.push_str(image_alt(tag, &rest[start + end + 1..])),
            _ => {}
        }
        rest = &rest[start + end + 1..];
    }
    out.push_str(rest);
    decode(&out)
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn german_label(english: &str) -> Option<&'static str> {
    Some(match english {
        "Burst Damage" => "Salvenschaden",
        "Grab Range" => "Greifreichweite",
        "Cooldown" => "Abklingzeit",
        "Duration" => "Dauer",
        "Damage" => "Schaden",
        "Damage Per Second" => "Schaden pro Sek.",
        "Radius" => "Radius",
        "Cast Range" => "Wirkreichweite",
        "Cast Delay" => "Zauberverzögerung",
        "Charge Delay" => "Verzögerung zw. Aufladungen",
        "Channel Duration" => "Kanalisierungsdauer",
        "Move Speed" => "Bewegungstempo",
        "Fire Rate" => "Feuerrate",
        "Weapon Damage" => "Waffenschaden",
        "Bullet Damage" => "Kugelschaden",
        "Bullet Resist" => "Kugelresistenz",
        "Spirit Resist" => "Magieresistenz",
        "Max Stacks" => "Max. Stapel",
        "Bonus Health" => "Bonus-Lebenspunkte",
        "Health Regen" => "Lebensregeneration",
        "Out of Combat Regen" => "Regeneration außerhalb des Kampfes",
        "Sprint Speed" => "Sprinttempo",
        "Stamina" => "Ausdauer",
        "Ammo" => "Munition",
        "Max Ammo" => "Max. Munition",
        "Reload Time" => "Nachladezeit",
        "Bullet Velocity" => "Kugelgeschwindigkeit",
        "Bullet Lifesteal" => "Kugel-Lebensraub",
        "Spirit Lifesteal" => "Magie-Lebensraub",
        "Spirit Power" => "Magiekraft",
        "Spirit Damage" => "Magieschaden",
        "Ability Duration" => "Fähigkeitsdauer",
        "Ability Cooldown Reduction" => "Reduzierung der Fähigkeitsabklingzeit",
        "Bonus Ability Charges" => "Zusätzliche Fähigkeitsaufladungen",
        "Barrier" => "Barriere",
        "Slow Duration" => "Verlangsamungsdauer",
        "Debuff Resist" => "Schwächungsresistenz",
        "Incoming Healing" => "Erhaltene Heilung",
        "Outgoing Healing" => "Ausgehende Heilung",
        "Damage Reduction" => "Schadensreduzierung",
        "Heal Amount" => "Heilmenge",
        "Charges" => "Aufladungen",
        "Dash Distance" => "Sprintdistanz",
        "Bonus Damage" => "Bonusschaden",
        "Air Control" => "Luftkontrolle",
        "Buildup Decay Time" => "Abbauzeit des Aufbaus",
        "Stun Duration" => "Betäubungsdauer",
        "Explosion Damage" => "Explosionsschaden",
        "Weapon Damage per Kill" => "Waffenschaden pro Kill",
        "Air Jump/Dash Distance" => "Luftsprung-/Sprintdistanz",
        "Stamina Recovery" => "Ausdauerregeneration",
        "Bleed Damage" => "Blutungsschaden",
        "Bonus Spirit Power for Charged Abilities" => {
            "Bonus-Magiekraft für aufgeladene Fähigkeiten"
        }
        "Cone Angle" => "Kegelwinkel",
        "Damage on Jump" => "Schaden beim Sprung",
        "Faster Time Between Charges" => "Kürzere Zeit zwischen Aufladungen",
        "Initial Damage" => "Anfangsschaden",
        "Shock Damage" => "Schockschaden",
        "Damage per second flown" => "Schaden pro Flugsekunde",
        "Fire Rate Bonus" => "Feuerratenbonus",
        "Fire Rate While Sliding" => "Feuerrate beim Rutschen",
        "Fire Rate per Kill" => "Feuerrate pro Kill",
        "Float Speed" => "Schwebetempo",
        "Heal Interval" => "Heilintervall",
        "Damage Threshold" => "Schadensschwelle",
        "Imbued Ability Duration" => "Dauer der verstärkten Fähigkeit",
        "Impact Damage" => "Aufprallschaden",
        "In Air Damage" => "Schaden in der Luft",
        "Damage Penalty" => "Schadensmalus",
        "Weapon Damage vs. NPCs" => "Waffenschaden gegen NSC",
        "Interrupt Cooldown" => "Abklingzeit der Unterbrechung",
        "Invis Move Speed" => "Bewegungstempo unsichtbar",
        "Item Cooldown Reduction" => "Reduzierung der Item-Abklingzeit",
        "Weapon Fall-off Range" => "Reichweite des Schadensabfalls",
        "Damage Amp" => "Schadensverstärkung",
        "Max DPS" => "Max. Schaden pro Sek.",
        "Max Damage" => "Max. Schaden",
        "Max Frequency" => "Max. Frequenz",
        "Weapon Multishot" => "Waffen-Mehrfachschuss",
        "Max Weapon Damage" => "Max. Waffenschaden",
        "Current Health Bonus Damage" => "Bonusschaden nach aktuellen Lebenspunkten",
        "Movement Slow" => "Bewegungsverlangsamung",
        "Non Charged Ability" => "Nicht aufgeladene Fähigkeit",
        "Cooldown Reduction" => "Abklingzeitreduzierung",
        "Close Range" => "Nahbereich",
        "Cleave Damage" => "Flächenschaden",
        "Respawn Time" => "Wiederbelebungszeit",
        "Shield Health" => "Schildlebenspunkte",
        "Active Movespeed Penalty" => "Bewegungstempo-Malus bei Aktivierung",
        "Charged Ability" => "Aufgeladene Fähigkeit",
        "Slow Radius" => "Verlangsamungsradius",
        "Spirit Damage Per Bullet" => "Magieschaden pro Kugel",
        "Spirit Damage Reduction" => "Magieschadensreduzierung",
        "Bullet Slow Proc" => "Verlangsamung durch Kugeln",
        "Bullet Resist per Stack" => "Kugelresistenz pro Stapel",
        "Spirit Power Per Soul" => "Magiekraft pro Seele",
        "Spirit Resist on Headshot" => "Magieresistenz bei Kopfschuss",
        "Buildup Per Bullet" => "Aufbau pro Kugel",
        "Bonus Weapon Damage" => "Bonus-Waffenschaden",
        "Bonus Fly Speed" => "Bonus-Flugtempo",
        "Stored Spirit Damage" => "Gespeicherter Magieschaden",
        "Bonus Ammo Per Kill" => "Bonusmunition pro Kill",
        "Summon Health" => "Lebenspunkte der Beschwörung",
        "Summon Lifetime" => "Lebensdauer der Beschwörung",
        "Teleport Range" => "Teleportreichweite",
        "Temporary Ammo" => "Temporäre Munition",
        "Base Damage" => "Grundschaden",
        "Ambush Fire Rate" => "Feuerrate aus dem Hinterhalt",
        "Evasion While Sliding" => "Ausweichen beim Rutschen",
        "Explode Damage" => "Explosionsschaden",
        "Delayed Damage" => "Verzögerter Schaden",
        "Fade Time" => "Ausblendzeit",
        "Weapon Damage per Stack" => "Waffenschaden pro Stapel",
        "Head Shot Bonus Damage" => "Kopfschuss-Bonusschaden",
        "Headshot Bonus Damage" => "Kopfschuss-Bonusschaden",
        _ => return None,
    })
}

fn localized_label(fact: &Map<String, Value>) -> Option<&'static str> {
    let text = |field: &str| {
        fact.get(field)
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|label| !label.is_empty())
    };
    german_label(text("label_de").or_else(|| text("label"))?)
}

fn present_fact(fact: &mut Map<String, Value>) {
    let named = fact
        .get("stat_key")
        .and_then(Value::as_str)
        .is_some_and(|key| key.starts_with("name_") || key.starts_with("identity_"));
    if !named {
        if let Some(Value::String(text)) = fact.get_mut("value") {
            *text = plain(text);
        }
    }
    if let Some(label) = localized_label(fact) {
        fact.insert("label_de".into(), Value::String(label.into()));
    }
}

pub fn present(value: &mut Value) {
    match value {
        Value::Object(object) => {
            if object.contains_key("stat_key") {
                present_fact(object);
            }
            for (key, field) in object.iter_mut() {
                if key != "value" {
                    present(field);
                }
            }
        }
        Value::Array(values) => values.iter_mut().for_each(present),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn facts_lose_markup_and_gain_german_labels() {
        let mut card = json!({"facts":{"facts":[
            {"stat_key":"item_description_desc","value":"Dein <span class=\"highlight\">Kopfschuss</span> fügt <svg width=\"128\"><path d=\"M32.0027 9C40\"/></svg> <span class=\"inline-attribute-label BonusWeaponDamage\">Bonuswaffenschaden</span> zu.","condition":{"lang":"de"}},
            {"stat_key":"item_abilitycooldown","value":9,"label":"Cooldown","condition":{}},
            {"stat_key":"item_headshotbonusdamage","value":45,"label":"Head Shot Bonus Damage","condition":{}},
            {"stat_key":"name_de","value":"A &amp; B","condition":{}}
        ]},"identity":{"name_de":"A &amp; B"}});
        present(&mut card);
        let facts = &card["facts"]["facts"];
        assert_eq!(
            facts[0]["value"],
            "Dein Kopfschuss fügt Bonuswaffenschaden zu."
        );
        assert_eq!(facts[1]["label_de"], "Abklingzeit");
        assert_eq!(facts[1]["value"], 9);
        assert_eq!(facts[2]["label_de"], "Kopfschuss-Bonusschaden");
        assert_eq!(facts[3]["value"], "A &amp; B");
        assert_eq!(card["identity"]["name_de"], "A &amp; B");
    }
}
