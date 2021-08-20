use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use lazy_static::lazy_static;

use crate::*;

const PREFIXES: [&str; 4] = ["bo", "ge", "ekster", "ek"];
const SUFFIXES: [&str; 1] = ["lol"];
const RADIX: [&str; 4] = ["manĝ", "kur", "ir", "san"];
const ALL_TOKENS: [&str; 13] = [
    "manĝ", "kur", "ir", "san", "bo", "ge", "ekster", "ek", "bone", "kol", "eg", "o", "koleg",
];

lazy_static! {
    static ref PREFIXES_AC: AhoCorasick = AhoCorasickBuilder::new()
        .match_kind(MatchKind::Standard)
        .build(PREFIXES);
    static ref SUFFIXES_AC: AhoCorasick = AhoCorasickBuilder::new()
        .match_kind(MatchKind::Standard)
        .build(SUFFIXES);
    static ref ALL_AC: AhoCorasick = AhoCorasickBuilder::new()
        .match_kind(MatchKind::Standard)
        .build(ALL_TOKENS);
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Prefix {
    Bo,
    Ge,
    Ekster,
    Ek,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Suffix {}

fixe_parsing!(
    Prefix,
    [(Bo, "bo"), (Ge, "ge"), (Ekster, "ekster"), (Ek, "ek")]
);

fixe_parsing!(Suffix, []);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BaseTense {
    Past,
    Present,
    Future,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Tense {
    Indicative(BaseTense),
    Passive(BaseTense),
    Active(BaseTense),
    Volitive,
    Conditional,
    Infinitive,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Ending {
    Verb,
    Noun,
    Adjective,
    Adverb,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Radix {
    String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Prefix(Prefix),
    Suffix(Suffix),
    Radix(Radix),
    Ending(Ending),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Word {
    left: Vec<EndedSimpleWord>,
    right: SimpleWord,
    ending: Ending,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EndedSimpleWord {
    core: SimpleWord,
    ending: Option<Ending>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SimpleWord {
    prefixes: Vec<Prefix>,
    root: Radix,
    suffixes: Vec<Suffix>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Extend<T> {
    None,
    Some((T, usize)),
    Multiple(Vec<(T, usize)>),
}

fn parse_fragment(src: &str) -> Extend<&'static str> {
    let aho_res = ALL_AC
        .find_overlapping_iter(src)
        .filter(|x| x.start() == 0)
        .collect::<Vec<_>>();
    if aho_res.is_empty() {
        Extend::None
    } else if aho_res.len() == 1 {
        Extend::Some((
            ALL_TOKENS[aho_res[0].pattern()],
            aho_res[0].end() - aho_res[0].start(),
        ))
    } else {
        Extend::Multiple(
            aho_res
                .into_iter()
                .map(|elt| (ALL_TOKENS[elt.pattern()], elt.end() - elt.start()))
                .collect::<Vec<_>>(),
        )
    }
}

fn parse_src_aux(
    src: &str,
    vector: &mut Vec<(bool, Vec<&'static str>)>,
    long_index: usize,
) -> usize {
    //println!("Lel {:#?}", vector);
    match parse_fragment(src) {
        Extend::None => {
            vector[long_index].0 = false;
            long_index + 1
        }
        Extend::Some((unique_tok, tok_len)) => {
            vector[long_index].1.push(unique_tok);
            if src.len() == tok_len {
                return long_index + 1;
            }
            parse_src_aux(&src[tok_len..], vector, long_index)
        }
        Extend::Multiple(tok_vec) => {
            println!("Got multiple");
            vector[long_index].1.push(tok_vec.first().unwrap().0);
            let mut new_long_index = long_index;
            let mut init = true;
            let temp_copy = vector[long_index].1.clone();
            for (tok, tok_len) in tok_vec {
                if !init {
                    vector.push((true, temp_copy.clone()));
                }
                vector[new_long_index].1.pop();
                vector[new_long_index].1.push(tok);
                init = false;
                new_long_index = parse_src_aux(&src[tok_len..], vector, new_long_index);
            }
            new_long_index
        }
    }
}

pub fn parse_src(src: &str) -> Vec<Vec<&'static str>> {
    let mut res = vec![(true, Vec::new())];
    parse_src_aux(src, &mut res, 0);
    res.into_iter()
        .filter_map(|tup| if tup.0 { Some(tup.1) } else { None })
        .collect::<Vec<Vec<&'static str>>>()
}

pub fn parse_word(_word: &str) -> Result<Word, ()> {
    todo!()
}
