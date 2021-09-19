pub fn get_sum(a: i32, b: i32) -> i32 {
    let mut y = b;
    let mut x = a;

    while y != 0 {
        let carry = x & y;
        x = x ^ y;
        y = carry << 1;
    }

    x
}