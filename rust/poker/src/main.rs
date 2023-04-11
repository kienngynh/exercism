use std::collections::HashSet;
fn hs_from<'a>(input: &'a [&'a str]) -> HashSet<&'a str> {
    let mut hs = HashSet::new();
    for item in input.iter() {
        hs.insert(*item);
    }
    hs
}
fn main() {
    let input = &[
        "4D 5S 6S 8D 3C",
        "2S 4C 7S 9H 10H",
        "3S 4S 5D 6H JH",
        "3H 4H 5C 6C JD",
    ];
    println!("{:?}", input)
}
