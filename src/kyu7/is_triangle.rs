#[allow(dead_code)]
fn is_triangle(a: i64, b: i64, c: i64) -> bool {
    a + b > c && a + c > b && b + c > a
}

#[allow(dead_code)]
pub fn is_triangle_test() {
    println!("{}", is_triangle(2, 5, 6));
}
