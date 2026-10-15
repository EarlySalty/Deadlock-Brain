use crate::Extractor;

pub const EXTRACTORS: &[&dyn Extractor] = &[
    &crate::hero::Hero,
    &crate::docs::DocsExtractor,
    &crate::timers::TimerExtractor,
    &crate::assets::Assets,
    &crate::assets::hero_slots::HeroSlots,
];
