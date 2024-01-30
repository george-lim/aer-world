use crate::{EntityId, EntityMap, Notification};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy)]
pub enum Reaction {
    OpportunityAttack { damage_amount: i64 },
    Reinforce { armor_amount: i64 },
    Spite { damage_amount: i64 },
}

pub struct ReactionSystem<NotificationHandler> {
    _notification_handler: NotificationHandler,
    pub reactions_map: EntityMap<Vec<Reaction>>,
}

impl<NotificationHandler> ReactionSystem<NotificationHandler>
where
    NotificationHandler: Fn(EntityId, Notification),
{
    pub fn new(notification_handler: NotificationHandler) -> Self {
        Self {
            _notification_handler: notification_handler,
            reactions_map: Default::default(),
        }
    }

    pub fn _reactions(&self, entity: &EntityId) -> Option<&Vec<Reaction>> {
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
