use crate::Extractor;

pub const EXTRACTORS: &[&dyn Extractor] = &[
    &crate::hero::Hero,
    &crate::assets::Assets,
    &crate::assets::hero_slots::HeroSlots,
];
