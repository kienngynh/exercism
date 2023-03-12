#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}
fn calculator_input(s: &str) -> Vec<CalculatorInput> {
    s.split_whitespace()
        .map(|s| match s {
            "+" => CalculatorInput::Add,
            "-" => CalculatorInput::Subtract,
            "*" => CalculatorInput::Multiply,
            "/" => CalculatorInput::Divide,
            n => CalculatorInput::Value(n.parse().unwrap()),
        })
        .collect()
}
fn main () {
    let mut inputs = calculator_input("4 8 + 7 5 - /");
    println!("{:?}",inputs.pop())
}