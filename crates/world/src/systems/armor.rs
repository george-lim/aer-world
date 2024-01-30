use crate::{EntityId, EntityMap, Notification};

#[derive(Clone, Copy)]
pub struct Armor {
    pub current: i64,
}

#[cfg(debug_assertions)]
impl std::fmt::Debug for Armor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.current.fmt(f)
    }
}

pub struct ArmorSystem<NotificationHandler> {
    notification_handler: NotificationHandler,
    armor_map: EntityMap<Armor>,
}

impl<NotificationHandler> ArmorSystem<NotificationHandler>
where
    NotificationHandler: Fn(EntityId, Notification),
{
    pub fn new(notification_handler: NotificationHandler) -> Self {
        Self {
            notification_handler,
            armor_map: Default::default(),
        }
    }

    pub fn _armor(&self, entity: &EntityId) -> Option<&Armor> {
        self.armor_map.get(entity)
    }

    pub fn gain(&mut self, entity: EntityId, amount: i64) {
        let Some(armor) = self.armor_map.get_mut(&entity) else {
            return;
        };

        armor.current += amount;
        (self.notification_handler)(entity, Notification::ChangeArmor(Some(armor)));
    }

    // Returns amount of overflow damage.
    pub fn lose(&mut self, entity: EntityId, amount: i64) -> Option<i64> {
        let Some(armor) = self.armor_map.get_mut(&entity) else {
            return None;
        };

        armor.current -= amount;

        let overflow_damage = match armor.current < 0 {
            true => {
                let overflow_damage = -armor.current;
                armor.current = 0;
                Some(overflow_damage)
            }
            false => Some(0),
        };

        (self.notification_handler)(entity, Notification::ChangeArmor(Some(armor)));
        overflow_damage
    }

    pub fn insert(&mut self, entity: EntityId, armor: Armor) {
        self.armor_map.insert(entity, armor);
    }

    pub fn remove(&mut self, entity: &EntityId) {
        self.armor_map.remove(entity);
    }
}
