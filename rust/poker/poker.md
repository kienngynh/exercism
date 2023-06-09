# Instruction

Pick the best hand(s) from a list of poker hands.

See [Wikipedia](https://en.wikipedia.org/wiki/List_of_poker_hands) for
an overview of poker hands.

  - Ranking a list of poker hands can be considered a sorting problem.
  - Rust provides the
    [sort](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort)
    method for `Vec<T> where T: Ord`.
  - [`Ord` types](https://doc.rust-lang.org/std/cmp/trait.Ord.html) form
    a [total order](https://en.wikipedia.org/wiki/Total_order) exactly
    one of `a < b`, `a == b`, or `a > b` must be true.
  - Poker hands do not conform to a total order: it is possible for two
    hands to be non-equal but has equal sort order. Example:
    `"3S 4S 5D 6H JH"`, `"3H 4H 5C 6C JD"`.
  - Rust provides the [`PartialOrd`
    trait](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html) to
    handle the case of sortable things which do not have a total order.
    However, it doesn’t provide a standard `sort` method for `Vec<T>
    where T: PartialOrd`. The standard idiom to sort a vector in this
    case; is `your_vec.sort_by(|a, b|
    a.partial_cmp(b).unwrap_or(Ordering::{Less|Equal|Greater}));`,
    depending on your needs.
  - You might consider implementing a type representing a poker hand
    that implements `PartialOrd`.

# Let’s solve it

## List of poker hand

First, let’s research the poker games, I didn’t play them before.
According to Wikipedia, I see a table of hand-ranking categories.

| Index | Name            | Example          | Description                                                               |
| :---- | :-------------- | :--------------- | :------------------------------------------------------------------------ |
| 1     | Royal flush     | 10S JS QS KS AS  | I think this is the best hand                                             |
| 2     | Straight flush  | JS 10S 9S 8S 7S  | Hand contain five cards of sequential rank and the same suite             |
| 3     | Four-of-a-kind  | 5C 5D 5H 5S 2D   | Hand contain five cards in one rank and a card in another rank            |
| 5     | Full house      | 6S 6H 6D KC KH   | Hand contains three cards in one rank and two cards in another rank       |
| 6     | Flush           | JD 9D 8D 4D 3D   | Hand contains five cards same suite, not all of the sequential rank       |
| 7     | Straight        | 10D 9S 8H 7D 6C  | Hand contains five cards of sequential rank and not all of the same suite |
| 8     | Three-of-a-kind | QC QS QH 9H 2S   | Hand contain one of three cards in the same rank                          |
| 9     | Two pair        | JS JH 3D 3C 2H   | Hand contain two of the double card in two other the same rank            |
| 10    | One pair        | 10S 10H 8D 7C KH | Hand contains two cards in the same rank                                  |
| 11    | High card       | KS QH 7D 4C 3H   | Nothing                                                                   |

  - Struct of instruction
      - Get source input
      - Determiner detail card on hand
          - Rank of card
              - Two
              - Three
              - Four
              - Five
              - Six
              - Seven
              - Eight
              - Nine
              - Ten
              - Jack
              - Queen
              - King
              - Ace
          - Suit of card
              - Spades
              - Clubs
              - Diamonds
              - Hearts
      - Determiner categories of hand
          - High card
          - One pair
          - Two pair
          - Three of a kind
          - Straight
          - Flush
          - Full house
          - Four of a kind
          - Straight flush
  - Todo list
      - [x] Struct
          - [ ] Hand
              - [ ] Convert from source -\> \[Card;5\]  
              - [ ] Compare with other hand  
              - [ ] Import to trait sort()  
      - [ ] Enum
          - [ ] Card
              - [ ] Convert form source -\> \[Rank,Suit\]  
          - [ ] Rank
              - [ ] Convert from source -\> \[Rank::?\]  
          - [ ] Suit
              - [ ] Convert from source -\> \[Suit::?\]
