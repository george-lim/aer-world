use crate::{log_with_indentation, systems::components::*, Action, EntityId, Notification, World};

#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Event {
    AfterMove { from_position: Position },
    AfterDamage,
    BeforeDestroy,
}

impl<NotificationHandler> World<NotificationHandler>
where
    NotificationHandler: Fn(EntityId, Notification),
{
    fn handle_event(
        &mut self,
        event: &Event,
        source: EntityId,
        target: EntityId,
        reactor: EntityId,
        reaction: &Reaction,
        stack_depth: u64,
    ) {
        match (event, reaction) {
            (Event::AfterMove { from_position }, Reaction::OpportunityAttack { damage_amount }) => {
                let Some(reactor_allegiance) = self.allegiance_system.allegiance(&reactor) else {
                    return;
                };

                let Some(target_allegiance) = self.allegiance_system.allegiance(&target) else {
                    return;
                };

                if !(target_allegiance != reactor_allegiance) {
                    return;
                }

                let Some(reactor_position) = self.position_system.position(&reactor) else {
                    return;
                };

                if !(from_position == reactor_position) {
                    return;
                }

                log_with_indentation!(stack_depth, "[Reaction] {reactor:?} {reaction:?}");

                self.perform(
                    Action::Damage {
                        amount: *damage_amount,
                    },
                    reactor,
                    target,
                    stack_depth + 1,
                )
            }
            (Event::AfterDamage, Reaction::Reinforce { armor_amount }) => {
                if !(target == reactor) {
                    return;
                }

                log_with_indentation!(stack_depth, "[Reaction] {reactor:?} {reaction:?}");

                self.perform(
                    Action::GainArmor {
                        amount: *armor_amount,
                    },
                    reactor,
                    reactor,
                    stack_depth + 1,
                )
            }
            (Event::BeforeDestroy, Reaction::Spite { damage_amount }) => {
                if !(target == reactor) {
                    return;
                }

                log_with_indentation!(stack_depth, "[Reaction] {reactor:?} {reaction:?}");

                self.perform(
                    Action::Damage {
                        amount: *damage_amount,
                    },
                    reactor,
                    source,
                    stack_depth + 1,
                )
            }
            _ => (),
        }
    }

    pub fn emit(&mut self, event: &Event, source: EntityId, target: EntityId, stack_depth: u64) {
        log_with_indentation!(stack_depth, "[Event] {source:?} -> {target:?} {event:?}");

        let reactions_map = self.reaction_system.reactions_map.clone();

        for (reactor, reactions) in reactions_map.into_iter() {
            for reaction in reactions {
                self.handle_event(event, source, target, reactor, &reaction, stack_depth)
            }
        }
    }
}
