use bevy::prelude::*;
use bevy_adventure::prelude::*;

// Starting text of the game
fn invasion(mut iomgr: ResMut<IOManager>) {
    // Clear screen
    iomgr.clear();

    // Opening stuffs
    iomgr.println(concat!(
        "\"I had often before this said, that if the Indians should come, I should ",
        "chuse rather to be killed by them then taken alive[,] but when it came to the tryal my mind changed; ",
        "their glittering weapons so daunted my spirit, that I chose rather to go along with those ",
        "(as I may say) ravenous beasts…\"\n",
        "\n===\n"
    ));
    iomgr.println(concat!(
        "February 10, 1676\n",
        "Lancaster, Massachusetts, Colonial America\n",
        "Dawn\n",
    ));

    // Actual game start
    iomgr.println(concat!(
        "You wake up to the sound of gunshots. ",
        "Through the window, you see swarms of indigenous soldiers charging towards your home in Lancaster. ",
        "Several houses are ablaze, with smoke billowing towards the sky in a series of darkened spires. ",
        "You watch in horror as natives surround your home, armed with rifles. ",
        "They fire, and waves of bullets tear into your house like hail.\n\n",
        "You need to leave quickly, but you know your kids are still in the house. You should go get them!"
    ));

    // Prompt player to act
    iomgr.autoprompt();
}

// When the player takes their kids with them
fn take_kids(iomgr: Res<IOManager>, mut checkpoints: ResMut<Checkpoints>) {
    // Check if the player already did this, if not trigger the talk event
    if checkpoints.0.contains(&"kids") {
        iomgr.println("You grab your kids.");
        checkpoints.0.push("kids");
    }
}

// When the player talks to their kids
fn talk_to_kids(iomgr: Res<IOManager>) {
    iomgr.println(concat!(
        "Your kids are scared.\n",
        "You try to calm them, but the constant gunshots continue to frighten them."
    ));
}

// When the player leaves the house
fn on_leave(iomgr: Res<IOManager>, checkpoints: ResMut<Checkpoints>, cmds: Commands) {
    // Print conundrum while leaving
    iomgr.println(concat!(
        "You turn to leave, but as soon as you get to the door, another wave of bullets slams into your house!\n",
        "You turn to go back, but you see a band of indigenous warriors that have broken into your house.\n",
        "Which way do you go - forwards to the bullets, or back to the warriors?"
    ));

    // Make the player choose to go forwards or backwards, then see if they chose forwards
    if iomgr.two_option_prompt(
        "Please choose to go forwards or backwards.",
        "Forwards",
        "Backwards",
    ) == 1
    {
        // If the player chose to go forwards
        leave_front_door(iomgr, checkpoints, cmds)
    } else {
        // If the player chose to go backwards
        soldier_encounter(iomgr, checkpoints, cmds);
    }
}

// Player leaves through the front door
fn leave_front_door(iomgr: Res<IOManager>, mut checkpoints: ResMut<Checkpoints>, cmds: Commands) {
    // If they did, print text for going outside
    let sentence_one = if checkpoints.0.contains(&"kids") {
        "You sprint out the door, dragging your kids with you. You pray the bullets will miss all of you."
    } else {
        "You sprint out the door, praying that the bullets will somehow miss you."
    };

    iomgr.println(format!(
        "{} {}\n{}",
        sentence_one,
        "Unfortunately, you're not so lucky - almost immediately, a bullet hits you in the side!",
        "You hesitate for a moment - should you keep going?"
    ));
    checkpoints.0.push("playerShot");

    if iomgr.yes_no_prompt() {
        iomgr.println(concat!(
            "Boldly, you keep sprinting. ",
            "You make a mad dash from the battle, down the hill your house is built on. ",
            "After several minutes of running, you slow. It looks like you're out of the battle.\n\n",
            "It's only when you stop that you realize your youngest daughter, Sarah, was also shot. ",
            "To make matters worse, you look to your left and realize another band of indigenous soldiers stands there...\n"
        ));
        checkpoints.0.push("sarahShot");
    } else {
        iomgr.println("You sprint back to the house.");
    }

    // Either way, the player encounters soldiers, L
    soldier_encounter(iomgr, checkpoints, cmds);
}

// Player encounters soldiers
fn soldier_encounter(iomgr: Res<IOManager>, checkpoints: ResMut<Checkpoints>, mut cmds: Commands) {
    iomgr.println(concat!(
        "You turn to face the soldiers. They have their weapons drawn - there's no chance to fight them. ",
        "You raise your hands in the air, hoping they'll spare your life. And, sure enough, the soldiers spare you. ",
        "Instead, they opt to take you hostage. At least you're alive."
    ));

    if checkpoints.0.contains(&"kids") {
        iomgr.println("Your kids are terrified, but the soldiers make no motion to harm them.");
    }

    iomgr.println("\n\nEND CHAPTER 1");
    iomgr.println("Proceed to next chapter? (No will quit game)");
    if iomgr.yes_no_prompt() {
        cmds.set_room("Remove One");
    } else {
        iomgr.println("Goodbye!");
        cmds.quit_game();
    }
}

pub fn build(mut commands: Commands) {
    let kids = commands
        .spawn((Name("kids"), Aliases(vec!["children"])))
        .on_interact(WordType::Take, take_kids)
        .on_interact(WordType::Talk, talk_to_kids)
        .on_interact(WordType::Move, talk_to_kids)
        .id();

    commands
        .spawn(Room {
            name: "Invasion",
            description: None,
        })
        .on_enter_room(invasion)
        .on_interact(WordType::Move, on_leave)
        .add_child(kids);
}
