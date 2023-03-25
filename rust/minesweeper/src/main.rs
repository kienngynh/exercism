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
    let mut result: Vec<String> = Vec::new();
    let vector_minefield: Vec<_> = minefield
        .iter()
        .flat_map(|&s| s.chars())
        .enumerate()
        .collect();
    let (x, y) = (minefield.len(), vector_minefield.len());
    println!("{:?}", vector_minefield);
    /*<_>
    let (x, y) = (minefield.len(), v.len());
    for i in 0..y {
        match (i / x, i % (y / x + 1)) {
            (_, _) => todo!(),
        }
    }
    */
    result
}
fn main() {
    let test_case = ["1*22*1", "12*322", " 123*2", "112*4*", "1*22*2", "111111"];
    let cleaned = remove_annotations(&test_case);
    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
    println!("{:?}", annotate(&cleaned_strs))
}
