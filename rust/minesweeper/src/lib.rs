pub fn get_index(position: (i32, i32), row: i32, column: i32) -> Option<usize> {
    if position.0 >= 0 && position.0 < column && position.1 >= 0 && position.1 < row {
        Some((position.0 * row + position.1) as usize)
    } else {
        None
    }
}
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let vector_minefield = minefield
        .iter()
        .flat_map(|&s| s.chars())
        .collect::<Vec<char>>();
    match minefield.len() {
        0 => vec![],
        row => match vector_minefield.len() / minefield.len() {
            0 => vec!["".to_string()],
            column => {
                let mut result: Vec<String> = Vec::new();
                let mut s = String::new();
                for (k, v) in vector_minefield.iter().enumerate() {
                    if k % column == 0 {
                        s = String::new()
                    }
                    let value: String = match (k, v) {
                        (_, '*') => '*'.to_string(),
                        (k, _) => {
                            let (p_x, p_y) = (k / row, k % row);
                            let v = vec![
                                get_index(
                                    (p_x as i32 - 1, p_y as i32 - 1),
                                    row as i32,
                                    column as i32,
                                ),
                                get_index((p_x as i32 - 1, p_y as i32), row as i32, column as i32),
                                get_index(
                                    (p_x as i32 - 1, p_y as i32 + 1),
                                    row as i32,
                                    column as i32,
                                ),
                                get_index((p_x as i32, p_y as i32 - 1), row as i32, column as i32),
                                get_index((p_x as i32, p_y as i32 + 1), row as i32, column as i32),
                                get_index(
                                    (p_x as i32 + 1, p_y as i32 - 1),
                                    row as i32,
                                    column as i32,
                                ),
                                get_index((p_x as i32 + 1, p_y as i32), row as i32, column as i32),
                                get_index(
                                    (p_x as i32 + 1, p_y as i32 + 1),
                                    row as i32,
                                    column as i32,
                                ),
                            ];
                            let mut count: u32 = 0;
                            for i in v.iter() {
                                match i {
                                    None => (),
                                    Some(k) => {
                                        if vector_minefield[*k] == '*' {
                                            count += 1
                                        }
                                    }
                                }
                            }
                            match count {
                                0 => ' '.to_string(),
                                _ => format!("{}", count),
                            }
                        }
                    };
                    s.push_str(&value);
                    if k % column == column - 1 {
                        result.push(s.clone())
                    }
                }
                result
            }
        },
    }
}
