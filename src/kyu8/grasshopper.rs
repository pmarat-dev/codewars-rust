// Grasshopper - Terminal game move function
// https://www.codewars.com/kata/563a631f7cbbc236cf0000c2

#[allow(dead_code)]
fn move_hero(position: u32, roll: u32) -> u32 {
    return position + 2 * roll;
}

#[allow(dead_code)]
pub fn move_hero_test() {
    println!("{:?}", move_hero(3, 6));
}