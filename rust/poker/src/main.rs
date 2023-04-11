use poker::winning_hands;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = &[
        "4D 5S 6S 8D 3C",
        "2S 4C 7S 9H 10H",
        "3S 4S 5D 6H JH",
        "3H 4H 5C 6C JD",
        "2G 4Q 5A 6D KH",
    ];
    println!("{:?}", winning_hands(input))
}
