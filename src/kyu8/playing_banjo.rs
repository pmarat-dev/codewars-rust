// Are You Playing Banjo?
// https://www.codewars.com/kata/53af2b8861023f1d88000832

#[allow(dead_code)]
fn are_you_playing_banjo(name: &str) -> String {
    match name.to_uppercase().starts_with("R") {
        true => format!("{} plays banjo", name),
        false => format!("{} does not play banjo", name)
    }
}

#[allow(dead_code)]
pub fn are_you_playing_banjo_test() {
    println!("{}", are_you_playing_banjo("Robert"));
}
