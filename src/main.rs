use calculator_cli::{calculate, Operation};
use dialoguer::{theme::ColorfulTheme, Select};
use std::io;

const OPERATIONS: [&str; 4] = ["Add", "Subtract", "Multiply", "Divide"];
fn main() {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick an operation")
        .default(0)
        .items(&OPERATIONS[..])
        .interact()
        .unwrap();

    println!("selected value: {}", selection);

    let operation = match OPERATIONS[selection] {
        "Add" => Operation::Add,
        "Subtract" => Operation::Subtract,
        "Multiply" => Operation::Multiply,
        "Divide" => Operation::Divide,
        _ => {
            panic!("Not Selected Any Value! Please Try Again");
        }
    };

    let first_number = read_number("Enter First Number:");
    let second_number = read_number("Enter Second Number");

    let calculated_value = calculate(operation, first_number, second_number);

    match calculated_value {
        Some(num) => println!("= {}", num),
        None => println!("Failed to get result!"),
    }
}

fn read_number(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input! Please try agaub");
    let parsed_number: f64 = input
        .trim()
        .parse()
        .expect("Failed to Parse! Please Enter Valid Number");
    parsed_number
}
