#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
impl Comparison {
    pub fn compare<T: PartialEq>(_short_list: &[T], _long_list: &[T]) -> Self {
        Comparison::Sublist        
    }
    pub fn compare_equal<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Self {
        if _first_list == _second_list {
            Comparison::Equal
        } else {
            Comparison::Unequal
        }
    }
}
pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match _first_list.len() as i32 - _second_list.len() as i32 {
        ..=-1 => Comparison::compare(_first_list, _second_list),
        0 => Comparison::compare_equal(_first_list, _second_list),
        1.. => Comparison::compare(_second_list, _first_list),
    }
}