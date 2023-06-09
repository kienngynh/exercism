use std::{ cmp::Ordering, convert::TryFrom };
use std::collections::HashMap;

use counter::Counter;
/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
#[derive(Debug, Eq, Ord, PartialEq)]
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
        let mut cards = value
            .split_whitespace()
            .map(|card| Card::try_from(card))
            .collect::<Result<Vec<Card>, &'static str>>()?;
        cards.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));
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
impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(
            self.categories.cmp(&other.categories).then_with(|| {
                use crate::Categories::*;
                match &self.categories {
                    HighCard => self.cmp_high_card(other, 4),
                    OnePair => self.cmp_cascade_by_freq(other),
                    TwoPairs => self.cmp_cascade_by_freq(other),
                    ThreeOfAKind => self.cmp_cascade_by_freq(other),
                    Straight => self.cmp_straight(other),
                    Flush => self.cmp_high_card(other, 4),
                    FullHouse => self.cmp_cascade_by_freq(other),
                    FourOfAKind => self.cmp_cascade_by_freq(other),
                    StraightFlush => self.cmp_straight(other),
                }
            })
        )
    }
}
impl<'a> Hand<'a> {
    fn cmp_high_card(&self, other: &Hand, card: usize) -> Ordering {
        let mut ordering = self.cards[card].rank
            .get_value()
            .cmp(&other.cards[card].rank.get_value());
        if card != 0 {
            ordering = ordering.then_with(|| self.cmp_high_card(other, card - 1));
        }
        ordering
    }

    fn value_by_frequency(&self) -> (Option<Rank>, Option<Rank>, Option<Rank>) {
        let rank_counter = self.cards
            .iter()
            .map(|c| c.rank)
            .collect::<Counter<_>>();
        let mut rc_iter = rank_counter
            .most_common_tiebreaker(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Less))
            .into_iter()
            .map(|(rank, _count)| rank);
        (rc_iter.next(), rc_iter.next(), rc_iter.next())
    }

    fn cmp_cascade_by_freq(&self, other: &Hand) -> Ordering {
        let (s1, s2, s3) = self.value_by_frequency();
        let (o1, o2, o3) = other.value_by_frequency();
        s1.partial_cmp(&o1)
            .map(|c| {
                c.then(
                    s2
                        .partial_cmp(&o2)
                        .map(|c2| c2.then(s3.partial_cmp(&o3).unwrap_or(Ordering::Equal)))
                        .unwrap_or(Ordering::Equal)
                )
            })
            .unwrap_or(Ordering::Equal)
    }

    fn cmp_straight(&self, other: &Hand) -> Ordering {
        let s = if Categories::is_ace_low_straight(&self.cards) {
            5
        } else {
            self.cards[4].rank.get_value()
        };
        let o = if Categories::is_ace_low_straight(&other.cards) {
            5
        } else {
            other.cards[4].rank.get_value()
        };
        s.cmp(&o)
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
    fn is_ace_low_straight(cards: &[Card]) -> bool {
        // special case: ace-low straight
        // still depends on the sorted precondition
        cards[0].rank.get_value() == 2 &&
            cards[4].rank == Rank::Ace &&
            cards
                .windows(2)
                .take(3) // (0, 1), (1, 2), (2, 3) --> skips 4, ace
                .map(|pair| pair[1].rank.get_value() - pair[0].rank.get_value())
                .all(|diff| diff == 1)
    }
    fn check_categories(cards: Vec<Card>) -> Result<Self, &'static str> {
        if cards.len() != 5 {
            return Err("Invalid number of cards");
        }
        let mut sorted_cards = cards.clone();
        sorted_cards.sort();
        sorted_cards.reverse();
        let is_flush = sorted_cards.iter().all(|&card| card.suit == sorted_cards[0].suit);
        let is_straight =
            (sorted_cards[0].rank.get_value() - sorted_cards[4].rank.get_value() == 4 &&
                sorted_cards[1].rank.get_value() - sorted_cards[4].rank.get_value() == 3 &&
                sorted_cards[2].rank.get_value() - sorted_cards[4].rank.get_value() == 2 &&
                sorted_cards[3].rank.get_value() - sorted_cards[4].rank.get_value() == 1) ||
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
    hands.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));
    println!("{:?}", hands);
    hands
        .last()
        .map(|last| {
            hands
                .iter()
                .rev()
                .take_while(|&item| item.partial_cmp(last) == Some(Ordering::Equal))
                .map(|hand| hand.hand)
                .collect()
        })
        .unwrap_or_default()
}
