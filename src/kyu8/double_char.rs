// Double Char
// https://www.codewars.com/kata/56b1f01c247c01db92000076

#[allow(dead_code)]
fn double_char(s: &str) -> String {
    s.chars().flat_map(|c| [c, c]).collect()
}

#[allow(dead_code)]
pub fn double_char_test() {
    println!("{:?}", double_char("Hello"));
}