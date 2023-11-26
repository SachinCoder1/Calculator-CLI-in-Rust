pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub fn calculate(operation: Operation, num1: f64, num2: f64) -> Option<f64> {
    match operation {
        Operation::Add => Some(num1 + num2),
        Operation::Subtract => Some(num1 - num2),
        Operation::Multiply => Some(num1 * num2),
        Operation::Divide => {
            if num2 == 0.0 {
                None
            } else {
                Some(num1 / num2)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(calculate(Operation::Add, 5.0, 3.0), Some(8.0));
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(calculate(Operation::Subtract, 20.0, 10.0), Some(10.0));
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(calculate(Operation::Multiply, 5.0, 3.0), Some(15.0));
    }

    #[test]
    fn test_divide() {
        assert_eq!(calculate(Operation::Divide, 4.0, 2.0), Some(2.0));
    }
}
