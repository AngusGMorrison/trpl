fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    // Also works on string literals directly
    let s = "initial contents".to_string();
    let s = String::from("initial_contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // Add takes ownership of self (s1), appends a copy of the contents of &s2,
    // and returns the result. s1 can no longer be used.
    // String is automatically coerced into &str.
    let s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // format! does not take ownership of its parameters.
    let s = format!("{}-{}-{}", s1, s2, s3);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // Getting grapheme clusters (e.g. letters with diacritics) from strings is
    // not provided by the standard library.
}
