pub fn divide(x: i32, y: i32) -> (i32, i32) {
    if y == 0 {
        panic!("ERROR: attempt to divide by zero");
    }

    let div = x/y;
    let rem = x %y;
    (div, rem)
}
