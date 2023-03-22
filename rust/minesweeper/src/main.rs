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
    let mut result = Vec::new();
    let (x, y) = (
        minefield.len(),
        minefield[0].chars().collect::<Vec<char>>().len(),
    );
    for i in 0..x {
        let line = minefield[i].chars().collect::<Vec<char>>();
        for j in 0..y {
            match (i == 0 || i == x, j == 0 || j == y, line[j] != '*') {
                (true, true, true) => (),
                (true, false, true) => (),
                (false, true, true) => (),
                (false, false, true) => (),
                _ => (),
            }
        }
    }
    result
}
fn main() {
    let test_case = ["1*22*1", "12*322", " 123*2", "112*4*", "1*22*2", "111111"];
    let cleaned = remove_annotations(&test_case);
    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
    println!("{:?}", annotate(&cleaned_strs))
}
