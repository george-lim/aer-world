use crate::{EntityId, EntityMap, EntitySet};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Allegiance {
    Player,
    Golem,
}

#[derive(Default)]
pub struct AllegianceSystem {
    pub entities: EntitySet,
    allegiance_map: EntityMap<Allegiance>,
}

impl AllegianceSystem {
    pub fn allegiance(&self, entity: &EntityId) -> Option<&Allegiance> {
        self.allegiance_map.get(entity)
    }

    pub fn entities(&self, allegiances: &[Allegiance]) -> EntitySet {
        if allegiances.is_empty() {
            return Default::default();
        }

        let mut entities = EntitySet::default();

        for (entity, allegiance) in &self.allegiance_map {
            if allegiances.contains(allegiance) {
                entities.insert(entity);
            }
        }

        entities
    }

    pub fn insert(&mut self, entity: EntityId, allegiance: Allegiance) {
        self.entities.insert(&entity);
        self.allegiance_map.insert(entity, allegiance);
    }

    pub fn remove(&mut self, entity: &EntityId) {
        self.entities.remove(entity);
        self.allegiance_map.remove(entity);
    }
}
