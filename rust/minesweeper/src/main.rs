fn remove_annotations(board: &[&str]) -> Vec<String> {
    board.iter().map(|r| remove_annotations_in_row(r)).collect()
}

fn remove_annotations_in_row(row: &str) -> String {
    row.chars()
        .map(|ch| match ch {
            '*' => '*',
            _ => ' ',
        })
        .collect()
}
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mi = minefield.iter().map(|&x| x.as_bytes()).collect::<Vec<_>>();
    println!("{:?}",mi);
    vec!["111".to_owned(), "222".to_owned()]
}
fn main() {
    let test_case = ["1*22*1", "12*322", " 123*2", "112*4*", "1*22*2", "111111"];
    let cleaned = remove_annotations(&test_case);
    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
    println!("{:?}", annotate(&cleaned_strs))
}
