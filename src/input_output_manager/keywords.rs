use phf::phf_map;

/// The type of action detected from different keywords.
#[derive(Debug, Clone)]
pub enum WordType {
    Look,
    Take,
    Talk,
    Use,
    Open,
    Break,
    Move,
    Ignore,
    PotentialTarget,
}

pub const KEYWORDS: phf::Map<&'static str, WordType> = phf_map! {
    "look" => WordType::Look,
    "observe" => WordType::Look,
    "study" => WordType::Look,
    "examine" => WordType::Look,

    "talk" => WordType::Talk,
    "chat" => WordType::Talk,

    "use" => WordType::Use,

    "take" => WordType::Take,
    "steal" => WordType::Take,

    "open" => WordType::Open,
    "unlock" => WordType::Open,

    "break" => WordType::Break,
    "destroy" => WordType::Break,
    "shatter" => WordType::Break,

    "move" => WordType::Move,
    "go" => WordType::Move,
    "visit" => WordType::Move,
    "enter" => WordType::Move,

    "to" => WordType::Ignore,
    "the" => WordType::Ignore,
    "a" => WordType::Ignore,
    "an" => WordType::Ignore,
};
