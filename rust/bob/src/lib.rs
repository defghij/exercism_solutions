pub fn reply(message: &str) -> &str {
    let m: &str = message.trim();
    if m.len() == 0 {
        return "Fine. Be that way!";
    }

    let punc = m.get((m.len() - 1)..(m.len())).unwrap();

    match punc {
        "?" => {
            if is_yell(m) && contains_letters(m) {
                "Calm down, I know what I'm doing!"
            } else {
                "Sure."
            }
        }
        _ => {
            if is_yell(m) && contains_letters(m) {
                "Whoa, chill out!"
            } else {
                "Whatever."
            }
        }
    }
}

fn is_yell(message: &str) -> bool {
    for c in message.chars() {
        if c.is_ascii() && c.is_ascii_lowercase() {
            return false;
        }
    }
    true
}

fn contains_letters(message: &str) -> bool {
    for c in message.chars() {
        if c.is_ascii_alphabetic() {
            return true;
        }
    }
    false
}
