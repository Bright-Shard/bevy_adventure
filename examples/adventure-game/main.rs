use adventure_rs::{Game, components::*, events::*, level::*};
use hecs::World;

struct Damage(i32);

fn entry_level <'a> () -> Level {
    Level::new()
        .add_room(
            Room::new()
                .on_enter(|_, _| println!("Room 1 entered"))
        )
        .add_room(
            Room::new()
                .on_enter(|_, _| println!("Room 2 entered"))
        )
}

fn spawn_entities(world: &mut World, amount: usize, starting_health: i32) {
    let entities = (0..amount)
        .map(|_| {
            let health = Health(starting_health);
            let name = Name("eee".into());
            let on_death = OnDeath(||println!("Entity died"));
            
            (health, name, on_death)
        });
    world.spawn_batch(entities);
}

fn spawn_boss(world: &mut World) {
    let name = Name("BossMan".into());
    let health = Health(50);
    let on_death = OnDeath(||{
        println!("WTF, the boss has died!!!!");
    });

    world.spawn((name, health, on_death));
}

fn main() {
    let mut game = Game::new();

    game.world.spawn((Damage(0), 0));
    spawn_entities(&mut game.world, 10, 20);
    spawn_boss(&mut game.world);

    game.start();
}
