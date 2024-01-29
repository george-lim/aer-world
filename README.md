# AER World

AER World is a prototype game state container designed to model games like [Hearthstone](https://hearthstone.blizzard.com/en-us) and [Super Auto Pets](https://teamwood.itch.io/super-auto-pets), where entities can react to actions by spawning more actions at any point in time.

## AER

AER (Action Event Reaction) is a design pattern used in AER World that describes a way to clearly track and manage nested state changes caused by actions. The best way to demonstrate this is with an example.

The following is an example of AER World simulating the game state as `Golem 2` attempts to move away from `Player 1`.

```

---- Player 1 ----
life: 10/10 + 10
position: (0, 0)
reactions: [OpportunityAttack { damage_amount: 3 }]


---- Golem 2 ----
life: 2/3 + 2
position: (0, 0)
reactions: [Reinforce { armor_amount: 3 }]

[Action] 2 -> 2 Move { to_position: (0, 1) }
[Event] 2 -> 2 AfterMove { from_position: (0, 0) }
[Reaction] 1 OpportunityAttack { damage_amount: 3 }
        [Action] 1 -> 2 Damage { amount: 3 }
        [Event] 1 -> 2 AfterDamage
        [Reaction] 2 Reinforce { armor_amount: 3 }
                [Action] 2 -> 2 GainArmor { amount: 3 }

---- Player 1 ----
life: 10/10 + 10
position: (0, 0)
reactions: [OpportunityAttack { damage_amount: 3 }]


---- Golem 2 ----
life: 1/3 + 3
position: (0, 1)
reactions: [Reinforce { armor_amount: 3 }]

```

1. When `Golem 2` moves away from `Player 1` as its enemy, `Player 1`'s "Opportunity Attack" reaction triggers. This interrupts the move action, and spawns a new action to deal 3 damage to the Golem.
2. When `Golem 2` takes 3 damage, its own "Reinforce" reaction triggers. This interrupts the damage action, and spawns a new action to gain 3 armor for the Golem.
3. Since there are no more reactions to any of the actions on the stack, the actions are popped. The golem is left with 1 HP and 3 Armor as a result of moving away from the player.

The AER pattern does not permit direct mutation of systems. Instead, every possible state change must come from an **action**. An action must be performed by a source entity onto a target entity (the target can also be the source).

Entity queries can be used to provide a "fuzzy search" for action targets, like AOE attacks that attack a map region as opposed to a single target.

An action may mutate state multiple times, across multiple systems. For instance, dealing damage might first interact with the armor system to absorb some damage before interacting with the health system.

After each system state mutation, an action may choose to emit an event based on the result having met some criteria.

**Events** should only be created and used if there is a reaction that depends on it. For example, it does not make sense to create an `AfterDestroy` event if there are no reactions in the world that trigger after an entity is destroyed. However, it does make sense to add an event for `AfterMove` because Opportunity Attacks may occur if an entity moves away from an enemy.

**Reactions** are pre-defined event handlers, which conditionally perform more actions. They happen after an event is fired, which can happen at any point while an existing action is occuring. When a reaction occurs, it pauses the existing action and executes immediately. For example, the `Reinforce` reaction gains the reactor 3 armor whenever they are damaged.

AER does not support custom runtime reactions for entities. Every reaction must already exist in the world, although you may add or remove reactions from entities during runtime as part of an action.

## ECS

Game world objects are represented using the [ECS](https://en.wikipedia.org/wiki/Entity_component_system) pattern. This is done for three reasons:

1. **State change clarity** - when actions are performed, it becomes clear which systems are involved with the action, while not caring about what the entity actually is.
2. **Flexible component storage** - because an entity's data is scattered across components, we are free to store the components in any way that we see fit. We could use a bidirectional map in a system for example, to efficiently query all entities in a specific position, as well as query the position of any entity. This also means that we don't have to store many variations of the same entity just to improve performance.
3. **Powerful entity querying** - with ECS, we can making entity querying simple yet exhaustive by doing a simple set intersection for entities that have a specific component, or meet some criteria relating to the component.

To keep things simple, AER World systems currently use identity hash maps to store mappings between entities and components.

AER World uses [hierarchical sparse bitsets](https://github.com/tower120/hi_sparse_bitset) to perform entity queries, which provide extremely fast set intersection performance across any number of sets, at any size.

## Development Flow

AER World has two crates. `world` is the library for the game state container and `playground` is the simulation binary that allows you to test the public interface of `world`.

Just execute `cargo run` to start the `playground` simulation.

`world` will only log actions, events, and reactions to `stdout` when `debug_assertions` is enabled. The `World::describe(entity: &EntityId)` method can be used to log every component of an entity.
