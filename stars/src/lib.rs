pub fn stars(n: u32) -> String {
    let end = n*2;
    let mut res = String::new();

    for _ in 1..=end {
        res += "*";
    }

    res
}