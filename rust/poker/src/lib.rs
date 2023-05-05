use std::{cmp::Ordering, convert::TryFrom};
/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
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
    Heart,
    Diamonds,
}
struct Card {
    rank: Rank,
    suit: Suit,
}
enum Categories {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straght,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}
struct Hand<'a> {
    hand: &'a str,
    cards: [Card; 5],
    categories: Categories,
}
impl<'a> TryFrom<&'a str> for Hand<'a> {
    type Error = &'static str;

    fn try_from(hand: &'a str) -> Result<Self, Self::Error> {
        let mut cards: Vec<_> = hand
            .split_whitespace()
            .map(|card| Card::try_from(card))
            .collect();
        todo!();
    }
}
impl<'a> TryFrom<&'a str> for Card {
    type Error = &'static str;
    fn try_from(card: &'a str) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl<'a> TryFrom<&'a str> for Suit {
    type Error = &'static str;
    fn try_from(suit: &'a str) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl<'a> TryFrom<&'a str> for Rank {
    type Error = &'static str;
    fn try_from(rank: &'a str) -> Result<Self, Self::Error> {
        todo!()
    }
}
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let hand: Vec<_> = hands.iter().map(|hand| Hand::try_from(*hand)).collect();
    vec!["4D 3D 2H 2S AC"]
}
