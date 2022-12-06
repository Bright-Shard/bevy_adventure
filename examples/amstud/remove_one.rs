use bevy::prelude::*;
use bevy_adventure::prelude::*;

fn first_remove_description(mut iomgr: ResMut<IOManager>, checkpoints: Res<Checkpoints>) {
    iomgr.clear();
    iomgr.println(concat!(
        "\"this was the dolefullest night that ever my eyes saw. ",
        "Oh the roaring, and singing and danceing, and yelling of those black creatures in the night, ",
        "which made the place a lively resemblance of hell.\"\n",
        "\n===\n"
    ));
    iomgr.println(concat!(
        "February 10, 1676\n",
        "Just outside Lancaster\n",
        "Dusk\n\n",
        "It seems the native warriors won the battle. ",
        "In total, some twelve colonists were killed or mortally wounded. Twice that were taken as captives. ",
        "Only one adult and three children seem to have escaped the battle safely.\n"
    ));

    // Updates on what happened last chapter

    if checkpoints.0.contains(&"playerShot") {
        iomgr.print(concat!(
            "Still, you have your own worries to think about. ",
            "Your side hurts immensely where you were shot, and there's no way to get medical help right now, as you're traveling. ",
        ))
    }

    iomgr.println(concat!(
        "The native soldiers are traveling back to a hill just outside of Lancaster. ",
        "They are, of course, taking their captives with them; thus, everyone must walk the mile to the top of the hill.\n"
    ));

    if checkpoints.0.contains(&"kids") {
        iomgr.println("You're currently walking with your kids.");

        if checkpoints.0.contains(&"sarahShot") {
            iomgr.println(concat!(
                "Sarah isn't doing well. ",
                "The bullet that hit her went through her hand and stomach. ",
                "You're carrying her up the hill.\n"
            ));
        } else {
            iomgr.println(concat!(
                "Everyone seems to be doing OK, all things considered. ",
                "The children are scared, but they seem to alright other than that. ",
                "Thank goodness none of them are hurt!\n"
            ));
        }
    }

    // This chapter
    iomgr.println("...\n");

    iomgr.println(concat!(
        "You've arrived at the hill. ",
        "The natives are already setting up camp. They're in high spirits, which disgusts you. ",
        "They're preparing food - a miserable set of meats from various animals, roasted, cooked, or boiled. ",
        "It also looks like they're preparing for festivities, with dancing and singing. They seem to be setting up and practicing now.\n\n",
        "(Note: When you're done interacting with everyone, go to bed to start the next chapter.)"
    ));

    iomgr.autoprompt();
}

fn eat_food(iomgr: Res<IOManager>) {
    iomgr.println(
        "You grudgingly eat some of the food. It's not terrible, for being made by heathens.",
    );
}
fn take_food(
    iomgr: Res<IOManager>,
    mut inv: ResMut<Inventory>,
    query: Query<(Entity, &bevy_adventure::prelude::Name)>,
) {
    let (food_entity, _) = query.iter().find(|(_, name)| name.0 == "food").unwrap();

    if !inv.0.contains(&food_entity) {
        iomgr.println("You take some of the food for yourself.");
        inv.0.push(food_entity)
    } else {
        iomgr.println("You already have food!");
    }
}
fn talk_to_kids(iomgr: Res<IOManager>, checkpoints: Res<Checkpoints>) {
    if checkpoints.0.contains(&"sarahShot") {
        iomgr.println("Sarah's condition is deteriorating. As you're still in the wilderness, you can't get help for Sarah. You hope she'll be OK.");
        iomgr.println("The other kids are fine, but tired from the walk.");
    } else {
        iomgr.println("Your kids alright, but tired from the walk.");
    }

    if !checkpoints.0.contains(&"fedKids") {
        iomgr.println("\nYour kids all look hungry, perhaps you could get them some food?");
    }
}
fn give_food(
    iomgr: Res<IOManager>,
    mut inv: ResMut<Inventory>,
    mut checkpoints: ResMut<Checkpoints>,
    query: Query<(Entity, &bevy_adventure::prelude::Name)>,
) {
    let (food_entity, _) = query.iter().find(|(_, name)| name.0 == "food").unwrap();

    if inv.0.contains(&food_entity) && !checkpoints.0.contains(&"fedKids") {
        iomgr.println("You give some food to the kids. They eat quickly, and look much better aftwards, but a little tired.");
        let pos = inv.0.iter().position(|item| *item == food_entity);
        inv.0.remove(pos.unwrap());
        checkpoints.0.push("fedKids");
    } else {
        iomgr.println("You don't have any food to give to the kids.");
    }
}
fn sleep(mut cmds: Commands, iomgr: Res<IOManager>, checkpoints: Res<Checkpoints>) {
    if checkpoints.0.contains(&"kids") {
        iomgr.println("Exhausted from your day, you put the kids to bed and then sleep yourself.");
    } else {
        iomgr.println("Exhausted from your day, you go to sleep.");
    }

    iomgr.println("\n\nEND CHAPTER 2");
    iomgr.println("Proceed to next chapter? (No will quit game)");
    if iomgr.yes_no_prompt() {
        cmds.set_room("Remove Two");
    } else {
        iomgr.println("Goodbye!");
        cmds.quit_game();
    }
}

pub fn build(mut cmds: Commands) {
    let food = cmds
        .spawn((Name("food"), Aliases(vec!["meat", "meats"])))
        .on_interact(WordType::Eat, eat_food)
        .on_interact(WordType::Take, take_food)
        .on_interact(WordType::Give, give_food)
        .id();

    let kids = cmds
        .spawn((Name("kids"), Aliases(vec!["children"])))
        .on_interact(WordType::Give, give_food)
        .on_interact(WordType::Talk, talk_to_kids)
        .id();

    let bed = cmds
        .spawn((Name("bed"), Aliases(vec!["sleep"])))
        .on_interact(WordType::Any, sleep)
        .id();

    cmds.spawn(Room {
        name: "Remove One",
        description: None,
    })
    .on_enter_room(first_remove_description)
    .add_child(food)
    .add_child(kids)
    .add_child(bed);
}
