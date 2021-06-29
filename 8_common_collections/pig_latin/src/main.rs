use regex::Regex;

fn main() {
    let input = "I like apples and coding";
    println!("{}", pig_latinize(input));
}

fn pig_latinize(phrase: &str) -> String {
    phrase
        .split_whitespace()
        .map(pig_latinize_word)
        .collect::<Vec<String>>()
        .join(" ")
}

fn pig_latinize_word(word: &str) -> String {
    let mut latinized = String::from(word);

    let starts_with_vowel = Regex::new(r"^[aeiouAEIOU]").unwrap();
    if starts_with_vowel.is_match(word) {
        latinized.push_str("hay");
        return latinized;
    }

    let first_char = match word.chars().nth(0) {
        Some(ch) => ch.to_uppercase().to_string(),
        None => return String::from(""),
    };
    latinized.push_str(&first_char);
    latinized.push_str("ay");
    String::from(&latinized[1..])
}
