use std::{ cmp::Ordering, convert::TryFrom, fmt::Result };
/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
struct Hand<'a> {
    hand: &'a str,
    cards: [Card; 5],
    categories: Categories,
}

struct Card {
    rank: Rank,
    suit: Suit,
}
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}
enum Categories {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}
impl<'a> TryFrom<&'a str> for Hand<'a> {
    type Error = &'static str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let cards: Vec<_> = value
            .split_whitespace()
            .map(|card| Card::try_from(card))
            .collect();
        if cards.len() == 5 {
            Ok(Hand {
                hand: value,
                cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
                categories: Categories::HighCard,
            })
        } else {
            Err("Invalid hands")
        }
    }
}
impl<'a> TryFrom<&'a str> for Card {
    type Error = &'static str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl<'a> TryFrom<&'a str> for Rank {
    type Error = &'static str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl<'a> TryFrom<&'a str> for Suit {
    type Error = &'static str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        todo!()
    }
}
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let hand: Vec<_> = hands
        .iter()
        .map(|hand| Hand::try_from(*hand))
        .collect();
    vec!["4D 3D 2H 2S AC"]
}