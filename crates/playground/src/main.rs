use world::*;

fn main() {
    let mut world = World::new();

    let player = world.spawn(
        Some(Allegiance::Player),
        Some(Armor { current: 5 }),
        Some(Health {
            current: 10,
            max: 10,
        }),
        Some(Position { x: 0, y: 0 }),
        vec![Reaction::OpportunityAttack { damage_amount: 10 }],
    );

    let golem = world.spawn(
        Some(Allegiance::Golem),
        Some(Armor { current: 0 }),
        Some(Health { current: 3, max: 3 }),
        Some(Position { x: 0, y: 5 }),
        vec![Reaction::Reinforce { armor_amount: 3 }],
    );

    world.describe(&player);
    world.describe(&golem);

    world.perform(Action::DealDamage { amount: 1 }, player, golem, 0);

    let query = EntityQuery {
        allegiance_filter: ComponentFilter::Include(&[Allegiance::Golem]),
        position_filter: ComponentFilter::Include(&[Position { x: 0, y: 5 }]),
    };

    world.perform_with_query(Action::DealDamage { amount: 1 }, player, query, 0);
    world.perform(Action::GainArmor { amount: 5 }, player, player, 0);

    world.perform(
        Action::Move {
            to_position: Position { x: 0, y: 0 },
        },
        golem,
        golem,
        0,
    );

    world.perform(
        Action::Move {
            to_position: Position { x: 0, y: 1 },
        },
        golem,
        golem,
        0,
    );

    world.describe(&player);
    world.describe(&golem);
}
