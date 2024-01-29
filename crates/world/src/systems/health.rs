use crate::{EntityId, EntityMap};

#[derive(Clone, Copy)]
pub struct Health {
    pub current: i64,
    pub max: i64,
}

#[cfg(debug_assertions)]
impl std::fmt::Debug for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}/{}", self.current, self.max))
    }
}

#[derive(Default)]
pub struct HealthSystem {
    health_map: EntityMap<Health>,
}

impl HealthSystem {
    pub fn health(&self, entity: &EntityId) -> Option<&Health> {
        self.health_map.get(entity)
    }

    // Returns whether the entity is alive.
    pub fn lose(&mut self, entity: &EntityId, amount: i64) -> Option<bool> {
        let Some(armor) = self.health_map.get_mut(entity) else {
            return None;
        };

        armor.current -= amount;
        Some(armor.current > 0)
    }

    pub fn insert(&mut self, entity: EntityId, health: Health) {
        self.health_map.insert(entity, health);
    }

    pub fn remove(&mut self, entity: &EntityId) {
        self.health_map.remove(entity);
    }
}
