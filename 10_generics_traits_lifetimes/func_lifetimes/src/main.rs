fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// All references in the parameters must live at least as long as 'a. The
// lifetime of the reference returned is therefore the same as the smaller of
// the lifetimes of the references passed in. I.e. the concrete lifetime that is
// substituted for 'a is the part of the scope of x that overlaps with the scope
// of y.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
