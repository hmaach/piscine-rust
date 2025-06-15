use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Number(u8),
    Ace,
    King,
    Queen,
    Jack,
}

impl Suit {
    pub fn random() -> Suit {
        match rand::thread_rng().gen_range(1..=4) {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            _ => Suit::Club,
        }
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
    pub fn random() -> Self {
        let n = rand::thread_rng().gen_range(1..=13);

        match n {
            13 => Rank::King,
            12 => Rank::Queen,
            11 => Rank::Jack,
            1 => Rank::Ace,
            _ => Rank::Number(n),
        }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            13 => Rank::King,
            12 => Rank::Queen,
            11 => Rank::Jack,
            1 => Rank::Ace,
            _ => Rank::Number(value),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}
