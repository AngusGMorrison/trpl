fn main() {
    let s = String::from("Hello world!");
    let word = first_word(&s);
    println!("the first word is: {}", word);

    let literal = "Hello world!";
    let word = first_word(literal);
    println!("the first word is: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}
