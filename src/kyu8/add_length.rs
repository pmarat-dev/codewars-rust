// https://www.codewars.com/kata/559d2284b5bb6799e9000047
// Add Length

#[allow(dead_code)]
fn add_length(str: &str) -> Vec<String> {
    str.split_whitespace()
        .map(|v| format!("{} {}", v, v.len()))
        .collect()
}

#[allow(dead_code)]
pub fn add_length_test() {
    println!("{:?}", add_length("apple ban"));
}
