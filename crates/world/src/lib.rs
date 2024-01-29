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

pub const WORLD_ENTITY: EntityId = EntityId(0);

pub struct World {
    next_entity: EntityId,

    allegiance_system: AllegianceSystem,
    armor_system: ArmorSystem,
    health_system: HealthSystem,
    position_system: PositionSystem,
    reaction_system: ReactionSystem,
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

        self.perform(
            Action::Spawn {
                allegiance,
                armor,
                health,
                position,
                reactions,
            },
            WORLD_ENTITY,
            WORLD_ENTITY,
            0,
        );

        entity
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

impl Default for World {
    fn default() -> Self {
        Self {
            next_entity: EntityId(1),
            allegiance_system: Default::default(),
            armor_system: Default::default(),
            health_system: Default::default(),
            position_system: Default::default(),
            reaction_system: Default::default(),
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
