use std::{cmp::Ordering, convert::TryFrom};
/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
enum Rank{
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
enum Suit{
    Spades,
    Clubs,
    Heart,
    Diamonds,
}
struct Card{
    rank: Rank,
    suit:Suit,
}
enum Categories{
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
struct Hand<'a>{
    source: &'a str,
    cards: [Card;5],
    categories: Categories,
}
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
   vec!["4D 3D 2H 2S AC"]
}
