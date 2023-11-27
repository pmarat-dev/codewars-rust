// https://www.codewars.com/kata/544675c6f971f7399a000e79
// Convert a String to a Number!

#[allow(dead_code)]
pub fn string_to_number(s: &str) -> i32 {
    return s.parse().unwrap();
}

// test:
// println!("{}", string_to_number("123"));
