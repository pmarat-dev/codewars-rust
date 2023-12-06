// Vowel Count
// https://www.codewars.com/kata/54ff3102c1bad923760001f3

#[allow(dead_code)]
fn get_count(string: &str) -> usize {
    string
        .chars()
        .filter(|&c| "aeiou".contains(c))
        .count()
}

#[allow(dead_code)]
pub fn get_count_test() {
    println!("{}", get_count("Hello World out there!"));
}
