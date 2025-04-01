pub fn tokenize(input: &str) -> Option<Vec<String>> {
    let mut tokens: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut nest: usize = 0;

    for c in input.chars() {
        match c {
            '(' => {
                current.push(c);
                nest += 1;
            }
            ')' => {
                current.push(c);
                if nest != 0 {
                    nest -= 1;
                } else {
                    return None;
                }
            }
            ' ' => {
                if nest == 0 {
                    tokens.push(current.clone());
                    current.clear();
                } else {
                    current.push(c)
                }
            }
            _ => current.push(c),
        }
    }

    // Syntax error check
    if nest != 0 {
        return None;
    }
    if !current.is_empty() {
        tokens.push(current.clone());
    }
    Some(tokens)
}
