/*
Runtime: 4 ms, faster than 63.11% of Rust online submissions for Roman to Integer.
Memory Usage: 2 MB, less than 87.70% of Rust online submissions for Roman to Integer.
*/
pub fn roman_to_int(s: String) -> i32 {
    let mut result= 0;
    let mut ignore_next = false;
    for (i, &c) in s.as_bytes().iter().enumerate() {
        if ignore_next { 
            ignore_next = false;
            continue;
        }
        match &c {
            b'I' => {
                if i < s.len() - 1 {
                    match s.as_bytes()[i+1] {
                        b'V' => {
                            result += 4;
                            ignore_next = true;
                        },
                        b'X' => {
                            result += 9;
                            ignore_next = true;
                        }, 
                        _ => result += 1
                    }
                } else {
                    result += 1;
                }
            },
            b'V' => result += 5,
            b'X' => {
                if i < s.len() - 1 {
                    match s.as_bytes()[i+1] {
                        b'L' => {
                            result += 40;
                            ignore_next = true;
                        }, 
                        b'C' => {
                            result += 90;
                            ignore_next = true;
                        }, 
                        _ => result += 10
                    }
                } else {
                    result += 10;
                }
            },
            b'L' => result += 50,
            b'C' => {
                if i < s.len() - 1 {
                    match s.as_bytes()[i+1] {
                        b'D' => {
                            result += 400;
                            ignore_next = true;
                        }, 
                        b'M' => {
                            result += 900;
                            ignore_next = true;
                        }, 
                        _ => result += 100
                    }
                } else {
                    result += 100;
                }
            },
            b'D' => result += 500,
            b'M' => result += 1000,
            _ => ()
        }
    }
    result
}