use std::collections::HashSet;
fn main() {
    let mut book = HashSet::new();
    book.insert("Hello".to_string());
    book.insert("World".to_string());
    println!("{:?}", book)
}
