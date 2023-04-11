Instruction
===========

Pick the best hand(s) from a list of poker hands.

See [wikipedia](https://en.wikipedia.org/wiki/List_of_poker_hands) for
an overview of poker hands.

-   Ranking a list of poker hands can be considered a sorting problem.
-   Rust provides the
    [sort](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort)
    method for `Vec<T> where T: Ord`.
-   [`Ord` types](https://doc.rust-lang.org/std/cmp/trait.Ord.html) form
    a [total order](https://en.wikipedia.org/wiki/Total_order): exactly
    one of `a < b`, `a == b`, or `a > b` must be true.
-   Poker hands do not conform to a total order: it is possible for two
    hands to be non-equal but has equal sort order. Example:
    `"3S 4S 5D 6H JH"`, `"3H 4H 5C 6C JD"`.
-   Rust provides the [`PartialOrd`
    trait](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html) to
    handle the case of sortable things which do not have a total order.
    However, it doesn’t provide a standard `sort` method for
    `Vec<T> where T: PartialOrd`. The standard idiom to sort a vector in
    this case; is
    `your_vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::{Less|Equal|Greater}));`,
    depending on your needs.
-   You might consider implementing a type representing a poker hand
    that implements `PartialOrd`.

Let’s solve it
==============

List of poker hand
------------------

First, let’s research the poker games, I didn’t play them before.
According to Wikipedia, I see a table of hand-ranking categories.

<table>
<colgroup>
<col style="width: 25%" />
<col style="width: 25%" />
<col style="width: 25%" />
<col style="width: 25%" />
</colgroup>
<thead>
<tr class="header">
<th>Index</th>
<th>Name</th>
<th>Example</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>1</td>
<td>Royal flush</td>
<td>10S JS QS KS AS</td>
<td>I think this is the best hand</td>
</tr>
<tr class="even">
<td>2</td>
<td>Straight flush</td>
<td>JS 10S 9S 8S 7S</td>
<td>Hand contain five cards of sequential rank and same suite</td>
</tr>
<tr class="odd">
<td>3</td>
<td>Four of a kind</td>
<td>5C 5D 5H 5S 2D</td>
<td>Hand contain five cards in one rank and a card in another rank</td>
</tr>
<tr class="even">
<td>5</td>
<td>Full house</td>
<td>6S 6H 6D KC KH</td>
<td>Hand contains three cards in one rank and two cards in another rank</td>
</tr>
<tr class="odd">
<td>6</td>
<td>Flush</td>
<td>JD 9D 8D 4D 3D</td>
<td>Hand contains five cards same suite, not all of the sequential rank</td>
</tr>
<tr class="even">
<td>7</td>
<td>Straight</td>
<td>10D 9S 8H 7D 6C</td>
<td>Hand contains five cards of sequential rank and not all of the same suite</td>
</tr>
<tr class="odd">
<td>8</td>
<td>Three of a kind</td>
<td>QC QS QH 9H 2S</td>
<td>Hand contain one of three cards in the same rank</td>
</tr>
<tr class="even">
<td>9</td>
<td>Two pair</td>
<td>JS JH 3D 3C 2H</td>
<td>Hand contain two of the double card in two other the same rank</td>
</tr>
<tr class="odd">
<td>10</td>
<td>One pair</td>
<td>10S 10H 8D 7C KH</td>
<td>Hand contain two cards in same rank</td>
</tr>
<tr class="even">
<td>11</td>
<td>High card</td>
<td>KS QH 7D 4C 3H</td>
<td>Nothing</td>
</tr>
</tbody>
</table>

-   Todo-list
    -   Determinder detail card on hand
        -   Rank of card
        -   Suite of card
    -   Sort categoria
        -   Because the result is the best hand, not a list sorted. So,
            I want to use HashSet
        -   Overview, HashSet will contain
            {Categoria:\[“hand1”,“hand2”\]}
        -   Pick the best category then pick the best hand in there, we
            have the result.
        -   Test
