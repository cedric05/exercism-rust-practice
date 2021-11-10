use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = vec![];
    let char_map = HashMap::from([(')', '('), (']', '['), ('}', '{')]);
    for i in string.chars() {
        match i {
            '(' | '[' | '{' => stack.push(i),
            ')' | '}' | ']' => match stack.pop() {
                Some(available) if available == *char_map.get(&i).unwrap() => {}
                _ => {
                    return false;
                }
            },
            _ => {}
        }
    }
    stack.is_empty()
}
