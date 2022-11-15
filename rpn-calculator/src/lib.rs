#[derive(Debug, Copy, Clone)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

impl CalculatorInput {
    fn operator(self) -> fn(i32, i32) -> i32 {
        match self {
            Self::Add => |a, b| a + b,
            Self::Subtract => |a, b| a - b,
            Self::Multiply => |a, b| a * b,
            Self::Divide => |a, b| a / b,
            Self::Value(_) => panic!(),
        }
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    if inputs.is_empty() {
        return None;
    }

    let mut stack = Vec::<i32>::new();
    for input in inputs {
        if let CalculatorInput::Value(val) = input {
            stack.push(*val);
        } else {
            let arg2 = stack.pop()?;
            let arg1 = stack.pop()?;
            let operator = input.operator();
            stack.push(operator(arg1, arg2));
        }
    }

    if stack.len() == 1 {
        Some(stack[0])
    } else {
        None
    }
}
