use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // Owned values are moved, making the hash map the owner. Copied values are not.
    let field_name = String::from("Favourite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point. println!("{}",
    // field_value); If you insert references, the values won't be moved, but
    // the values the references point to must be valid for at least as long as
    // the hash map is valid.

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value)
    }

    // Overwriting a value
    scores.insert(String::from("Blue"), 25);

    // Inserting a value if the key has no value
    scores.entry(String::from("Blue")).or_insert(10);
    scores.entry(String::from("Red")).or_insert(50);

    // Updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1
    }
}
