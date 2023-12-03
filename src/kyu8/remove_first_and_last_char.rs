// Remove First and Last Character
// https://www.codewars.com/kata/56bc28ad5bdaeb48760009b0

#[allow(dead_code)]
pub fn remove_char(s: &str) -> String {
    s[1..s.len() - 1].to_string()
}

#[allow(dead_code)]
pub fn remove_char_test() {
    println!("{}", remove_char("Hello World"));
}