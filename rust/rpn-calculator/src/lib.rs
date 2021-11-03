#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];
    for input in inputs {
        match input {
            CalculatorInput::Add => {
                let top1 = stack.pop()?;
                let top2 = stack.pop()?;
                stack.push(top1 + top2);
            }
            CalculatorInput::Subtract => {
                let top1 = stack.pop()?;
                let top2 = stack.pop()?;
                stack.push(top2 - top1);
            }
            CalculatorInput::Multiply => {
                let top1 = stack.pop()?;
                let top2 = stack.pop()?;
                stack.push(top1 * top2);
            }
            CalculatorInput::Divide => {
                let top1 = stack.pop()?;
                let top2 = stack.pop()?;
                stack.push(top2 / top1);
            }
            CalculatorInput::Value(val) => stack.push(*val),
        }
    }
    if stack.len() == 1 {
        return stack.pop();
    }
    None
}
