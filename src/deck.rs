use amethyst::ecs::{Component, DenseVecStorage, World, WorldExt};
use amethyst::prelude::Builder;

#[derive(Debug)]
pub enum Suit {
    Green,
    Yellow,
    Red,
    Blue,
    Wizard,
    Jester,
}

#[derive(Debug)]
pub struct Card {
    value: u8,
    suit: Suit,
}

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Component for Deck {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_deck(mut world: World) {
    // init all the struct values
    // TODO: create entity
    // let mut entity_builder = world.create_entity();
    let mut deck: Deck = Deck { cards: Vec::new() };
    for suit in 0..4 {
        for value in 0..14 {
            // create base card values and colors
            deck.cards.push(Card {
                value: value,
                suit: match suit {
                    0 => Suit::Blue,
                    1 => Suit::Green,
                    2 => Suit::Red,
                    3 => Suit::Yellow,
                    _ => panic!("Incorrect suit for base values"),
                }
            });
        }
    }

    // then, add 4 wizards and 4 jesters
    for suit in 0..2 {
        for _value in 0..4 {
            deck.cards.push(Card {
                suit: match suit {
                    0 => Suit::Wizard,
                    1 => Suit::Jester,
                    _ => panic!("Incorrect suit for wizards and jesters"),
                },
                value: match suit {
                    0 => 13,
                    1 => 0,
                    _ => panic!("incorrect values for wizard and jesters"),
                }
            });
        }
    }
}

pub fn shuffle() {
    // TODO: shuffle existing deck
}
