use crate::{EntityId, EntityMap};

pub struct Armor {
    pub current: i64,
}

#[cfg(debug_assertions)]
impl std::fmt::Debug for Armor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.current.fmt(f)
    }
}

#[derive(Default)]
pub struct ArmorSystem {
    armor_map: EntityMap<Armor>,
}

impl ArmorSystem {
    pub fn armor(&self, entity: &EntityId) -> Option<&Armor> {
        self.armor_map.get(entity)
    }

    pub fn gain(&mut self, entity: &EntityId, amount: i64) {
        let Some(armor) = self.armor_map.get_mut(entity) else {
            return;
        };

        armor.current += amount;
    }

    // Returns amount of overflow damage.
    pub fn lose(&mut self, entity: &EntityId, amount: i64) -> Option<i64> {
        let Some(armor) = self.armor_map.get_mut(entity) else {
            return None;
        };

        armor.current -= amount;

        match armor.current < 0 {
            true => {
                let overflow_damage = -armor.current;
                armor.current = 0;
                Some(overflow_damage)
            }
            false => Some(0),
        }
    }

    pub fn insert(&mut self, entity: EntityId, armor: Armor) {
        self.armor_map.insert(entity, armor);
    }

    pub fn remove(&mut self, entity: &EntityId) {
        self.armor_map.remove(entity);
    }
}
