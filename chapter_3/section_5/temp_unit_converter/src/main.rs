use std::io;

fn get_unit() -> String {
    loop {
        let mut input = String::new();
        println!("Please enter the temperature unit (F/C).");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "F" || input.trim() == "C" {
            break input;
        }
    }
}

fn get_temp(unit: &str) -> i32 {
    loop {
        let mut input = String::new();
        println!("Please enter the temperature in {unit} degrees.");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let _input: i32 = match input.trim().parse() {
            Ok(num) => {
                break num;
            }
            Err(_) => {
                println!("make sure your entry is a whole number");
                continue;
            }
        };
    }
}

fn main() {
    let result = get_unit();
    let unit = result.trim();

    let temp = get_temp(unit);

    let new_temp = if unit == "F" {
        ((temp - 32) * 5) / 9
    } else {
        ((temp / 5) * 9) + 32
    };

    let new_unit = if unit == "F" { "C" } else { "F" };

    println!("{temp} {unit}° converts to {new_temp} {new_unit}°");
}

// °C to °F  Divide by 5, then multiply by 9, then add 32
// °F to °C  Deduct 32, then multiply by 5, then divide by 9
