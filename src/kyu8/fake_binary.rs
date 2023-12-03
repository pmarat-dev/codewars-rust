// Fake Binary
// https://www.codewars.com/kata/57eae65a4321032ce000002d

#[allow(dead_code)]
fn fake_bin(s: &str) -> String {
    s.chars().map(|c| if c < '5' {'0'} else {'1'}).collect()
}

#[allow(dead_code)]
pub fn fake_bin_test() {
    println!("{:?}", fake_bin("123456789"));
}