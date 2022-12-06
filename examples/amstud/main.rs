use bevy::prelude::*;
use bevy_adventure::prelude::*;

mod finale;
mod invasion;
mod remove_eight;
mod remove_eighteen;
mod remove_five;
mod remove_one;
mod remove_two_three;

fn opening_text(mut iomgr: ResMut<IOManager>, mut commands: Commands) {
    // Clear screen, hide cursor
    iomgr.hide_cursor();
    iomgr.clear();

    // DEVELOPMENT STUFF, DELETE WHEN DONE
    /*
    let neopoints = vec!["kids"];
    checkpoints.0 = neopoints;
    */

    // Disclaimer about game
    iomgr.println(concat!(
        "=== THIS GAME IS NOT HISTORICALLY ACCURATE ===",
        "\nThat being said, it is based off of actual events that happened to Mary Rowlandson in colonial America.\n",
        "All quotes in the game are taken from 'A Narrative of the Captivity and Restauration of Mrs. Mary Rowlandson'. ",
        "If the quotes feel out of place from the gameplay, then you've strayed far from what actually happened in the text. ",
        "Also, almost all of the dates in this game are estimates. The text does not give an exact timeline of events, unfortunately."
    ));

    // Prompt the player to start - if they don't want to, exit the game
    iomgr.println("Do you want to start the game?");
    if !iomgr.yes_no_prompt() {
        iomgr.print("Goodbye!");
        commands.quit_game();
    }

    commands.set_room("Invasion");
}

fn build_opening(mut cmds: Commands) {
    cmds.spawn(Room {
        name: "Opening",
        description: None,
    })
    .on_enter_room(opening_text)
    .insert(ActiveRoom);
}

fn main() {
    // Bevy app
    App::new()
        .add_plugin(AdventurePlugin)
        .add_startup_system(build_opening)
        .add_startup_system(invasion::build)
        .add_startup_system(remove_one::build)
        .add_startup_system(remove_two_three::build)
        .add_startup_system(remove_five::build)
        .add_startup_system(remove_eight::build)
        .add_startup_system(remove_eighteen::build)
        .add_startup_system(finale::build)
        .run();
}
