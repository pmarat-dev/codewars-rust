// Gravity Flip
// https://www.codewars.com/kata/5f70c883e10f9e0001c89673

#[allow(dead_code)]
fn flip(dir: char, cubes: &[u32]) -> Vec<u32> {
    let mut vector: Vec<u32> = cubes.to_vec();
    vector.sort();
    if dir == 'L' {
        vector.reverse();
    }
    return vector;
}

#[allow(dead_code)]
pub fn flip_test() {
    println!("{:?}", flip('L', &vec![3, 2, 1, 2]));
}