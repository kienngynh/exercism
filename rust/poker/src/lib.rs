/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Hash)]
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
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Hash)]
enum Suit {
    Spades,
    Club,
    Diamonds,
    Hearts,
}
#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Card {
    rank: Rank,
    suit: Suit,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Hash)]
enum Categories {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKing,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}
#[derive(Debug, PartialEq, Eq)]
struct Hand<'a> {
    source: &'a str,
    cards: [Card; 5],
    categories: Categories,
}
impl<'a> Hand<'a> {}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    unimplemented!("Out of {hands:?}, which hand wins?")
}
