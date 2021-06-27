use std::io;

fn main() {
    let unit = get_unit();
    let temp = get_temp();

    if unit == "F" {
        println!("{}C is {:.2}F", temp, to_fahrenheit(temp))
    } else {
        println!("{}F is {:.2}C", temp, to_celsius(temp))
    }
}

fn get_unit() -> String {
    loop {
        println!("Convert to F or C?");

        let mut unit = String::new();
        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read from stdin");

        let unit = unit.trim().to_string();
        if unit == "F" || unit == "C" {
            break unit;
        }
    }
}

fn get_temp() -> f64 {
    loop {
        println!("Enter temperature:");

        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read from stdin");

        let temp: f64 = match temp.trim().parse() {
            Ok(t) => t,
            Err(_) => continue,
        };

        break temp;
    }
}

fn to_fahrenheit(c: f64) -> f64 {
    c.mul_add(9f64 / 5f64, 32f64)
}

fn to_celsius(f: f64) -> f64 {
    (f - 32f64) * 5f64 / 9f64
}
