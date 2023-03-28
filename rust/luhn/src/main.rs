pub fn is_valid(code: &str) -> bool {
    let s = code
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    println!("{:?}", s);
    true
}

fn main() {
    println!("{:?}", is_valid(" 8273 1232 7352 0569"))
}
