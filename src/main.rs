// use structs::person::person::Person;

fn get_first_word(s: &String) -> &str {
    let b = s.as_bytes();

    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    println!("{}", "=".repeat(25));
    println!("= main.rs");
    println!("{}", "=".repeat(25));

    let sentence = String::from("Fred a");
    println!("{}", get_first_word(&sentence));
}
