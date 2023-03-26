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
pub fn get_index(position: (i32, i32), column: i32, row: i32) -> Option<usize> {
    if position.0 >= 0 && position.0 < column && position.1 >= 0 && position.1 < row {
        Some((position.0 * column + position.1) as usize)
    } else {
        None
    }
}
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let vector_minefield = minefield
        .iter()
        .flat_map(|&s| s.chars())
        .collect::<Vec<char>>();
    let (colume, row) = (minefield.len(), vector_minefield.len() / minefield.len());
    let mut s = String::new();
    for (k, v) in vector_minefield.iter().enumerate() {
        if k % colume == 0 {
            s = String::new()
        }
        let value = match (k, v) {
            (_, '*') => '*',
            (k, _) => {
                let (p_x, p_y) = (k / colume, k % colume);
                println!(
                    "{:?}",
                    (
                        get_index((p_x as i32 - 1, p_y as i32 - 1), colume as i32, row as i32),
                        get_index((p_x as i32 - 1, p_y as i32), colume as i32, row as i32),
                        get_index((p_x as i32 - 1, p_y as i32 + 1), colume as i32, row as i32),
                        get_index((p_x as i32, p_y as i32 - 1), colume as i32, row as i32),
                        get_index((p_x as i32, p_y as i32 + 1), colume as i32, row as i32),
                        get_index((p_x as i32 + 1, p_y as i32 - 1), colume as i32, row as i32),
                        get_index((p_x as i32 + 1, p_y as i32), colume as i32, row as i32),
                        get_index((p_x as i32 + 1, p_y as i32 + 1), colume as i32, row as i32),
                    )
                );
                ' '
            }
        };
        s.push_str(&value.to_string());
        if k % colume == colume - 1 {
            result.push(s.clone())
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
