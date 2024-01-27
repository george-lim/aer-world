mod action;
mod entity_query;
mod event;
mod systems;
mod utils;

use event::*;
use systems::*;
use utils::*;

pub use action::Action;
pub use entity_query::*;
pub use systems::components::*;

#[derive(Default)]
pub struct World {
    allegiance_system: AllegianceSystem,
    armor_system: ArmorSystem,
    health_system: HealthSystem,
    position_system: PositionSystem,
    reaction_system: ReactionSystem,

    next_entity: EntityId,
}

impl World {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn spawn(
        &mut self,
        allegiance: Option<Allegiance>,
        armor: Option<Armor>,
        health: Option<Health>,
        position: Option<Position>,
        reactions: Vec<Reaction>,
    ) -> EntityId {
        let entity = self.next_entity;

        if let Some(allegiance) = allegiance {
            self.allegiance_system.insert(entity, allegiance);
        }

        if let Some(armor) = armor {
            self.armor_system.insert(entity, armor);
        }

        if let Some(health) = health {
            self.health_system.insert(entity, health);
        }

        if let Some(position) = position {
            self.position_system.insert(entity, position);
        }

        self.reaction_system.insert(entity, reactions);

        self.next_entity = EntityId(self.next_entity.0 + 1);
        entity
    }

    pub fn destroy(&mut self, entity: &EntityId) {
        self.allegiance_system.remove(entity);
        self.armor_system.remove(entity);
        self.health_system.remove(entity);
        self.position_system.remove(entity);
        self.reaction_system.remove(entity);
    }

    #[allow(unused_variables)]
    pub fn describe(&self, entity: &EntityId) {
        #[cfg(debug_assertions)]
        {
            println!();

            match self.allegiance_system.allegiance(entity) {
                Some(allegiance) => println!("---- {allegiance:?} {entity:?} ----"),
                None => println!("---- {entity:?} ----"),
            };

            if let Some(health) = self.health_system.health(entity) {
                print!("life: {health:?}");

                match self.armor_system.armor(entity) {
                    Some(armor) if armor.current > 0 => {
                        println!(" + {armor:?}")
                    }
                    _ => println!(),
                }
            }

            if let Some(position) = self.position_system.position(entity) {
                println!("position: {position:?}");
            }

            if let Some(reactions) = self.reaction_system.reactions(entity) {
                println!("reactions: {reactions:?}");
            }

            println!();
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EntityId(usize);

#[cfg(debug_assertions)]
impl std::fmt::Debug for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
