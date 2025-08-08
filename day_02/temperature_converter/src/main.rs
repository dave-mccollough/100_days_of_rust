use std::io;

fn main() {
    println!("🌡️ Temperature Converter");
    println!("Press 1 to convert Celsius to Fahrenheit");
    println!("Press 2 to convert Fahrenheit to Celsius");
    println!("Press 3 to convert Celsius to Kelvin");
    println!("press 4 to convert Fahrenheit to Kelvin");
    println!("Please select an option (1, 2, 3, or 4)");

    // create mutable variable - type string
    let mut choice = String::new();

    // Access standard input stream
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    // Convert choice to U32 
    // .trim removes leading or trailing white spaces
    // .parse converts into U32
    // match is pattern matching
    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ Invalid Choice, Please select 1, 2, 3, or 4");
            return;
        }
    };

    if choice == 1 {
        celsius_to_fahrenheit();
    } else if choice == 2 {
        fahrenhit_to_celsisus();
    } else if choice == 3 {
        celsisus_to_kelvin();
    } else if choice == 4 {
        fahrenheit_to_kelvin();
    } else {
        println!("❌ Invalid choice. Please select 1, 2, 3, or 4");
    }
}

fn celsius_to_fahrenheit(){
    println!("Enter temperature in Celsius");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ Invalid input. Please enter a valid number.");
            return;
        }
    };

    let fahrenheit = (temp * 9.0 / 5.0) + 32.0;
    println!("{:.2}°C is {:2}°F", temp, fahrenheit);
}

fn fahrenhit_to_celsisus(){
    println!("Enter temperature in Fahrenheit");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    let celsisus = (temp - 32.0) * 5.0 / 9.0;
    println!("{:2}°F is {:2}°C", temp, celsisus);
}

fn celsisus_to_kelvin() {
    println!("Enter termperature in Celsius");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    let kelvin = temp + 273.15;
    println!("{:2}°C is {:2} K", temp, kelvin);
}

fn fahrenheit_to_kelvin() {
    println!("Enter temperature in Fahrenheit");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number");
            return;
        }
    };

    let kelvin = (temp - 32.0) * 5.0 / 9.0 + 273.15;
    println!("{:2}°F is {:2} K", temp, kelvin);
}