fn main() {
    println!("{}", fib(1));
    println!("{}", fib(3));
    println!("{}", fib(5));
}

fn fib(n: u32) -> u32 {
    if n == 1 {
        return 0;
    }

    let mut fib_numbers = [0, 1];

    for _ in 1..n - 1 {
        let next = fib_numbers[0] + fib_numbers[1];
        fib_numbers[0] = fib_numbers[1];
        fib_numbers[1] = next;
    }

    fib_numbers[1]
}
