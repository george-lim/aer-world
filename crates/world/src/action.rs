use crate::{log_with_indentation, systems::components::*, EntityId, Event, World};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub enum Action {
    Spawn {
        allegiance: Option<Allegiance>,
        armor: Option<Armor>,
        health: Option<Health>,
        position: Option<Position>,
        reactions: Vec<Reaction>,
    },
    Destroy,
    Move {
        to_position: Position,
    },
    Damage {
        amount: i64,
    },
    GainArmor {
        amount: i64,
    },
}

impl World {
    pub fn perform(
        &mut self,
        action: Action,
        source: EntityId,
        target: EntityId,
        stack_depth: u64,
    ) {
        log_with_indentation!(stack_depth, "[Action] {source:?} -> {target:?} {action:?}");

        match action {
            Action::Spawn {
                allegiance,
                armor,
                health,
                position,
                reactions,
            } => {
                let entity = self.next_entity;
                self.next_entity = EntityId(entity.0 + 1);

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
            }
            Action::Destroy => {
                self.emit(&Event::BeforeDestroy, source, target, stack_depth);

                self.allegiance_system.remove(&target);
                self.armor_system.remove(&target);
                self.health_system.remove(&target);
                self.position_system.remove(&target);
                self.reaction_system.remove(&target);
            }
            Action::Move { to_position } => {
                let Some(from_position) = self.position_system.position(&target).copied() else {
                    return;
                };

                self.position_system.move_to(target, to_position);

                self.emit(
                    &Event::AfterMove { from_position },
                    source,
                    target,
                    stack_depth,
                )
            }
            Action::Damage { amount } => {
                let overflow_damage = self.armor_system.lose(&target, amount).unwrap_or(amount);

                let Some(is_alive) = self.health_system.lose(&target, overflow_damage) else {
                    return;
                };

                if overflow_damage > 0 {
                    self.emit(&Event::AfterDamage, source, target, stack_depth)
                }

                if !is_alive {
                    self.perform(Action::Destroy, source, target, stack_depth)
                }
            }
            Action::GainArmor { amount } => self.armor_system.gain(&target, amount),
        }
    }
}
