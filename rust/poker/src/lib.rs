use std::{ cmp::Ordering, convert::TryFrom };
use std::collections::HashMap;
/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Hand<'a> {
    hand: &'a str,
    cards: [Card; 5],
    categories: Categories,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    rank: Rank,
    suit: Suit,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
impl Rank {
    fn get_value(&self) -> u8 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Categories {
    HighCard,
    OnePair,
    TwoPairs,
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
            .collect::<Result<Vec<Card>, &'static str>>()?;
        if cards.len() == 5 {
            Ok(Hand {
                hand: value,
                cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
                categories: Categories::check_categories(cards)?,
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
impl Categories {
    fn check_categories(cards: Vec<Card>) -> Result<Self, &'static str> {
        if cards.len() != 5 {
            return Err("Invalid number of cards");
        }
        let mut sorted_cards = cards.clone();
        sorted_cards.sort();
        sorted_cards.reverse();
        let is_flush = sorted_cards.iter().all(|&card| card.suit == sorted_cards[0].suit);
        let is_straight =
            sorted_cards[0].rank.get_value() - sorted_cards[4].rank.get_value() == 4 ||
            (sorted_cards[0].rank.get_value() == 14 &&
                sorted_cards[1].rank.get_value() == 5 &&
                sorted_cards[1].rank.get_value() - sorted_cards[4].rank.get_value() == 3);
        if is_flush && is_straight {
            return Ok(Categories::StraightFlush);
        } else if is_flush {
            return Ok(Categories::Flush);
        } else if is_straight {
            return Ok(Categories::Straight);
        } else {
            let mut rank_counts: HashMap<Rank, u8> = HashMap::new();
            for card in cards {
                *rank_counts.entry(card.rank).or_insert(0) += 1;
            }
            let mut pairs: Vec<Rank> = vec![];
            let mut three_of_a_kind: Option<Rank> = None;
            let mut four_of_a_kind: Option<Rank> = None;
            for (rank, count) in rank_counts {
                if count == 2 {
                    pairs.push(rank);
                } else if count == 3 {
                    three_of_a_kind = Some(rank);
                } else if count == 4 {
                    four_of_a_kind = Some(rank);
                }
            }
            if let Some(_rank) = four_of_a_kind {
                Ok(Categories::FourOfAKind)
            } else if let Some(_rank) = three_of_a_kind {
                if !pairs.is_empty() {
                    Ok(Categories::FullHouse)
                } else {
                    Ok(Categories::ThreeOfAKind)
                }
            } else if pairs.len() == 2 {
                Ok(Categories::TwoPairs)
            } else if pairs.len() == 1 {
                Ok(Categories::OnePair)
            } else {
                Ok(Categories::HighCard)
            }
        }
    }
}
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands = match
        hands
            .iter()
            .map(|hand| Hand::try_from(*hand))
            .collect::<Result<Vec<Hand>, &'static str>>()
    {
        Ok(hands) => hands,
        Err(err) => {
            return vec![err];
        }
    };
    hands.sort();
    for h in hands {
        println!("{:?}",h)
    }
    let x = vec!["4D 3D 2H 2S AC"];
    x
}