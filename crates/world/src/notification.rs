use crate::{Allegiance, Armor, Health, Position, Reaction};

pub enum Notification<'a> {
    Spawn {
        allegiance: Option<&'a Allegiance>,
        armor: Option<&'a Armor>,
        health: Option<&'a Health>,
        position: Option<&'a Position>,
        reactions: &'a Vec<Reaction>,
    },
    Destroy,
    ChangeAllegiance(Option<&'a Allegiance>),
    ChangeArmor(Option<&'a Armor>),
    ChangeHealth(Option<&'a Health>),
    ChangePosition(Option<&'a Position>),
    ChangeReactions(&'a Vec<Reaction>),
}
