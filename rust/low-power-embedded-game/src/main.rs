#![allow(unused)]
pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        unimplemented!("implement `fn manhattan`")
    }
}

fn main() {
    let mut even_ints = evens(1..10);
    println!("{:?}", even_ints.next());
    println!("{:?}", even_ints.next());
    println!("{:?}", even_ints.next());
    println!("{:?}", even_ints.next());
    println!("{:?}", even_ints.next());
    println!("{:?}", even_ints.next());
}
