pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();

    for c in string.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if !paired(&mut stack, '(') {
                    return false;
                }
            }
            ']' => {
                if !paired(&mut stack, '[') {
                    return false;
                }
            }
            '}' => {
                if !paired(&mut stack, '{') {
                    return false;
                }
            }
            _ => {}
        }
    }
    if stack.len() != 0 {
        return false;
    }
    true
}

fn paired(stack: &mut Vec<char>, right: char) -> bool {
    stack.pop().unwrap_or(' ') == right
}
