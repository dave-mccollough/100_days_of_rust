use std::io;

fn main(){
    println!("THis is a simple calculator");
    println!("Available operations include add, subtract, multiply, and divide");
    println!("Enter you calculation (e.g. 5 + 3)");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

    if tokens.len() != 3 {
        println!("Invalid input. Please use the following format: number operatior number.");
        return;
    }

    let num_one: f64 = match tokens[0].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid first number.");
            return;
        }
    };

    let operator = tokens[1];

    let num_two: f64 = match tokens[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid second number.");
            return;
        }
    };

    let result = match operator {
        "+" => add(num_one, num_two),
        "-" => subtract(num_one, num_two),
        "*" => multiply(num_one, num_two),
        "/" => divide(num_one, num_two),
        _ => {
            println!("Invalid operator, please use: +, -, *, or / operators");
            return;
        }
    };

    println!("Result is: {:.2}", result);

}

fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        println!{"Division by zero is not allowed"};
        std::process::exit(1);
    }
    a / b
}