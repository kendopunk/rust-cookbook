pub fn separator(s: String, extra_newline: bool) {
    println!("{}", "=".repeat(25));
    println!("= {}", s);
    println!("{}", "=".repeat(25));
    if extra_newline {
        println!("");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
