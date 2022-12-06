use bevy::prelude::*;
use bevy_adventure::prelude::*;

fn finale_description(iomgr: Res<IOManager>, mut cmds: Commands) {
    iomgr.clear();

    iomgr.println(concat!(
        "Several weeks later, two natives - who have been converted to Christianity - deliver a letter about captives. ",
        "You find out the natives plan to release you; and, embarassingly, break down into tears in front of them. ",
        "Shortly after, a council met with you to determine the ransom price. You uncertainly name twenty pounds, but request less. ",
        "The council forwards your ransom of twenty pounds. Several friends in the colonies raise the money and pay your ransom; ",
        "the natives release you back to your previous life.",
        "\n\n......\n"
    ));
    iomgr.println(concat!(
        "Over the course of their history, Colonial Americans would kill thousands of indigenous people. ",
        "Colonial Americans would also take indegenous people as slaves, just as they did with Africans.\n",
        "Slaves were not fed when they cried.\n",
        "Slaves were not paid for their work.\n",
        "Slaves were almost never released from bondage for money.\n",
        "Slaves did not slowly warm up to the culture of their masters. They were not treated so kindly. \n",
        "\n\n",
        "Which raises the question: Who, truly, were the heathens?"
    ));

    cmds.quit_game();
}

pub fn build(mut cmds: Commands) {
    cmds.spawn(Room {
        name: "Finale",
        description: None,
    })
    .on_enter_room(finale_description);
}
