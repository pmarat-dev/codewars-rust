#[allow(dead_code)]
fn add_binary(a: u64, b: u64) -> String {
    format!("{:b}", a+b)
}

#[allow(dead_code)]
pub fn add_binary_test() {
    println!("{}", add_binary(8, 7));
}