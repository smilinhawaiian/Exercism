use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut is_isogram = true;
    let mut table: HashSet<char> = HashSet::new();
    for letter in candidate.to_lowercase().chars() {
        if !table.contains(&letter) {
            if letter != ' ' && letter != '-' {
                table.insert(letter);
            }
        } else {
            is_isogram = false;
        }
    }
    is_isogram
}
