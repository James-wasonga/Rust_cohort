 use std::io;

fn main() {
    println!("Welcome to the Rust Calculator!");

    // Prompt user for input
    println!("Please enter the first number:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1)
        .expect("Failed to read line");
    let num1: f64 = num1.trim().parse()
        .expect("Please enter a valid number!");

    println!("Please enter the second number:");
    
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).
    expect("Failed to read line");
    let num2: f64 = num2.trim().parse()
    .expect("Please enter valid number");


    // Prompt user for operation
    println!("Please choose the operation (+, -, *, /):");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator)
        .expect("Failed to read line");
    let operator = operator.trim();

    // Perform calculation based on the operator
    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Error: Division by zero!");
                return;
            }
            num1 / num2
        },
        _ => {
            println!("Invalid operator!");
            return;
        }
    };

    // Display the result
    println!("Result: {}", result);
}

