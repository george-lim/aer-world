use crate::{log_with_indentation, systems::components::*, EntityId, Event, World};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy)]
pub enum Action {
    Move { to_position: Position },
    DealDamage { amount: i64 },
    GainArmor { amount: i64 },
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
            Action::Move { to_position } => {
                let Some(from_position) = self.position_system.position(&target).copied() else {
                    return;
                };

                self.position_system.move_to(target, to_position);

                self.emit(Event::Moved { from_position }, source, target, stack_depth)
            }
            Action::DealDamage { amount } => {
                let overflow_damage = self.armor_system.lose(&target, amount).unwrap_or(amount);

                let Some(is_alive) = self.health_system.lose(&target, overflow_damage) else {
                    return;
                };

                if overflow_damage > 0 {
                    self.emit(Event::Damaged, source, target, stack_depth)
                }

                if !is_alive {
                    self.destroy(&target)
                }
            }
            Action::GainArmor { amount } => self.armor_system.gain(&target, amount),
        }
    }
}
