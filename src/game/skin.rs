use std::ops::Not;

use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, FromRepr};

use crate::game::{Card, NUM_MONSTERS};

use super::Suit;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default, FromRepr)]
#[repr(u8)]
pub enum ColorMode {
    #[default] Dark, Light
}

impl Not for ColorMode {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            ColorMode::Dark => ColorMode::Light,
            ColorMode::Light => ColorMode::Dark,
        }
    }
}

impl ColorMode {
    pub fn choose<T>(self, light: T, dark: T) -> T {
        if self == ColorMode::Light {light} else {dark}
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Skin;

pub const SYMBOLS_2_FONT_STR: &str = "'Noto Sans Symbols 2'";
pub const RANK_FONT_STR: &str = "TeXGyreChorus";

const COLOR_AMBER: [&str; 2] = ["#b70", "#ffb433"];
const COLOR_GREEN: [&str; 2] = ["#062", "#00ff55"];
const COLOR_BLUE: [&str; 2] = ["#00d", "#aaaaff"];
const COLOR_BLACK: [&str; 2] = ["#000", "#fff"];

const MONSTER_SYMBOLS: [&str; NUM_MONSTERS] = ["🦑", "🐋", "🦈"];

impl Skin {
    pub fn rank_text(self, card: Card) -> String {
        match card {
            Card::Number { rank, .. } => rank.to_string(),
            Card::Monster { .. } => "M".to_string(),
            Card::Pirate { .. } => "P".to_string(),
        }
    }

    pub fn suit_symbol(self, card: Card) -> &'static str {
        match card {
            Card::Number { suit, .. } => match suit {
                Suit::Parrots => "🦜",
                Suit::Anchors => "⚓",
                Suit::Treasures => "🪎",
            },
            Card::Monster { id } => MONSTER_SYMBOLS[id as usize],
            Card::Pirate { .. } => "😀🏴‍☠️",
        }
    }

    pub fn color(self, card: Card, mode: ColorMode) -> &'static str {
        let res = match card {
            Card::Number { suit, .. } => match suit {
                Suit::Parrots => COLOR_GREEN,
                Suit::Anchors => COLOR_BLUE,
                Suit::Treasures => COLOR_AMBER,
            },
            _ => COLOR_BLACK
        };
        res[mode as usize]
    }
}