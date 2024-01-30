use crate::{EntityId, EntityMap, EntitySet, Notification};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

#[cfg(debug_assertions)]
impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.x).field(&self.y).finish()
    }
}

pub struct PositionSystem<NotificationHandler> {
    notification_handler: NotificationHandler,
    pub entities: EntitySet,
    position_map: EntityMap<Position>,
}

impl<NotificationHandler> PositionSystem<NotificationHandler>
where
    NotificationHandler: Fn(EntityId, Notification),
{
    pub fn new(notification_handler: NotificationHandler) -> Self {
        Self {
            notification_handler,
            entities: Default::default(),
            position_map: Default::default(),
        }
    }

    pub fn position(&self, entity: &EntityId) -> Option<&Position> {
        self.position_map.get(entity)
    }

    pub fn entities(&self, positions: &[Position]) -> EntitySet {
        if positions.is_empty() {
            return Default::default();
        }

        let mut entities = EntitySet::default();

        for (entity, position) in &self.position_map {
            if positions.contains(position) {
                entities.insert(entity);
            }
        }

        entities
    }

    pub fn move_to(&mut self, entity: EntityId, position: Position) {
        let Some(current_position) = self.position_map.get_mut(&entity) else {
            return;
        };

        *current_position = position;

        (self.notification_handler)(entity, Notification::ChangePosition(Some(current_position)));
    }

    pub fn insert(&mut self, entity: EntityId, position: Position) {
        self.entities.insert(&entity);
        self.position_map.insert(entity, position);
    }

    pub fn remove(&mut self, entity: &EntityId) {
        self.entities.remove(entity);
        self.position_map.remove(entity);
    }
}
