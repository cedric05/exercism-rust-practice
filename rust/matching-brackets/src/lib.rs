use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = vec![];
    let mut balanced = true;
    let char_map = HashMap::from([(')', '('), (']', '['), ('}', '{')]);
    for i in string.chars() {
        match i {
            '(' | '[' | '{' => stack.push(i),
            ')' | '}' | ']' => {
                let get = *char_map.get(&i).unwrap();
                if let Some(has) = stack.pop() {
                    if has != get {
                        balanced = false;
                        break;
                    }
                } else {
                    balanced = false;
                    break;
                }
            }
            _ => {}
        }
    }
    stack.is_empty() && balanced
}
