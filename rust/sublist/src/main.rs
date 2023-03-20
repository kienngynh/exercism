#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
impl Comparison {
    pub fn compare<T: PartialEq>(_short_list: &[T], _long_list: &[T]) -> bool {
        _short_list.iter().all(|x| _long_list.contains(x))
    }
}
pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let v = match _first_list.len() as i32 - _second_list.len() as i32 {
        ..=-1 => (Comparison::compare(_first_list, _second_list), -1),
        0 => (_first_list == _second_list, 0),
        1.. => (Comparison::compare(_second_list, _first_list), 1),
    };
    match v {
        (true, -1) => Comparison::Sublist,
        (true, 0) => Comparison::Equal,
        (true, 1) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}

fn main() {
    let mut _first_list: Vec<u32> = (1..1_000_000).collect();
    println!("{:?}", sublist(&_first_list, &[3, 4, 5]))
}
