use crate::{systems::components::*, utils::*, Action, EntityId, Notification, World};

pub enum ComponentFilter<'qry, Component> {
    Ignore,
    Include(&'qry [Component]),
    Any,
}

pub struct EntityQuery<'qry> {
    pub allegiance_filter: ComponentFilter<'qry, Allegiance>,
    pub position_filter: ComponentFilter<'qry, Position>,
}

impl<NotificationHandler> World<NotificationHandler>
where
    NotificationHandler: Fn(EntityId, Notification),
{
    fn entities(&self, query: EntityQuery) -> EntitySet {
        let allegiance_entities = match query.allegiance_filter {
            ComponentFilter::Include(allegiances) => {
                Some(self.allegiance_system.entities(allegiances))
            }
            _ => None,
        };

        let position_entities = match query.position_filter {
            ComponentFilter::Include(positions) => Some(self.position_system.entities(positions)),
            _ => None,
        };

        EntitySet::intersection(&[
            match query.allegiance_filter {
                ComponentFilter::Ignore => None,
                ComponentFilter::Include(_) => allegiance_entities.as_ref(),
                ComponentFilter::Any => Some(&self.allegiance_system.entities),
            },
            match query.position_filter {
                ComponentFilter::Ignore => None,
                ComponentFilter::Include(_) => position_entities.as_ref(),
                ComponentFilter::Any => Some(&self.position_system.entities),
            },
        ])
    }

    pub fn perform_with_query(
        &mut self,
        action: Action,
        source: EntityId,
        query: EntityQuery,
        stack_depth: u64,
    ) {
        for target in self.entities(query).iter() {
            self.perform(action.clone(), source, target, stack_depth)
        }
    }
}
