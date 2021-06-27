const DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const GIFTS: [&str; 12] = [
    "a patridge in a pear tree.",
    "Two turtle doves,",
    "Three french hens,",
    "Four calling birds,",
    "FIVE GOOOO-OOOLLLLD RIIIIIIINGS,",
    "Six geese a-laying,",
    "Seven swans a-swimming,",
    "Eight maids a-milking,",
    "Nine ladies dancing,",
    "Ten lords a-leaping,",
    "Eleven pipers piping,",
    "Twelve drummers drumming,",
];

fn main() {
    for i in 0..12 {
        println!(
            "On the {} day of Christmas, my true love gave to me",
            DAYS[i]
        );

        for j in (1..=i).rev() {
            println!("{}", GIFTS[j])
        }

        if i == 0 {
            println!("{}", GIFTS[0])
        } else {
            println!("and {}", GIFTS[0])
        }

        println!()
    }
}
