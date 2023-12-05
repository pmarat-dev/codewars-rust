// Shortest Word
// https://www.codewars.com/kata/57cebe1dc6fdc20c57000ac9

#[allow(dead_code)]
fn find_short(s: &str) -> u32 {
    s.split_whitespace()
        .map(|word| word.len())
        .min()
        .unwrap_or(0) as u32
}

#[allow(dead_code)]
pub fn find_short_test() {
    println!("{}", find_short("Hello World out there!"));
}
