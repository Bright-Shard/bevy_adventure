use bevy::prelude::*;
use bevy_adventure::prelude::*;

fn fifth_remove_description(
    mut iomgr: ResMut<IOManager>,
    checkpoints: Res<Checkpoints>,
    mut cmds: Commands,
) {
    // Standard opening stuffs
    iomgr.clear();
    iomgr.println(concat!(
        "\"The first week of my being among them, I hardly ate any thing; the second week, ",
        "I found my stomach grow very faint for the want of something; and yet it was very hard to get down their filthy trash: ",
        "but the third week, though I could think how formerly my stomach would turn against this or that, ",
        "and I could starve and dy before I eat such things, yet they were sweet and savoury to my taste.\"\n",
        "\n===\n"
    ));
    iomgr.println(concat!(
        "Late February, 1676\n",
        "Traveling from Wenimesset\n",
        "Morning\n",
    ));

    // Transition from previous chapter
    if checkpoints.0.contains(&"playerShot") {
        iomgr.println(concat!(
            "Your wound gets infected. You hope that with time it'll get better. ",
            "In reality, it's too late for you. The infection, plus the wound, combined with a lack ",
            "of much food in the past few days finally does you in. ",
            "\n\nEnding 1: Death"
        ));
        cmds.quit_game();
    }
    if checkpoints.0.contains(&"kids") {
        iomgr.println(concat!(
            "Unfortunately, by this point, you have been separated from your children. ",
            "Although you bump into them occassionally, that's all - ",
            "you've been sold to different masters.\n"
        ));
    }

    // This chapter
    iomgr.println(concat!(
        "The natives are on the move once again. ",
        "This time, you think it's because of the approaching colonial militia. ",
        "The natives rush to the Bacquaug River. ",
        "You can see some of the stronger ones carrying weaker or elderly ones as everyone travels.",
        "\n\n...\n"
    ));
    iomgr.println(concat!(
        "You've arrived! The natives start felling trees to make rafts. ",
        "You're one of the first people they let cross the river. ",
        "The natives set up camp on the other side. It'll be a few days before everyone can make it across.\n"
    ));

    // Camp description
    iomgr.println(concat!(
        "The camp on this side of the river is fairly small, as only a few people have crossed so far. ",
        "Still, there is food to eat and some natives to talk to."
    ));

    // Autoprompt
    iomgr.autoprompt();
}

fn talk_to_natives(iomgr: Res<IOManager>) {
    loop {
        match iomgr.options_prompt(vec![
            "Where are we going?",
            "Why is the militia after us?",
            "Say goodbye",
        ]) {
            1 => {
                iomgr.println(concat!(
                    "The natives tell you that the group is traveling to Conneticut. ",
                    "You're going to meet with Metacom, known to the colonists as King Phillip, ",
                    "the leader of the Wampanoag."
                ));
            }
            2 => {
                iomgr.println(concat!(
                    "The natives explain that you're in the middle of the King Phillip's War. The colonists, allied with some indigenous tribes, are fighting ",
                    "several other native tribes, including the Wampanoag. This is also why the natives attacked your home in Lancester in the first place."
                ));
            }
            _ => break,
        }
    }
}

fn eat(iomgr: Res<IOManager>) {
    iomgr.println(concat!(
        "You find yourself enjoying the food, surprisingly. ",
        "The first few weeks, you could barely bring yourself to eat the food; ",
        "but, this week, you finally give in, and the food is delicious!"
    ));
}

fn sleep(iomgr: Res<IOManager>, mut cmds: Commands) {
    iomgr.println(concat!(
        "There's not much to do besides wait, so you go to sleep.",
        "\n\n...\n\n",
        "By the next day, all of the natives have arrived to your side of the river. ",
        "Shortly after, the natives burn their wigwams and prepare to leave. ",
        "As they leave, you see the Colonial Militia approach the other side of the river. ",
        "Though they see the natives and burning wigwams, the river stops them from continuing their pursuit. ",
        "'How strange is the providence of God,' you think, 'to preserve these Heathens from our militia.'"
    ));

    iomgr.println("\n\nEND CHAPTER 4");
    iomgr.println("Proceed to next chapter? (No will quit game)");
    if iomgr.yes_no_prompt() {
        cmds.set_room("Remove Eight");
    } else {
        iomgr.println("Goodbye!");
        cmds.quit_game();
    }
}

pub fn build(mut cmds: Commands) {
    let bed = cmds
        .spawn((Name("bed"), Aliases(vec!["sleep"])))
        .on_interact(WordType::Any, sleep)
        .id();

    let food = cmds
        .spawn(Name("food"))
        .on_interact(WordType::Any, eat)
        .id();

    let natives = cmds
        .spawn((Name("natives"), Aliases(vec!["indians", "indigenous"])))
        .on_interact(WordType::Talk, talk_to_natives)
        .id();

    cmds.spawn(Room {
        name: "Remove Five",
        description: None,
    })
    .on_enter_room(fifth_remove_description)
    .add_child(food)
    .add_child(natives)
    .add_child(bed);
}
