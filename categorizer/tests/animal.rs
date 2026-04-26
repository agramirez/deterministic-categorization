mod framework;

use cucumber::{given, when, then, World};
use framework::*;

// Steps are defined with `given`, `when` and `then` attributes.
#[given(expr="a {state} cat")]
fn given_some_cat(world: &mut AnimalWorld, state:Feeling) {
    world.cat.feeling = state;
}
#[given(expr="a can of {food}")]
fn given_some_food(world: &mut AnimalWorld, food:Food) {
    world.food = food;
}

#[when(expr="I {action} the cat")]
fn when_some_action(world:&mut AnimalWorld, act:Action) {
    match act {
        Action::Feed => world.cat.feed(&world.food),
        Action::Pet => world.cat.pet(),
        Action::Call => world.cat.call(),
    }
}

#[then(expr="the cat is {state}")]
fn then_result(world:&mut AnimalWorld, feeling:Feeling) {
    assert_eq!(world.cat.feeling,feeling)
}

// This runs before everything else, so you can set up things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(AnimalWorld::run(
        "/workspaces/deterministic-normalization/categorizer/features/animal.feature",
    ));
}

