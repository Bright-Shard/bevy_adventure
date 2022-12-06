use bevy::prelude::*;
use bevy_adventure::prelude::*;
type Name = bevy_adventure::prelude::Name;

fn second_remove_description(
    iomgr: Res<IOManager>,
    checkpoints: Res<Checkpoints>,
    mut cmds: Commands,
) {
    // Standard opening stuffs
    iomgr.clear();
    iomgr.println(concat!(
        "\"…yet so it must be, that I must sit all this cold winter night upon the cold snowy ground, with my sick Child in my armes, ",
        "looking that every hour would be the last of its life; and having no Christian friend near me, either to comfort or help me.\"\n",
        "\n===\n"
    ));
    iomgr.println(concat!(
        "February 11, 1676\n",
        "Traveling from Lancaster\n",
        "Morning\n",
    ));

    if checkpoints.0.contains(&"sarahShot") {
        // Transition from last chapter
        iomgr.println(concat!(
            "Sarah's condition is terrible. The natives, to their credit, have put her on a horse so she doesn't have to walk. ",
            "This, however, doesn't help your mental condition. Your own daughter is in such a sickly state! ",
            "You morbidly follow the horse Sarah is on, your worry growing over time. ",
            "Finally, it's too much. You take Sarah and carry her yourself, for so long that arms start to give out. ",
            "The natives, seeing you in such a state, put you and Sarah on your own horse.\n",
            "And yet, things still don't get any better. The horse you were given doesn't have a saddle, and, sure enough...\n",
            "After several minutes of riding, you and Sarah fall off the horse! And the indigenous people laughed at you. They laughed.",
            "\n\n...\n"
        ));

        // This chapter
        iomgr.println(concat!(
            "You've finally stopped for the night. The natives start to set up camp. ",
            "It starts snowing that night. The snow is cold. Freezing cold. Sarah develops a fever. ",
            "The night stretches on for ages. You keep asking the natives for more water for Sarah. ",
            "Sarah only gets wosre and worse. You clutch her in your arms, desparately hoping she will survive the night. ",
            "By some miracle, she survives, and the morning arrives."
        ));
    } else if checkpoints.0.contains(&"playerShot") {
        if checkpoints.0.contains(&"kids") {
            iomgr.println(concat!(
                "The natives keep moving. As captives, you and your kids have no choice but to follow. Still, your wound hurts terribly. ",
                "After a while, the soldiers give you a horse to ride on. One way or another, you make it through the day. ",
                "Once night arrives, the soldiers set up camp. It starts snowing. The snow is freezing cold. Your wound hurts even worse. ",
                "The kids enjoy the snow, but that's the only thing that's gone right at this point. ",
                "One way or another, you make it through the night, and the morning arrives."
            ))
        } else {
            iomgr.println(concat!(
                "The natives keep moving. As a captive, you have no choice but to follow. Still, your wound hurts terribly. ",
                "After a while, the soldiers give you a horse to ride on. One way or another, you make it through the day. ",
                "Once night arrives, the soldiers set up camp. It starts snowing. The snow is freezing cold. Your wound hurts even worse. ",
                "One way or another, you make it through the night, and the morning arrives."
            ));
        }
    } else {
        let opening = if checkpoints.0.contains(&"kids") {
            "You and your kids"
        } else {
            "You"
        };

        // Transition from last chapter
        iomgr.println(format!(
            "{} {}",
            opening,
            concat!(
                "have a full day of travel ahead of you. ",
                "You're heading straight into the wilderness. You've never been this far from Lancester! ",
                "Still, all the walking isn't fun. And, you're now technically a captive of the heathen natives.",
                "\n\n...\n"
            )
        ));

        // This chapter
        iomgr.println(concat!(
            "Finally, the natives stop for the day. They set up camp and start a fire. It gets cold, and starts to snow. ",
            "The snow is pretty, but also extremely cold. Everyone stays close to the fire."
        ));
        if checkpoints.0.contains(&"kids") {
            iomgr.println("Still, if nothing else, the kids love the snow. You have to force them to go to bed after a while.");
        }
        iomgr.println(
            "Exhausted from a day of traveling, you go to sleep, and the morning arrives.",
        );
    }

    cmds.set_room("Remove Three");
}

fn third_remove_description(mut iomgr: ResMut<IOManager>, checkpoints: Res<Checkpoints>) {
    // Standard opening stuffs
    iomgr.println(concat!(
        "\n\n...\n\n\"…then they told me it was upon the hill: then they went and shewed me where it was, ",
        "where I saw the ground was newly digged, and there they told me they had buried it…\"\n",
        "\n===\n"
    ));
    iomgr.println(concat!(
        "February 12, 1676\n",
        "Traveling from Lancaster\n",
        "Morning\n",
    ));

    if checkpoints.0.contains(&"sarahShot") {
        iomgr.println(concat!(
            "The day has a grim start. The natives give you a horse to ride on, again - but you're hungry. ",
            "Intensely hungry. You've no food for a while now. Still, you hang on, and in the afternoon, finally arrive...",
            "\n\nThe town you arrive in is named Wenimesset. You arrive the day before the Sabbath (Sunday).\n"
        ));
        iomgr.println(concat!("Sarah is close to death."));
        iomgr.println(concat!(
            "You spend the next several days and nights on your knees, Sarah in your lap, helplessly watching your child die. ",
            "One day, one 'Robbert Pepper' stops by to see you. He suggests putting oak leaves on the injuries to heal them.\n",
        ));
        iomgr.println(concat!(
            "Desparate for a solution, you do as he says.\n",
            "Miraculously, you do indeed seem to heal!\n",
            "Sarah, though, does not. ",
            "You continue to sit there on your knees, Sarah on your lap. The natives stop by occassionally. ",
            "The only comfort they offer is knocking Sarah on the head. You, of course, do not let them. What terrible comforters.\n",
        ));
        iomgr.println(concat!(
            "Finally, Sarah passes. Nine days after the initial wounding, and 6 years, 5 months since her birth.\n\n",
            "You spend the whole night with your dead child, lying morbidly beside her body. When you wake up, the natives ",
            "order you to leave the child. You do, but the first chance you get, you go back to check on her. ",
            "You then find out that the natives have already buried her. They lead you to the top of a hill, where a freshly dug grave awaits you."
        ));
    } else if checkpoints.0.contains(&"playerShot") {
        iomgr.println(concat!(
            "The day has a morbid start. Your wound aches, and you're starving. You haven't had food in ages. ",
            "Still, you hang on, and the day improves. You finally arrive in a town named Wenimesset. ",
            "When you arrive, you hear of another colonist in town, one 'Robbert Pepper'. Perhaps you should go talk to him."
        ));
    } else {
        let opening = if checkpoints.0.contains(&"kids") {
            "You and your kids wake up hungry. You realized you haven't eaten in a while. The kids are extremely unhappy."
        } else {
            "You wake up hungry, and realize you haven't eaten in quite a while."
        };
        iomgr.print(opening);
        iomgr.println(concat!(
            " Still, you hang on until the afternoon, and arrive in a town named Wenimesset. ",
            "Finally, you can rest. No more walking for quite a while."
        ));
    }

    iomgr.println(concat!(
        "Shortly after arriving in town, you hear an enormous roar. ",
        "...no, wait. Not a roar. Is it... whooping? Confused, you step out of your wigwam. ",
        "A band of native soldiers enters the town. You later find out that each whoop represents a confirmed kill in battle. ",
        "The soldiers are returning from a successful raid on another colony, Medfield. They look disgustingly happy. Heathens."
    ));

    iomgr.autoprompt();
}

fn talk_to_soldiers(
    iomgr: Res<IOManager>,
    mut inv: ResMut<Inventory>,
    query: Query<(Entity, &Name)>,
) {
    let (bible_id, _) = query.iter().find(|(_, item)| item.0 == "bible").unwrap();
    if !inv.0.contains(&bible_id) {
        iomgr.println(concat!(
            "You approach the soldiers as they return from Medfield. You ask them about the battle, and find this out:\n",
            "- The natives were attacking the colony of Medfield\n",
            "- The natives killed 23 colonists (they showed you the disgusting scalps of the colonists... such is their nature.)\n",
            "- One of the natives took a Bible from the colony\n",
            "The soldier with the Bible offers it to you, saying you will be allowed to read it. Do you accept it?"
        ));
        if iomgr.yes_no_prompt() {
            inv.0.push(bible_id);
        }
    } else {
        iomgr.println("There's nothing you want to do with heathen soldiers.");
    }
}

fn sleep(iomgr: Res<IOManager>, mut cmds: Commands, checkpoints: Res<Checkpoints>) {
    if checkpoints.0.contains(&"kids") {
        iomgr.println("Exhausted from your day, you put the kids to bed and then sleep yourself.");
    } else {
        iomgr.println("Exhausted from your day, you go to sleep.");
    }

    iomgr.println("\n\nEND CHAPTER 3");
    iomgr.println("Proceed to next chapter? (No will quit game)");
    if iomgr.yes_no_prompt() {
        cmds.set_room("Remove Five");
    } else {
        iomgr.println("Goodbye!");
        cmds.quit_game();
    }
}

fn talk_to_pepper(iomgr: Res<IOManager>) {
    iomgr.println(concat!(
        "You go and talk to Robbert Pepper. ",
        "He tells you that he was also involved in a battle, and gives you a story from it. ",
        "Once he finishes, he tells you that he was injured in the battle, but used oak leaves ",
        "to heal. Perhaps you should go grab some."
    ));
}

fn get_oak_leaves(
    iomgr: Res<IOManager>,
    mut inv: ResMut<Inventory>,
    query: Query<(Entity, &Name)>,
) {
    iomgr.println(concat!(
        "You go up to an oak tree and collect several leaves. ",
        "Perhaps there's some way to use these?"
    ));
    let (entity, _) = query.iter().find(|(_, name)| name.0 == "leaves").unwrap();
    inv.0.push(entity);
}
fn use_oak_leaves(
    iomgr: Res<IOManager>,
    mut inv: ResMut<Inventory>,
    mut checkpoints: ResMut<Checkpoints>,
    query: Query<(Entity, &Name)>,
) {
    iomgr.println(concat!(
        "Following Robbert's advice, you place the oak leaves on your wound. ",
        "Time passes, and sure enough, you are healed. Your wound still hurts, ",
        "but it is doing much better than before."
    ));

    let index = checkpoints
        .0
        .iter()
        .position(|checkpoint| *checkpoint == "playerShot")
        .unwrap();
    checkpoints.0.remove(index);

    let (entity, _) = query.iter().find(|(_, name)| name.0 == "leaves").unwrap();
    let index = inv.0.iter().position(|item| *item == entity).unwrap();
    inv.0.remove(index);
}

pub fn build(mut cmds: Commands) {
    let soldiers = cmds
        .spawn((Name("soldiers"), Aliases(vec!["army", "warriors"])))
        .on_interact(WordType::Talk, talk_to_soldiers)
        .id();

    let bed = cmds
        .spawn((Name("bed"), Aliases(vec!["sleep"])))
        .on_interact(WordType::Any, sleep)
        .id();

    let robert = cmds
        .spawn((Name("Robbert"), Aliases(vec!["Robert", "Pepper"])))
        .on_interact(WordType::Talk, talk_to_pepper)
        .id();

    let tree = cmds
        .spawn(Name("oak"))
        .on_interact(WordType::Take, get_oak_leaves)
        .id();

    cmds.spawn(Name("bible"));
    cmds.spawn(Name("leaves"))
        .on_interact(WordType::Any, use_oak_leaves);

    cmds.spawn(Room {
        name: "Remove Two",
        description: None,
    })
    .on_enter_room(second_remove_description);

    cmds.spawn(Room {
        name: "Remove Three",
        description: None,
    })
    .on_enter_room(third_remove_description)
    .add_child(soldiers)
    .add_child(bed)
    .add_child(tree)
    .add_child(robert);
}
