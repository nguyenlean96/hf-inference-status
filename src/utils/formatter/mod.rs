pub fn format_with_commas(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::with_capacity(s.len() + s.len() / 3);

    for (i, c) in s.chars().rev().enumerate() {
        if i % 3 == 0 && i > 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}
