// Find the smallest integer in the array
// https://www.codewars.com/kata/55a2d7ebe362935a210000b2

#[allow(dead_code)]
fn find_smallest_int(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}

#[allow(dead_code)]
pub fn find_smallest_int_test() {
    println!("{:?}", find_smallest_int(&vec![1, 3, -2, 5, 6]));
}
