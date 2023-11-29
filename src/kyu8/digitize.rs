// Convert number to reversed array of digits
// https://www.codewars.com/kata/5583090cbe83f4fd8c000051

#[allow(dead_code)]
fn digitize(number: u64) -> Vec<u8> {
    number
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .rev()
        .collect::<Vec<u8>>()
}

#[allow(dead_code)]
pub fn digitize_test() {
    println!("{:?}", digitize(123));
}