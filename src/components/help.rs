use dioxus::prelude::*;

use crate::{components::{CardText, VIDEO_GAMEPLAY, rem}, game::{Card, ColorMode, GameState, RANK_FONT_STR, ScreenState, Suit}};

#[component]
fn Emph(children: Element) -> Element {
    rsx! {
        strong {
            color: "#ff0",
            {children}
        }
    }
}

fn rank_text(text: &str) -> Element {
    rsx! {
        span {
            font_family: RANK_FONT_STR,
            font_size: "1.2em",
            {text}
        }
    }
}

#[component]
pub fn Help(mut game_state: Signal<GameState>) -> Element {
    let st = game_state.read();
    let skin = st.skin;

    let stack_example = || {
        let mut ite = [
            Card::Number { rank: 5, suit: Suit::Parrots },
            Card::Number { rank: 4, suit: Suit::Treasures },
            Card::Number { rank: 3, suit: Suit::Parrots },
            Card::Number { rank: 2, suit: Suit::Anchors },
        ].into_iter().map(|card| {
            rsx! {
                CardText { 
                    card, skin, color_mode: ColorMode::Light,
                }
            }
        });


        let last = ite.next().unwrap();
        rsx! {
            {ite.next().unwrap()},
            for x in ite { "–", {x} },
            " can be placed on ", {last}
        }
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; font-size: 3.5rem; color: #fff; padding: 4rem;",
            class: "help",

            div {
                text_align: "left",

                p {
                    margin_top: "0",
                    "The deck is a special 36-card deck, whose cards are in one of three categories:"

                    ul {
                        li { "27 ", Emph{"number"}, " cards: of ranks ",
                            {rank_text("1")}, "~", 
                            {rank_text("9")}, ", in each of 3 suits."
                        },

                        li {
                            "3 " Emph{"monster"} " cards (" {rank_text("M")} {")."}
                        },

                        li {
                            "6 " Emph{"pirate"} " cards (" {rank_text("P")} {")."}
                        },
                    }
                }

                p {
                    Emph{"Number"}, " cards stack in the ", Emph{"tableau"}, " by ", Emph {"decrementing rank"}, " and " Emph {"unlike suit"},
                    ". Such stacks of any size can be moved as a unit. (e.g. ",{stack_example()},")."
                }

                p {
                    Emph{"Number"} " cards can be put away to the " Emph{"foundations"} " at the top-right, in incrementing order by suit."
                }

                p {
                    "The 3 " Emph{"free cells"} " at the top left can each store a card of any kind."
                }

                p {
                    Emph{"Monster"} " and " Emph{"pirate"} " cards do not stack on other cards. To " Emph{"defeat"} " a monster:"
                    ul {
                        li {
                            "First, the " Emph{"monster"} " must be in a " Emph{"free cell"} ", and " Emph{"at least 2 pirates"}
                            " must be exposed."
                        }
                        li {
                            "Move a " Emph{"pirate"} " onto the " Emph{"monster"} "."
                        }
                        li {
                            "If there were more than 2 pirates exposed, you will need to select the second pirate to use."
                        }
                        li {
                            "All 3 cards will then permanently occupy the free cell."
                        }
                    }
                }

                p {
                    "To ",Emph{"win the game"},", defeat all monsters, and put all number cards away to the foundations."
                }

                p {
                    Emph{"Shortcut note:"}," Double-clicking on a card will try to use it up (number cards to the foundations, \
                    pirate cards to defeat monsters) if possible. Otherwise, it will go to a free cell."
                }

                div {
                    position: "absolute",
                    bottom: rem(2.),
                    width: "92rem",
                    display: "flex",
                    justify_content: "center",

                    a {
                        href: VIDEO_GAMEPLAY,
                        target: "_blank",
                        text_decoration: "none",
                        margin_right: rem(4.),
                        div {
                            width: rem(30.),
                            position: "relative",
                            class: "game-button",
                            "Example video"
                        }
                    }

                    div {
                        width: rem(30.),
                        position: "relative",
                        class: "game-button",
                        onclick: move |_| game_state.write().screen_state = ScreenState::Game,
                        "Back to game"
                    }
                }
            }
        }
    }
}