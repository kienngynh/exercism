#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
impl Comparison {
    pub fn compare<T: PartialEq>(_short_list: &[T], _long_list: &[T]) -> bool {
        let mut _long_list_cp = _long_list.clone();
        for _i in _long_list.iter() {
            if _long_list_cp.starts_with(_short_list) {
                return true;
            } else {
                (_,_long_list_cp) = _long_list_cp.split_at(1);
            }
        }
        return false;
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
