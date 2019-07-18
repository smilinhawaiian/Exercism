pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();
    // get the number of words
    let prov_len = list.len();
    // make sure there is a list
    if prov_len > 0 {
        let cause = "And all for the want of a ".to_string() + list[0] + ".";
        for current in 1..prov_len {
            let effect = "For want of a ".to_string()
                + list[current - 1]
                + " the "
                + list[current]
                + " was lost.\n";
            proverb += &effect;
        }
        proverb += &cause;
    }
    proverb
}
