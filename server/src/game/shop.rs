use super::{SHOP_SIZE, SHOP_SPELL_SIZE};
use protocol::{
    characters::get_characters, protocol::CharacterInstance, protocol_types::spell::Spell,
    spells::get_spells,
};
use rand::seq::SliceRandom;

#[derive(Debug, Default, Clone)]
pub struct Shop {
    pub characters: Vec<Option<CharacterInstance>>,
    pub spells: Vec<Option<Spell>>,
    pub locked: bool,
}

impl Shop {
    pub fn new(lvl: u8) -> Self {
        Self {
            characters: Self::get_new_characters(SHOP_SIZE, lvl),
            spells: Self::get_new_spells(SHOP_SPELL_SIZE, lvl),
            locked: false,
        }
    }

    pub fn fill(&mut self, lvl: u8) {
        // Remove all None values
        self.characters.retain(|c| c.is_some());
        self.spells.retain(|s| s.is_some());

        // Fill the rest of the shop
        self.characters.append(&mut Self::get_new_characters(
            SHOP_SIZE - self.characters.len(),
            lvl,
        ));
        self.spells.append(&mut Self::get_new_spells(
            SHOP_SPELL_SIZE - self.spells.len(),
            lvl,
        ));
        self.locked = false;
    }

    pub fn get_new_characters(count: usize, lvl: u8) -> Vec<Option<CharacterInstance>> {
        get_characters()
            .iter()
            .filter(|c| c.cost <= lvl)
            .collect::<Vec<_>>()
            .choose_multiple(&mut rand::thread_rng(), count)
            .cloned()
            .map(|c| Some(CharacterInstance::from(c, false)))
            .collect::<Vec<_>>()
    }

    pub fn get_new_spells(count: usize, lvl: u8) -> Vec<Option<Spell>> {
        let mut spells = get_spells()
            .iter()
            .filter(|s| s.lvl <= lvl)
            .cloned()
            .collect::<Vec<_>>();

        spells.shuffle(&mut rand::thread_rng());

        spells
            .choose_multiple(&mut rand::thread_rng(), count)
            .cloned()
            .map(Some)
            .collect::<Vec<_>>()
    }
}
