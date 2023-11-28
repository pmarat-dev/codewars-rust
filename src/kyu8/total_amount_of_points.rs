// Total amount of points
// https://www.codewars.com/kata/5bb904724c47249b10000131

use std::cmp::Ordering;

#[allow(dead_code)]
fn total_amount_of_points(games: &[&str]) -> u32 {
    games.iter().map(|game| {
        let (scored, conceded) = game.split_once(':').unwrap();
        match scored.cmp(conceded) {
            Ordering::Less => 0,
            Ordering::Equal => 1,
            Ordering::Greater => 3,
        }
    }).sum()
}

#[allow(dead_code)]
pub fn total_amount_of_points_test() {
    println!("{}", total_amount_of_points(&["1:0", "0:0", "0:2", "3:1"]));
}
