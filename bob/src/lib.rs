// RESPONSES:
// 0. No Statement == "Fine. Be that way!" //nada
// 1. CAPS Question == "Calm down, I know what I'm doing!" //upper && Q
// 2. CAPS Statement == "Whoa, chill out!" // upper NO Q
// 3. Regular Question == "Sure." //lower && Q
// 4. Regular Statement == "Whatever." //lower NO Q
pub fn reply(message: &str) -> &str {
    let mut question = false;
    let mut upper = false;
    let mut lower = false;
    let mut number = false;

    for ch in message[..].as_bytes() {
        match ch {
            63 => question = true,
            48...57 => {
                number = true;
                question = false;
            }
            65...90 => {
                upper = true;
                question = false;
            }
            97...122 => {
                lower = true;
                question = false;
            }
            _ => continue,
        }
    }

    if question {
        if !lower && upper {
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        }
    } else if !lower && upper {
        "Whoa, chill out!"
    } else if !lower && !upper && !number {
        "Fine. Be that way!"
    } else {
        "Whatever."
    }
}
