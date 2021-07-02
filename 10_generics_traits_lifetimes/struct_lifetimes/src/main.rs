// An instance of ImportantExcerpt can't outlive the reference it holds in its
// `part` field.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // No need to annotate the lifetime of the reference to self because there
    // is only one input parameter (and hence one input lifetime, 'a), and the
    // return value is not a reference.
    fn level(&self) -> i32 {
        3
    }

    // No need to annoate the lifetimes because, although the parameters are
    // annotated with two lifetimes, 'a and 'b, the return value of a method is
    // always given the lifetime of self.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
