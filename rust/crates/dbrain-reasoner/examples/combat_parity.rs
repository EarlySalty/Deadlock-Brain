use dbrain_reasoner::{
    combat::evaluate_inventory, enrich_frozen_models, HeroModel, ItemModel, ReasonerConfig,
};
use serde::Deserialize;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    time::Instant,
};

#[derive(Deserialize)]
struct Frozen {
    config: ReasonerConfig,
    heroes: Vec<Row>,
    raw_snapshots: Vec<serde_json::Value>,
}
#[derive(Deserialize)]
struct Row {
    hero: HeroModel,
    items: Vec<ItemModel>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 3 {
        return Err("Aufruf: combat_parity FREEZE.json AUSGABE.json".into());
    }
    let input: Frozen = serde_json::from_reader(BufReader::new(File::open(&args[1])?))?;
    let mut output = Vec::new();
    let mut elapsed = std::time::Duration::ZERO;
    for mut row in input.heroes {
        enrich_frozen_models(&mut row.hero, &mut row.items, &input.raw_snapshots)?;
        row.items.sort_by_key(|item| item.item_id);
        for offset in [0, 12, 24] {
            let held: Vec<_> = row.items.iter().skip(offset).take(12).cloned().collect();
            for window in [8.0, 40.0] {
                let cfg = ReasonerConfig {
                    combat_window_seconds: window,
                    ..input.config.clone()
                };
                let start = Instant::now();
                let evaluation = evaluate_inventory(&row.hero, &held, &cfg);
                elapsed += start.elapsed();
                output.push(serde_json::json!({"hero_id":row.hero.hero_id,"window":window,"held_ids":held.iter().map(|item|item.item_id).collect::<Vec<_>>(),"evaluation":evaluation}));
            }
        }
    }
    let mut writer = BufWriter::new(File::create(&args[2])?);
    serde_json::to_writer(&mut writer, &output)?;
    writer.flush()?;
    eprintln!(
        "{} vollständige Auswertungen, reine Auswertungszeit {:.3} s",
        output.len(),
        elapsed.as_secs_f64()
    );
    Ok(())
}
