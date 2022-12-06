use bevy::prelude::*;
use bevy_adventure::prelude::*;

fn eighteenth_remove_description(mut iomgr: ResMut<IOManager>) {
    // Standard opening stuffs
    iomgr.clear();
    iomgr.println(concat!(
        "\"…The Squaw was boiling horses feet; then she cut me off a little piece, and gave one of the English Children a piece also: ",
        "Being very hungry, I had quickly eat up mine; but the Child could not bite it… ",
        "but lay sucking, gnawing, chewing, and slobbering it in the mouth and hand; ",
        "then I took it of the child, and eat it myself; and savoury it was to my taste.\"\n",
        "\n===\n"
    ));
    iomgr.println(concat!(
        "April, 1676\n",
        "Traveling from Conneticut\n",
        "Morning\n",
    ));

    // Transition from last chapter
    iomgr.println(concat!(
        "By the time you wake up, the soldiers have already left for the battle and North-Hampton. ",
        "Your group leaves for a small, native town, where you'll stay the night. ",
        "On the way there, you see a dead Englishman on the ground. You try to ignore it - you don't know who it is. ",
        "Once you arrive in the town, you hear of some colonial children who are held captive here. ",
        "You also smell food coming from one of the wigwams."
    ));

    iomgr.autoprompt();
}

fn sleep(iomgr: Res<IOManager>, mut cmds: Commands) {
    iomgr.println("Exhausted from your day, you go to sleep.");

    iomgr.println("\n\nEND CHAPTER 6");
    iomgr.println("Proceed to finale? (No will quit game)");
    if iomgr.yes_no_prompt() {
        cmds.set_room("Remove Five");
    } else {
        iomgr.println("Goodbye!");
        cmds.quit_game();
    }
}

fn talk_to_children(iomgr: Res<IOManager>) {
    iomgr.println(concat!(
        "The kids seem surprisingly well, for being captives. ",
        "They aren't hurt, and seem well-fed. ",
        "You realize one of the children is your sister's daughter! ",
        "Still, her master wouldn't let you stay long."
    ));
}

fn take_food(iomgr: Res<IOManager>) {
    iomgr.println(concat!(
        "The child can't eat it anyways, you decide. ",
        "Before you can regret it, you steal the food from the child and eat it. ",
        "Slobber and all. Gross. ",
        "You try not to think about how moral that choice was."
    ));
}

fn inspect_food(iomgr: Res<IOManager>) {
    iomgr.println(concat!(
        "You inspect the source of the smell, and find a wigwam, where a native woman is making food. ",
        "There are two English children in the wigwam as well. The woman offers you and one of the children food. ",
        "You eat it - it's delicious!\n",
        "...But you're still hungry...\n",
        "...The child... they're struggling to eat their food, it's too tough for them...\n",
        "...They can't eat it anyways, right?\n",
        "Every instinct says to take the food."
    ));
    if iomgr.yes_no_prompt() {
        take_food(iomgr);
    } else {
        iomgr.println(
            "You're trying to resist, but the food is good, and you're hungry...\nTake the food?",
        );
        if iomgr.yes_no_prompt() {
            take_food(iomgr);
        } else {
            iomgr.println(concat!(
                "You force yourself to walk out of the wigwam. ",
                "You question your own morals - you were so close to stealing food from a child! ",
                "You pray for forgiveness."
            ));
        }
    }
}

pub fn build(mut cmds: Commands) {
    let bed = cmds
        .spawn((Name("bed"), Aliases(vec!["sleep"])))
        .on_interact(WordType::Any, sleep)
        .id();

    let kids = cmds
        .spawn((Name("children"), Aliases(vec!["kids"])))
        .on_interact(WordType::Talk, talk_to_children)
        .on_interact(WordType::Move, talk_to_children)
        .id();

    let food = cmds
        .spawn((Name("food"), Aliases(vec!["wigwam", "smell"])))
        .on_interact(WordType::Move, inspect_food)
        .id();

    cmds.spawn(Room {
        name: "Remove Eighteen",
        description: None,
    })
    .on_enter_room(eighteenth_remove_description)
    .add_child(kids)
    .add_child(food)
    .add_child(bed);
}
