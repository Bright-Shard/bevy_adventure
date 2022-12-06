use phf::phf_map;

/// The type of action detected from different keywords.
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum WordType {
    Look,
    Take,
    Talk,
    Use,
    Open,
    Break,
    Move,
    Eat,
    Give,
    Ignore,
    PotentialTarget,
    Any,
}

pub const KEYWORDS: phf::Map<&'static str, WordType> = phf_map! {
    "look" => WordType::Look,
    "observe" => WordType::Look,
    "study" => WordType::Look,
    "examine" => WordType::Look,

    "talk" => WordType::Talk,
    "chat" => WordType::Talk,

    "use" => WordType::Use,

    "eat" => WordType::Eat,
    "devour" => WordType::Eat,
    "drink" => WordType::Eat,

    "take" => WordType::Take,
    "steal" => WordType::Take,
    "get" => WordType::Take,

    "give" => WordType::Give,
    "hand" => WordType::Give,

    "open" => WordType::Open,
    "unlock" => WordType::Open,

    "break" => WordType::Break,
    "destroy" => WordType::Break,
    "shatter" => WordType::Break,

    "move" => WordType::Move,
    "run" => WordType::Move,
    "go" => WordType::Move,
    "visit" => WordType::Move,
    "enter" => WordType::Move,
    "leave" => WordType::Move,
    "exit" => WordType::Move,
};
