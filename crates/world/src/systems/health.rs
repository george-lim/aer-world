use crate::{EntityId, EntityMap, Notification};

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

pub struct HealthSystem<NotificationHandler> {
    notification_handler: NotificationHandler,
    health_map: EntityMap<Health>,
}

impl<NotificationHandler> HealthSystem<NotificationHandler>
where
    NotificationHandler: Fn(EntityId, Notification),
{
    pub fn new(notification_handler: NotificationHandler) -> Self {
        Self {
            notification_handler,
            health_map: Default::default(),
        }
    }

    pub fn _health(&self, entity: &EntityId) -> Option<&Health> {
        self.health_map.get(entity)
    }

    // Returns whether the entity is alive.
    pub fn lose(&mut self, entity: EntityId, amount: i64) -> Option<bool> {
        let Some(health) = self.health_map.get_mut(&entity) else {
            return None;
        };

        health.current -= amount;

        (self.notification_handler)(entity, Notification::ChangeHealth(Some(health)));
        Some(health.current > 0)
    }

    pub fn insert(&mut self, entity: EntityId, health: Health) {
        self.health_map.insert(entity, health);
    }

    pub fn remove(&mut self, entity: &EntityId) {
        self.health_map.remove(entity);
    }
}
