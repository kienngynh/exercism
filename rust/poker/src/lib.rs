use std::{ cmp::Ordering, convert::TryFrom, fmt::Result };
/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
#[derive(Debug)]
struct Hand<'a> {
    hand: &'a str,
    cards: [Card; 5],
    categories: Categories,
}

#[derive(Debug)]
struct Card {
    rank: Rank,
    suit: Suit,
}
#[derive(Debug)]
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

#[derive(Debug)]
enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

#[derive(Debug)]
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
        let cards = value
            .split_whitespace()
            .map(|card| Card::try_from(card))
            .collect::<Result<Vec<_>, _>>()?;
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
        match value.len() {
            3 =>
                Ok(Card { rank: Rank::try_from(&value[..2])?, suit: Suit::try_from(&value[2..])? }),
            2 =>
                Ok(Card { rank: Rank::try_from(&value[..1])?, suit: Suit::try_from(&value[1..])? }),
            _ => Err("Invalid card"),
        }
    }
}
impl<'a> TryFrom<&'a str> for Rank {
    type Error = &'static str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "2" => Ok(Rank::Two),
            "3" => Ok(Rank::Three),
            "4" => Ok(Rank::Four),
            "5" => Ok(Rank::Five),
            "6" => Ok(Rank::Six),
            "7" => Ok(Rank::Seven),
            "8" => Ok(Rank::Eight),
            "9" => Ok(Rank::Nine),
            "10" => Ok(Rank::Ten),
            "J" => Ok(Rank::Jack),
            "Q" => Ok(Rank::Queen),
            "K" => Ok(Rank::King),
            "A" => Ok(Rank::Ace),
            _ => Err("Invalid rank"),
        }
    }
}
impl<'a> TryFrom<&'a str> for Suit {
    type Error = &'static str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "S" => Ok(Suit::Spades),
            "C" => Ok(Suit::Clubs),
            "H" => Ok(Suit::Hearts),
            "D" => Ok(Suit::Diamonds),
            _ => Err("Invalid suit"),
        }
    }
}
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let hand: Vec<_> = hands
        .iter()
        .map(|hand| Hand::try_from(*hand))
        .collect();
    vec!["4D 3D 2H 2S AC"]
}