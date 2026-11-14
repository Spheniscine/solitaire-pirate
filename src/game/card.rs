use std::ops::RangeInclusive;

use enum_map::Enum;
use serde::{Deserialize, Serialize, de::Visitor};
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, EnumCount, EnumIter, Enum)]
pub enum Suit {
    Parrots, Anchors, Treasures
}

impl Suit {
    pub fn code(self) -> char {
        match self {
            Suit::Parrots => 'P',
            Suit::Anchors => 'A',
            Suit::Treasures => 'T',
        }
    }
    pub fn from_code(c: char) -> Option<Self> {
        match c {
            'P' => Some(Suit::Parrots),
            'A' => Some(Suit::Anchors),
            'T' => Some(Suit::Treasures),
            _ => None,
        }
    }
}

impl Serialize for Suit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_char(self.code())
    }
}

impl<'de> Deserialize<'de> for Suit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        struct MyVisitor;
        impl<'de> Visitor<'de> for MyVisitor {
            type Value = Suit;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "suit code, one of characters RGB")
            }
            fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
                where E: serde::de::Error, {
                Suit::from_code(v).ok_or_else(|| E::custom(format!("invalid suit code: {}", v)))
            }
        }
        deserializer.deserialize_char(MyVisitor)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Card {
    Number { rank: u8, suit: Suit },
    Monster { id: u8 },
    Pirate { id: u8 },
}

pub const RANK_MIN: u8 = 1;
pub const RANK_MAX: u8 = 9;
pub const RANKS: RangeInclusive<u8> = RANK_MIN ..= RANK_MAX;
pub const NUM_SUITS: usize = Suit::COUNT;
pub const NUM_RANKS: usize = (RANK_MAX - RANK_MIN) as usize + 1;
pub const NUM_MONSTERS: usize = 3;
pub const NUM_PIRATES: usize = NUM_MONSTERS * 2;

pub const NUM_NUMBER_CARDS: usize = NUM_RANKS * NUM_SUITS;
pub const DECK_SIZE: usize = NUM_NUMBER_CARDS + NUM_MONSTERS + NUM_PIRATES;

impl Card {
    pub fn code(self) -> String {
        match self {
            Card::Number { rank, suit } => format!("{}{}", rank, suit.code()),
            Card::Monster { id } => format!("M{}", id),
            Card::Pirate { id } => format!("P{}", id),
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        if let Some(id) = code.strip_prefix('M') {
            return Some(Card::Monster { id: id.parse().ok()? })
        } else if let Some(id) = code.strip_prefix('P') {
            return Some(Card::Pirate { id: id.parse().ok()? })
        } else {
            let mut it = code.chars();
            let suit = Suit::from_code(it.next_back()?)?;
            let rank: u8 = it.as_str().parse().ok()?;
            if !RANKS.contains(&rank) { return None; }
            Some(Card::Number { rank, suit })
        }
    }

    pub fn is_monster(self) -> bool {
        matches!(self, Card::Monster { .. })
    }

    pub fn is_pirate(self) -> bool {
        matches!(self, Card::Pirate { .. })
    }
}


impl Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.code())
    }
}

impl<'de> Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        struct MyVisitor;
        impl<'de> Visitor<'de> for MyVisitor {
            type Value = Card;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "card code")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {
                Card::from_code(v).ok_or_else(|| E::custom(format!("invalid card code: {}", v)))
            }
        }
        deserializer.deserialize_str(MyVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{Card, Suit};

    const TEST_PAIRS: &[(Card, &str)] = &[
        (Card::Number { rank: 9, suit: Suit::Parrots }, "9P"),
        (Card::Monster { id: 0 }, "M0"),
        (Card::Pirate { id: 5 }, "P5"),
    ];

    #[test]
    fn card_to_code_test() {
        for (c, s) in TEST_PAIRS {
            assert_eq!(*s, c.code())
        }
    }

    #[test]
    fn card_from_code_test() {
        for (c, s) in TEST_PAIRS {
            assert_eq!(Some(*c), Card::from_code(*s))
        }
    }
}