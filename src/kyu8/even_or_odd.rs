#[allow(dead_code)]
fn even_or_odd(i: i32) -> &'static str {
    match i % 2 {
        0 => "Even",
        _ => "Odd",
    }
}

#[allow(dead_code)]
pub fn even_or_odd_test() {
    println!("{:?}", even_or_odd(6));
    println!("{:?}", even_or_odd(7));
}