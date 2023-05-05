use std::{cmp::Ordering, convert::TryFrom};
/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
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
impl<'a> TryFrom<&'a str> for Rank {
    type Error = &'static str;
    fn try_from(source: &str) -> Result<Self, Self::Error> {
        match source {
            "A" => Ok(Rank::Ace),
            "K" => Ok(Rank::King),
            "Q" => Ok(Rank::Queen),
            "J" => Ok(Rank::Jack),
            "10" => Ok(Rank::Ten),
            "9" => Ok(Rank::Nine),
            "8" => Ok(Rank::Eight),
            "7" => Ok(Rank::Seven),
            "6" => Ok(Rank::Six),
            "5" => Ok(Rank::Five),
            "4" => Ok(Rank::Four),
            "3" => Ok(Rank::Three),
            "2" => Ok(Rank::Two),
            _ => Err("Invalid rank"),
        }
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Hash)]
enum Suit {
    Spades,
    Clubs,
    Diamonds,
    Hearts,
}
impl<'a> TryFrom<&'a str> for Suit {
    type Error = &'static str;
    fn try_from(source: &str) -> Result<Self, Self::Error> {
        match source {
            "S" => Ok(Suit::Spades),
            "H" => Ok(Suit::Hearts),
            "C" => Ok(Suit::Clubs),
            "D" => Ok(Suit::Diamonds),
            _ => Err("Invalid suit"),
        }
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd,Clone, Copy)]
struct Card {
    rank: Rank,
    suit: Suit,
}
impl Card {
    fn try_from_split(source: &str, split: usize) -> Result<Self, &'static str> {
        Ok(Card {
            rank: Rank::try_from(&source[..split])?,
            suit: Suit::try_from(&source[split..])?,
        })
    }
}
impl<'a> TryFrom<&'a str> for Card {
    type Error = &'static str;

    fn try_from(source: &str) -> Result<Self, Self::Error> {
        match source.len() {
            3 => Card::try_from_split(source, 2),
            2 => Card::try_from_split(source, 1),
            _ => Err("Invalid card"),
        }
    }
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
#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand<'a> {
    source: &'a str,
    cards: [Card; 5],
    categories: Categories,
}
impl<'a> TryFrom<&'a str> for Hand<'a> {
    type Error = &'static str;

    fn try_from(source: &'a str) -> Result<Self, Self::Error> {
        let mut cards = source
            .split_whitespace()
            .map(Card::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        cards.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));
        if cards.len() == 5 {
            Ok(Hand {
                source,
                cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
                categories: Categories::HighCard,
            })
        } else {
            Err("Invalid hand")
        }
    }
}
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    
    let hands = hands
        .iter()
        .map(|source| Hand::try_from(*source))
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_default();        
    println!("{:?}",hands);
    vec!["4D 3D 2E 2E 2E 2E 2E 2E"]
}
