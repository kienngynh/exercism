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
    let vector_minefield = minefield
        .iter()
        .flat_map(|&s| s.chars())
        .collect::<Vec<char>>();
    let (x, y) = (minefield.len(), vector_minefield.len() / minefield.len());
    let mut s = String::new();
    for (k, v) in vector_minefield.iter().enumerate() {
        if k % x == 0 {
            s = String::new()
        }
        let value = match (k, v) {
            (_, '*') => '*',
            (k, _) => {
                println!(
                    "{:?}",
                    (
                        Some(vector_minefield[k - y - 1]),
                        Some(vector_minefield[k - y]),
                        Some(vector_minefield[k - y + 1]),
                        Some(vector_minefield[k - 1]),
                        Some(vector_minefield[k + 1]),
                        Some(vector_minefield[k + y - 1]),
                        Some(vector_minefield[k + y]),
                        Some(vector_minefield[k + y + 1]),
                    )
                );
                '2'
            }
        };
        s.push_str(&value.to_string());
        if k % x == x - 1 {
            result.push(s.clone())
        }
    }
    /*
    let (x, y) = (minefield.len(), vector_minefield.len());
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
