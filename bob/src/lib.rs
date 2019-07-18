//Bob answers 'Sure.' if you ask him a question, such as "How are you?".
//He answers 'Whoa, chill out!' if you YELL AT HIM (in all capitals).
//He answers 'Calm down, I know what I'm doing!' if you yell a question at him.
//He says 'Fine. Be that way!' if you address him without actually saying anything.
//He answers 'Whatever.' to anything else.

pub fn reply(message: &str) -> &str {
    // CASES:
    // ignore anything that isn't a capital or lowercase letter
    // check last char for question mark
    // check if all chars are capital

    // RESPONSES: 
    // 0. No Statement == "Fine. Be that way!"
    // 1. Regular Statement == "Whatever."
    // 2. Regular Question == "Sure."
    // 3. CAPS Statement == "Whoa, chill out!"
    // 4. CAPS Question == "Calm down, I know what I'm doing!"
    //
    //
    //
    let mut last_char = ' '; //check for question mark
    let mut yelling = true; // check for all caps
    let mut statement = false; // check for words said
    let mut response = "Whatever.";

    //for ch in message[..].chars() {
    for ch in message[..].as_bytes() {
        match ch {
            63 => println!("63: {:?}", ch),//last_char = '?',
            65...90 => println!("65-90: {:?}", ch),
            97...122 => println!("97-122: {:?}", ch),
            _ => {println!("everything else .. {:?}", ch)},
        }
        if yelling == true{
            yelling = false;
        }
        if statement == false {
            statement = true;
        }
        println!("{:?}\n", ch);
    }

    response
}
