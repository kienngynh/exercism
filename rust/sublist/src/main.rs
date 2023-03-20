#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
impl Comparison {
    pub fn compare<T: PartialEq>(_short_list: &[T], _long_list: &[T]) -> Self {
        if _short_list == _long_list {
            Comparison::Equal
        } else {
            Comparison::Unequal
        }
    }
}
pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match _first_list.len() as i32 - _second_list.len() as i32 {
        ..=0 => Comparison::compare(_first_list, _second_list),
        1.. => Comparison::compare(_second_list, _first_list),
    }
}

fn main() {
    let mut _first_list: Vec<u32> = (1..1_000_000).collect();
    println!("{:?}", sublist(&[3, 4, 5], &[3, 4, 5]))
}
