use poker::winning_hands;

fn main() {
    let input = &[
        "4D 5S 6S 8D 3C",
        "2S 2C 2D 2H 10H",
        "3S 4S 5D 6H 7H",
        "3H 4H 5C 6C JD",
    ];
    println!("{:?}", winning_hands(input))
}
