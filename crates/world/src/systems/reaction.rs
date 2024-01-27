use crate::{EntityId, EntityMap};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy)]
pub enum Reaction {
    OpportunityAttack { damage_amount: i64 },
    Reinforce { armor_amount: i64 },
}

#[derive(Default)]
pub struct ReactionSystem {
    pub reactions_map: EntityMap<Vec<Reaction>>,
}

impl ReactionSystem {
    pub fn reactions(&self, entity: &EntityId) -> Option<&Vec<Reaction>> {
        self.reactions_map.get(entity)
    }

    pub fn insert(&mut self, entity: EntityId, reactions: Vec<Reaction>) {
        if reactions.is_empty() {
            return;
        }

        self.reactions_map.insert(entity, reactions);
    }

    pub fn remove(&mut self, entity: &EntityId) {
        self.reactions_map.remove(entity);
    }
}
