#[derive(PartialEq, Eq , Debug)]

pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}


#[derive(PartialEq, Eq , Debug)]
pub enum Rank {
    Number(u8),
    Ace,
    King,
    Queen,
    Jack,
}

use rand::Rng;

impl Suit {
    pub fn random() -> Suit {
        let num = rand::thread_rng().gen_range(1..4);
        Self::translate(num)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            _ => Suit::Club,
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let num = rand::thread_rng().gen_range(1..13);
        Self::translate(num)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => Rank::Number(value),
        }
    }
}

#[derive(PartialEq, Eq , Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card.suit == crate::Suit::Spade && card.rank == crate::Rank::Ace
}
