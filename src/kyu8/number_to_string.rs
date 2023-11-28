// Convert a Number to a String!
// https://www.codewars.com/kata/5265326f5fda8eb1160004c8

#[allow(dead_code)]
fn number_to_string(i: i32) -> String {
    return i.to_string();
}

#[allow(dead_code)]
pub fn number_to_string_test() {
    println!("{}", number_to_string(123));
}
