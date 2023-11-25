pub enum Operation {
    Add,
    Substract,
    Multiplication,
    Divide
}



pub fn calculate (operation: Operation, num1: f64, num2: f64) -> Option<f64> {
    match operation {
        Operation::Add => Some(num1 + num2),
        Operation::Substract => Some(num1 - num2),
        Operation::Multiplication => Some(num1 * num2),
        Operation::Divide => Some(num1/num2),
    }
}