use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter first number:");

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num1: i32 = input.trim().parse().expect("Please enter a valid number");
    input.clear();

    println!("Enter operator:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let operator: char = input.trim().chars().next().expect("Please enter a valid operator");
    input.clear();

    println!("Enter second number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num2: i32 = input.trim().parse().expect("Please enter a valid number");
    input.clear();

    let result = match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => {
            if num2 == 0 {
                println!("Error: Division by zero");
                return;
            } else {
                num1 / num2
            }
        }
        _ => {
            println!("Error: Invalid operator");
            return;
        }
    };

    println!("Result: {}", result);
}
