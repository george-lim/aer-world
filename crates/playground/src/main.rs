use std::{cell::RefCell, collections::HashMap, rc::Rc};

use world::*;

fn main() {
    let entity_map: Rc<RefCell<HashMap<EntityId, Entity>>> = Default::default();

    let mut world = World::new(|entity, notification| {
        let mut entity_map = entity_map.borrow_mut();

        match notification {
            Notification::Spawn {
                allegiance,
                armor,
                health,
                position,
                reactions,
            } => {
                entity_map.insert(
                    entity,
                    Entity {
                        id: entity,
                        allegiance: allegiance.copied(),
                        armor: armor.copied(),
                        health: health.copied(),
                        position: position.copied(),
                        reactions: reactions.clone(),
                    },
                );
            }
            Notification::Destroy => {
                entity_map.remove(&entity);
            }
            Notification::ChangeAllegiance(allegiance) => {
                entity_map.get_mut(&entity).unwrap().allegiance = allegiance.copied();
            }
            Notification::ChangeArmor(armor) => {
                entity_map.get_mut(&entity).unwrap().armor = armor.copied();
            }
            Notification::ChangeHealth(health) => {
                entity_map.get_mut(&entity).unwrap().health = health.copied();
            }
            Notification::ChangePosition(position) => {
                entity_map.get_mut(&entity).unwrap().position = position.copied();
            }
            Notification::ChangeReactions(reactions) => {
                entity_map.get_mut(&entity).unwrap().reactions = reactions.clone();
            }
        }
    });

    world.perform(
        Action::Spawn {
            allegiance: Some(Allegiance::Player),
            armor: Some(Armor { current: 5 }),
            health: Some(Health {
                current: 10,
                max: 10,
            }),
            position: Some(Position { x: 0, y: 0 }),
            reactions: vec![Reaction::OpportunityAttack { damage_amount: 10 }],
        },
        WORLD_ENTITY,
        WORLD_ENTITY,
        0,
    );

    world.perform(
        Action::Spawn {
            allegiance: Some(Allegiance::Golem),
            armor: Some(Armor { current: 0 }),
            health: Some(Health { current: 3, max: 3 }),
            position: Some(Position { x: 0, y: 5 }),
            reactions: vec![
                Reaction::Reinforce { armor_amount: 3 },
                Reaction::Spite { damage_amount: 50 },
            ],
        },
        WORLD_ENTITY,
        WORLD_ENTITY,
        0,
    );

    #[cfg(debug_assertions)]
    for entity in entity_map.borrow().values() {
        println!("{entity:?}");
    }

    let player = EntityId(1);
    let golem = EntityId(2);

    world.perform(Action::Damage { amount: 1 }, player, golem, 0);

    let query = EntityQuery {
        allegiance_filter: ComponentFilter::Include(&[Allegiance::Golem]),
        position_filter: ComponentFilter::Include(&[Position { x: 0, y: 5 }]),
    };

    world.perform_with_query(Action::Damage { amount: 1 }, player, query, 0);
    world.perform(Action::GainArmor { amount: 5 }, player, player, 0);

    world.perform(
        Action::Move {
            to_position: Position { x: 0, y: 0 },
        },
        golem,
        golem,
        0,
    );

    world.perform(
        Action::Move {
            to_position: Position { x: 0, y: 1 },
        },
        golem,
        golem,
        0,
    );

    #[cfg(debug_assertions)]
    for entity in entity_map.borrow().values() {
        println!("{entity:?}");
    }
}

#[allow(dead_code)]
struct Entity {
    id: EntityId,
    allegiance: Option<Allegiance>,
    armor: Option<Armor>,
    health: Option<Health>,
    position: Option<Position>,
    reactions: Vec<Reaction>,
}

#[cfg(debug_assertions)]
impl std::fmt::Debug for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\n")?;

        let entity = self.id;

        match self.allegiance {
            Some(allegiance) => f.write_fmt(format_args!("---- {allegiance:?} {entity:?} ----\n")),
            None => f.write_fmt(format_args!("---- {entity:?} ----\n")),
        }?;

        if let Some(health) = self.health {
            f.write_fmt(format_args!("life: {health:?}"))?;

            match self.armor {
                Some(armor) if armor.current > 0 => f.write_fmt(format_args!(" + {armor:?}\n")),
                _ => f.write_str("\n"),
            }?;
        }

        if let Some(position) = self.position {
            f.write_fmt(format_args!("position: {position:?}\n"))?;
        }

        if !self.reactions.is_empty() {
            f.write_fmt(format_args!(
                "reactions: {reactions:?}\n",
                reactions = self.reactions
            ))?;
        }

        Ok(())
    }
}
