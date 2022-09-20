use itertools::{izip, Itertools};
use std::{
    array::IntoIter as ArrIter,
    convert::TryFrom,
    fmt::{Display, Write},
    fs::File,
    io::{BufRead, BufReader},
    iter::{self, Zip},
    ops::Deref,
    path::Path,
    str::FromStr,
};

mod error;
mod macros;
pub mod scoring;

pub use error::{LoadError, LoadResult, ParseError};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Lang {
    EN,
    DE,
}

impl Lang {
    pub const fn wordlist(&self) -> &'static str {
        match self {
            Self::EN => "en_words.txt",
            Self::DE => "de_words.txt",
        }
    }

    pub const fn sollist(&self) -> &'static str {
        match self {
            Self::EN => "en_sols.txt",
            Self::DE => "de_sols.txt",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Word(pub [Letter; 5]);

impl Deref for Word {
    type Target = [Letter; 5];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Word {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        match itertools::process_results(s.chars().map(Letter::try_from), |it| it.collect_tuple())?
        {
            Some((a, b, c, d, e)) => Ok(Word([a, b, c, d, e])),
            None => Err(ParseError),
        }
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.into_iter().format(""))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl TryFrom<u8> for Letter {
    type Error = ParseError;

    fn try_from(n: u8) -> Result<Self, ParseError> {
        Ok(match n {
            b'a' | b'A' => Letter::A,
            b'b' | b'B' => Letter::B,
            b'c' | b'C' => Letter::C,
            b'd' | b'D' => Letter::D,
            b'e' | b'E' => Letter::E,
            b'f' | b'F' => Letter::F,
            b'g' | b'G' => Letter::G,
            b'h' | b'H' => Letter::H,
            b'i' | b'I' => Letter::I,
            b'j' | b'J' => Letter::J,
            b'k' | b'K' => Letter::K,
            b'l' | b'L' => Letter::L,
            b'm' | b'M' => Letter::M,
            b'n' | b'N' => Letter::N,
            b'o' | b'O' => Letter::O,
            b'p' | b'P' => Letter::P,
            b'q' | b'Q' => Letter::Q,
            b'r' | b'R' => Letter::R,
            b's' | b'S' => Letter::S,
            b't' | b'T' => Letter::T,
            b'u' | b'U' => Letter::U,
            b'v' | b'V' => Letter::V,
            b'w' | b'W' => Letter::W,
            b'x' | b'X' => Letter::X,
            b'y' | b'Y' => Letter::Y,
            b'z' | b'Z' => Letter::Z,
            _ => return Err(ParseError),
        })
    }
}

impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl TryFrom<char> for Letter {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Self, ParseError> {
        if c.is_ascii_alphabetic() {
            (c as u8).try_into()
        } else {
            Err(ParseError)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LMask {
    Miss,
    Partial,
    Match,
}

impl Display for LMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Miss => '\u{2B1C}',
            Self::Partial => '\u{01F7E8}',
            Self::Match => '\u{01F7E9}',
        };
        f.write_char(c)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct WMask {
    pub mask: [LMask; 5],
    pub guess: Word,
}

impl Display for WMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.mask.into_iter().format(""))
    }
}

impl IntoIterator for WMask {
    type Item = (Letter, LMask);
    type IntoIter = Zip<ArrIter<Letter, 5>, ArrIter<LMask, 5>>;

    fn into_iter(self) -> Self::IntoIter {
        self.guess.into_iter().zip(self.mask)
    }
}

impl WMask {
    pub fn matches(&self, word: Word) -> bool {
        // Keeps track of the letters which have already been counted
        let mut counted = [false; 5];

        // Check matches
        for (wletter, counted, (gletter, lmask)) in izip!(*word, &mut counted, *self) {
            if lmask == LMask::Match {
                if wletter == gletter {
                    *counted = true;
                } else {
                    return false;
                }
            }
        }

        // Check partials
        for (gi, (guess, lmask)) in self.into_iter().enumerate() {
            if lmask == LMask::Partial {
                match word
                    .into_iter()
                    .zip(counted.iter_mut())
                    .enumerate()
                    .find(|(i, (l, c))| !**c && *l == guess && *i != gi)
                {
                    Some((_, (_, c))) => *c = true,
                    None => return false,
                }
            }
        }

        // Check misses
        for (guess, lmask) in *self {
            if lmask == LMask::Miss {
                for (l, c) in iter::zip(*word, counted) {
                    if !c && l == guess {
                        return false;
                    }
                }
            }
        }

        // It's a match!
        true
    }

    pub fn compare(guess: Word, answer: Word) -> Self {
        let mut counted = [false; 5];
        let mut mask = [LMask::Miss; 5];

        // Mark matches
        for (guess, answer, counted, mask) in izip!(*guess, *answer, &mut counted, &mut mask) {
            if guess == answer {
                *counted = true;
                *mask = LMask::Match;
            }
        }

        // Mark partials
        for (guess, mask) in iter::zip(*guess, &mut mask) {
            if *mask == LMask::Miss {
                for (answer, counted) in iter::zip(*answer, &mut counted) {
                    if !*counted && guess == answer {
                        *counted = true;
                        *mask = LMask::Partial;
                        break;
                    }
                }
            }
        }

        Self { mask, guess }
    }
}

pub fn load_wordlist<P: AsRef<Path>>(path: P) -> LoadResult<Vec<Word>> {
    BufReader::new(File::open(path)?)
        .lines()
        .map(|line| Ok(line?.parse()?))
        .collect()
}
