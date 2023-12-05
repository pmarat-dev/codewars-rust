// Count of positives / sum of negatives
// https://www.codewars.com/kata/576bb71bbbcf0951d5000044

#[allow(dead_code)]
fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return Vec::new();
    }

    vec![
        input.iter().filter(|&&x| x > 0).count() as i32,
        input.iter().filter(|&&x| x < 0).sum()
    ]
}

#[allow(dead_code)]
pub fn count_positives_sum_negatives_test() {
    println!("{:?}", count_positives_sum_negatives(vec![1, 2, -4, -6, 6]));
}