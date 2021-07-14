fn main() {
    println!("Hello, world!");
}

struct Point {
    x: i32,
    y: i32,
}

fn destructure_structs() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x: 0, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destructure_enums() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quite variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {} and blue {}", r, g, b)
        }
    }
}

enum MultiColor {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum MultiColorMessage {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(MultiColor),
}

fn destructure_nested_structs_and_enums() {
    let msg = MultiColorMessage::ChangeColor(MultiColor::Hsv(0, 160, 255));

    match msg {
        MultiColorMessage::ChangeColor(MultiColor::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {} and blue {}", r, g, b)
        }
        MultiColorMessage::ChangeColor(MultiColor::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {} and value {}",
            h, s, v
        ),
        _ => (),
    }
}

struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

fn ignore_remaining_values() {
    let origin = Point3 { x: 0, y: 0, z: 0 };

    match origin {
        Point3 { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}

fn match_guard() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

enum Greeting {
    Hello { id: i32 },
}

fn at_bindings() {
    let greeting = Greeting::Hello { id: 5 };

    match greeting {
        Greeting::Hello { id: id @ 3..=7 } => println!("Found an id in range: {}", id),
        Greeting::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Greeting::Hello { id } => println!("Found some other id: {}", id),
    }
}
