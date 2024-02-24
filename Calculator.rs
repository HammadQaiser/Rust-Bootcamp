enum Operation 
{
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> f64 
{

    match op 
      {
        Operation::Add(x, y) => x + y,
        Operation::Subtract(x, y) => x - y,
        Operation::Multiply(x, y) => x * y,
        Operation::Divide(x, y) => x / y,
    }
}

fn main() 
{
    use std::io::{self, Write};
    
    println!("\t\t\t Welcome to Simple Calculator ");

    print!("Enter the first number: ");
    io::stdout().flush().unwrap();
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).unwrap();
    let num1: f64 = num1.trim().parse().unwrap();

    print!("Enter the operation (+, -, *, /): ");
    io::stdout().flush().unwrap();
    let mut op = String::new();
    io::stdin().read_line(&mut op).unwrap();
    let op = op.trim();
    
    print!("Enter the second number: ");
    io::stdout().flush().unwrap();
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).unwrap();
    let num2: f64 = num2.trim().parse().unwrap();


    let operation = match op 
    {
        "+" => Operation::Add(num1, num2),
        "-" => Operation::Subtract(num1, num2),
        "*" => Operation::Multiply(num1, num2),
        "/" => Operation::Divide(num1, num2),
        _ => panic!("Invalid operation"),
    };

 
    let result = calculate(operation);

    println!("The result of {} {} {} is: {}", num1, op, num2, result);
}

