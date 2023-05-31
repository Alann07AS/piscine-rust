use rand::Rng;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
	pub fn random() -> Suit {
        let random: u8 = rand::thread_rng().gen_range(0, 4);
        Suit::translate(random)
	}

	pub fn translate(value: u8) -> Suit {
        match value {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Spade,
            3 => Suit::Club,
            _ => Suit::Heart,
        }
	}
}

impl Rank {
	pub fn random() -> Rank {
        let random: u8 = rand::thread_rng().gen_range(1, 14);
        Rank::translate(random)
	}

	pub fn translate(value: u8) -> Rank {
        match value {
            1   => Rank::Ace,
            13  => Rank::King,
            12  => Rank::Queen,
            11  => Rank::Jack,
            nb  => Rank::Number(nb),
            // _   => Rank::Ace,
        }
	}
}

#[derive(Debug)]
pub struct Card {
	pub suit: Suit,
	pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    if card.suit == Suit::Spade && card.rank == Rank::Ace {
        true
    } else {
        false
    }
}