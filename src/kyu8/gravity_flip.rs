// Gravity Flip
// https://www.codewars.com/kata/5f70c883e10f9e0001c89673

#[allow(dead_code)]
fn flip(dir: char, cubes: &[u32]) -> Vec<u32> {
    let mut cube_vec: Vec<u32> = cubes.to_vec();
    cube_vec.sort();
    if dir == 'L' {
        cube_vec.reverse();
    }
    return cube_vec;
}

#[allow(dead_code)]
pub fn flip_test() {
    println!("{:?}", flip('L', &vec![3, 2, 1, 2]));
}