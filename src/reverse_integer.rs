//-2147483648 <= x <= 2147483647
pub fn reverse(x: i32) -> i32 {
    if x <= 9 && x >= -9 {
        return x;
    }
    let max_lenght = if x < 0 { 11 } else { 10 };

    let x_chars = x.to_string().chars().rev().collect::<Vec<char>>();
    let length = x_chars.len() as i32;

    if length > max_lenght {
        return 0;
    } else {
        if length == max_lenght {
            // length == 10 or 11 if x < 0 (last will be the '-' sign)
            if x_chars[0] > '2' {
                return 0;
            }
        }
    }

    let mut rev_str = x_chars.iter().collect::<String>();

    if x < 0 {
        // remove sign from the end
        rev_str.remove(rev_str.len() - 1);
        match rev_str.parse::<i32>() {
            Ok(v) => return v * -1,
            _ => return 0,
        }
    }

    match rev_str.parse::<i32>() {
        Ok(v) => return v,
        _ => return 0,
    }
}
