use std::collections::HashSet;
fn hs_from<'a>(input: &'a [&'a str]) -> HashSet<&'a str> {
    let mut hs = HashSet::new();
    for item in input.iter() {
        hs.insert(*item);
    }
    hs
}
fn main() {
    println!("{:?}", hs_from(&["2S 2H 2C 8D 2D", "4S 5H 5S 5D 5C"]));
}
