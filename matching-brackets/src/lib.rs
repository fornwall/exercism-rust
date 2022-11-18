fn opening(closing: char) -> char {
    match closing {
        ']' => '[',
        ')' => '(',
        '}' => '{',
        _ => {
            panic!("Not a bracket: {}", closing);
        }
    }
}

#[must_use]
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for c in string.chars() {
        match c {
            '(' | '[' | '{' => {
                stack.push(c);
            }
            ')' | ']' | '}' => {
                let opening_char = opening(c);
                if stack.pop() != Some(opening_char) {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}
