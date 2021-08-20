pub enum Prefix {}

pub enum Suffix {}

pub enum BaseTense {
    Past,
    Present,
    Future,
}

pub enum Tense {
    Indicative(BaseTense),
    Passive(BaseTense),
    Active(BaseTense),
    Volitive,
    Conditional,
    Infinitive,
}
pub enum Ending {
    Verb,
    Noun,
    Adjective,
    Adverb,
}

pub enum Radix {
    String,
}
pub struct Word {
    left: Vec<EndedSimpleWord>,
    right: SimpleWord,
    ending: Ending,
}

pub struct EndedSimpleWord {
    core: SimpleWord,
    ending: Option<Ending>,
}

pub struct  SimpleWord {
    prefixes: Vec<Prefix>,
    root: Radix,
    suffixes: Vec<Suffix>,
}