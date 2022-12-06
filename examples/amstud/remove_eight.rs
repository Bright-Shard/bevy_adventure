use bevy::prelude::*;
use bevy_adventure::prelude::*;
type Name = bevy_adventure::prelude::Name;

fn eighth_remove_description(mut iomgr: ResMut<IOManager>, checkpoints: Res<Checkpoints>) {
    // Standard opening stuffs
    iomgr.clear();
    iomgr.println(concat!(
        "\"I went, and he gave me a Pancake, about as big as two fingers; ",
        "it was made of parched wheat, beaten, and fryed in Bears grease, but I thought I never tasted pleasanter meat in my life…\"\n",
        "\n===\n"
    ));
    iomgr.println(concat!(
        "March, 1676\n",
        "Traveling to Conneticut\n",
        "Morning\n",
    ));

    // Catch up from the last chapter
    iomgr.println(concat!(
        "Finally, everyone makes it to your side of the river. The group sets of to continue travel.",
        "\n\n...\n\n",
        "Several days later, you encounter yet another river. You must cross this river to Conneticut in order to meet with King Philip. ",
        "The natives have brought two canoes to travel in; however, the second you get in yours, there's a new order. ",
        "The group gets out of the canoes and heads 5 miles north, to cross the river there. You think the reason is that ",
        "some colonial scouts have shown up.",
        "\n\n...\n"
    ));

    // Start this chapter
    if checkpoints.0.contains(&"kids") {
        iomgr.println(concat!(
            "The group finally takes a break from walking at noon. ",
            "There's food, and, unexpectedly, your son Joseph has shown up.\n",
            "(Tip: When you're done interacting with people, go to the river to continue your journey.)"
        ));
    } else {
        iomgr.println(concat!(
            "The group finally takes a break from walking at noon. ",
            "There's quite a lot of food to eat.\n",
            "(Tip: When you're done interacting with people, go to the river to continue your journey.)"
        ));
    }

    // Autoprompt
    iomgr.autoprompt();
}

fn king_philip_description(
    iomgr: Res<IOManager>,
    mut checkpoints: ResMut<Checkpoints>,
    mut cmds: Commands,
) {
    // Natives
    iomgr.println(concat!(
        "At last, you cross the river and arrive in King Philip's land. ",
        "As soon as you arrive, you are stunned - no, terrified - by the amount of pagans around you. ",
        "There's so many, it brings you to tears. Embarassingly, you start crying, in front of all of them.\n\n",
        "They ask you what's wrong. 'I shall be killed,' you respond. 'No,' a native responds, 'no one shall hurt you.'\n\n",
        "The natives do their best to comfort you. Another gives you lots of food, and a third gives you half a pint of peas - ",
        "which is worth quite a lot, at that time. If nothing else, the pagans are quite generous.",
        "\n\n...\n"
    ));

    // King Philip
    iomgr.println(concat!(
        "King Philip has invited you to talk. ",
        "When you meet him, he offers you a tobacco pipe.\n",
        "Do you want to smoke it?"
    ));
    if iomgr.yes_no_prompt() {
        iomgr.println(concat!(
            "You accept the pipe. ",
            "You've actually been smoking for years now."
        ));
    } else {
        iomgr.println(concat!(
            "You turn King Philip down. ",
            "You used to smoke, but hated how you always wanted another pipe after finishing, ",
            "so you stopped."
        ));
    }
    iomgr.println(concat!(
        "King Philip talks to you for a while. He tells you that the natives are gathering to attack North-Hampton. ",
        "Even as you speak, you can hear a native soldier going around announcing the plans outside. ",
        "But then, King Philip asks you something odd. He's heard you can make clothes, and wants you to make a shirt for his son.",
        "\nDo you accept?"
    ));
    if iomgr.yes_no_prompt() {
        checkpoints.0.push("kingPhilipShirt");
        iomgr.println(concat!(
            "You accept. King Philip looks happy. ",
            "He promises some payment in return."
        ));
    } else {
        iomgr.println(concat!(
            "You turn King Philip down. ",
            "He understands, but looks disappointed."
        ));
    }
    iomgr.println(concat!(
        "With that, King Philip bids you farewell.",
        "\n\n...\n"
    ));

    // Night after King Philip
    iomgr.println(concat!(
        "That night, the smell of food dominates the camp. ",
        "The soldiers are cooking food for their battle in North-Hampton. "
    ));
    if checkpoints.0.contains(&"kingPhilipShirt") {
        iomgr.println(concat!(
            "You spend the night working on a shirt for King Philip's son. ",
            "When you finish, as promised, King Philip pays you a shlling. ",
            "You assume the money is for your master - but your master refuses it! ",
            "So, you keep the money yourself."
        ));
    } else {
        iomgr.println(concat!(
            "Not having much else to do, you help with the cooking. ",
            "It really does smell quite good. You have to stop and get dinner yourself at one point!"
        ));
    }

    if checkpoints.0.contains(&"kingPhilipShirt") {
        iomgr.println(concat!(
            "King Philip was very happy with the shirt you made his son! ",
            "He asks you to make a cap as well, promising you dinner if you make it.\n",
            "Do you accept?"
        ));
        if iomgr.yes_no_prompt() {
            checkpoints.0.push("kingPhilipCap");
            iomgr.println(concat!(
                "King Philip thanks you in advance.",
                "\n\n...\n\n",
                "Finally, the cap is completed. You give it to King Philip. ",
                "As promised, you get to eat dinner with him. He gives you a Pancake, as thick as two fingers; ",
                "it's made of beaten wheat and fried in bear's grease. You've never had a more pleasant meal in your life!"
            ));
        }
    }

    // Next chapter
    iomgr.println("Exhausted from your day, you go to sleep.");

    iomgr.println("\n\nEND CHAPTER 5");
    iomgr.println("Proceed to next chapter? (No will quit game)");
    if iomgr.yes_no_prompt() {
        cmds.set_room("Remove Five");
    } else {
        iomgr.println("Goodbye!");
        cmds.quit_game();
    }
}

fn talk_to_joseph(iomgr: Res<IOManager>) {
    iomgr.println(concat!(
        "You talk to your son for a while, sharing stories about what's happened to you. ",
        "It seems Joseph essentially has a full family, with siblings and parents - although, he's still a captive from war. ",
        "Joseph also expresses that he wishes he could read the Bible, but has none."
    ));
}
fn give_bible_to_joseph(iomgr: Res<IOManager>, inv: Res<Inventory>, query: Query<(Entity, &Name)>) {
    let (bible_entity, _) = query.iter().find(|(_, name)| name.0 == "bible").unwrap();
    if inv.0.contains(&bible_entity) {
        iomgr.println(concat!(
            "Joseph eagerly accepts your Bible, and reads for a bit. ",
            "Joseph seems greatly comforted by the scripture."
        ));
    }
}

fn river(iomgr: Res<IOManager>, mut cmds: Commands) {
    iomgr.println(concat!(
        "The group keeps walking until that night, and then stops to rest. ",
        "The next morning, you cross the river, into King Philip's land."
    ));
    cmds.set_room("King Philip");
}

pub fn build(mut cmds: Commands) {
    let river = cmds
        .spawn(Name("river"))
        .on_interact(WordType::Move, river)
        .id();

    let joseph = cmds
        .spawn((Name("joseph"), Aliases(vec!["son", "child"])))
        .on_interact(WordType::Talk, talk_to_joseph)
        .on_interact(WordType::Give, give_bible_to_joseph)
        .id();

    cmds.spawn(Room {
        name: "Remove Eight",
        description: None,
    })
    .add_child(joseph)
    .add_child(river)
    .on_enter_room(eighth_remove_description);

    cmds.spawn(Room {
        name: "King Philip",
        description: None,
    })
    .on_enter_room(king_philip_description);
}
