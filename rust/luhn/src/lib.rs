pub fn is_valid(code: &str) -> bool {
    let (mut e, mut o) = (vec![], vec![]);
    let mut count_e: u32 = 0;
    let mut count_o: u32 = 0;
    let mut count: u32 = 0;
    for c in code.chars() {
        if c.is_digit(10) {
            if count % 2 == 0 {
                let d = c.to_digit(10).unwrap();
                if d > 4 {
                    count_e += 1
                };
                e.push(d);
            } else {
                let d = c.to_digit(10).unwrap();
                if d > 4 {
                    count_o += 1
                };
                o.push(d);
            }
            count += 1
        } else if c == ' ' {
            continue;
        } else {
            return false;
        }
    }
    let sum_e: u32 = e.iter().sum();
    let sum_o: u32 = o.iter().sum();
    let l = e.len() + o.len();
    if l > 1 {
        match l % 2 {
            0 => (sum_e * 2 + sum_o - count_e * 9) % 10 == 0,
            _ => (sum_o * 2 + sum_e - count_o * 9) % 10 == 0,
        }
    } else {
        return false;
    }
}
