pub mod random_generator_mod {
    use common::separator;
    use random_generator::{
        generate_random_numbers, generate_random_numbers_range_float,
        generate_random_numbers_range_int, random_password, random_password_from_set,
    };

    const CHARSET: &[u8] = b"abc123xyz890";

    pub fn kitchen_sink(visible: bool) {
        if visible {
            separator("random_generator".to_string(), false);
            generate_random_numbers();
            println!(
                "Random range int: {}",
                generate_random_numbers_range_int(10, 20)
            );
            println!(
                "Random range float: {}",
                generate_random_numbers_range_float(1.55, 8.77)
            );
            println!("Random password: {}", random_password());

            println!(
                "Random 8-char password from character set: {}",
                random_password_from_set(CHARSET, 8)
            );
        }
    }
}
