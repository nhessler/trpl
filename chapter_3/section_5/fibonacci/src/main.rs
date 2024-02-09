use std::io;

fn get_ordinal_indicator(point: u32, divisor: u32) -> String {
    let indicator = if point % divisor == 1 {
        "st"
    } else if point % divisor == 2 {
        "nd"
    } else if point % divisor == 3 {
        "rd"
    } else {
        "th"
    };

    indicator.to_string()
}

fn fibonacci(point: u32) -> u64 {
    let mut p = point;
    let mut a = 0;
    let mut b = 1;

    while p > 1 {
        let sum = a + b;
        a = b;
        b = sum;

        p -= 1;
    }

    return b;
}

fn main() {
    loop {
        let mut nth_number = String::new();

        println!("Please input the point in the fibonacci sequence you would like to calculate (enter 0 to quit)");

        io::stdin()
            .read_line(&mut nth_number)
            .expect("Failed to read line");

        let nth_number: u32 = match nth_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if nth_number == 0 {
            break;
        }

        let ordinal_indicator = if nth_number < 20u32 {
            get_ordinal_indicator(nth_number, 20u32)
        } else {
            get_ordinal_indicator(nth_number, 10u32)
        };

        let fib_at_point = fibonacci(nth_number);
        println!("the {nth_number}{ordinal_indicator} in the fibonacci sequense is {fib_at_point} ")
    }
}
