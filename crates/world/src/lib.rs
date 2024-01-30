mod action;
mod entity_query;
mod event;
mod notification;
mod systems;
mod utils;

use event::*;
use systems::*;
use utils::*;

pub use action::Action;
pub use entity_query::*;
pub use notification::*;
pub use systems::components::*;

pub const WORLD_ENTITY: EntityId = EntityId(0);

pub struct World<NotificationHandler> {
    notification_handler: NotificationHandler,
    next_entity: EntityId,

    allegiance_system: AllegianceSystem<NotificationHandler>,
    armor_system: ArmorSystem<NotificationHandler>,
    health_system: HealthSystem<NotificationHandler>,
    position_system: PositionSystem<NotificationHandler>,
    reaction_system: ReactionSystem<NotificationHandler>,
}

impl<NotificationHandler> World<NotificationHandler>
where
    NotificationHandler: Fn(EntityId, Notification) + Clone,
{
    pub fn new(notification_handler: NotificationHandler) -> Self {
        Self {
            notification_handler: notification_handler.clone(),
            next_entity: EntityId(1),
            allegiance_system: AllegianceSystem::new(notification_handler.clone()),
            armor_system: ArmorSystem::new(notification_handler.clone()),
            health_system: HealthSystem::new(notification_handler.clone()),
            position_system: PositionSystem::new(notification_handler.clone()),
            reaction_system: ReactionSystem::new(notification_handler),
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EntityId(pub usize);

#[cfg(debug_assertions)]
impl std::fmt::Debug for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
