// Reverse Words
// https://www.codewars.com/kata/5259b20d6021e9e14c0010d4

#[allow(dead_code)]
fn reverse_words(str: &str) -> String {
    str.to_string().split(" ")
        .map(|word| word.chars().rev().collect::<String>())
        .collect::<Vec<String>>().join(" ")
}

#[allow(dead_code)]
pub fn reverse_words_test() {
    println!("{}", reverse_words("Hello World out there!"));
}
