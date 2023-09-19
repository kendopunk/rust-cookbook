#![warn(unused_imports)]
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

/**
 * simple random number generator
 */
pub fn generate_random_numbers() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}

/**
 * random number within a range of ints
 */
pub fn generate_random_numbers_range_int(a: i32, b: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(a..b)
}

/**
 * random number within a range of floats
 */
pub fn generate_random_numbers_range_float(a: f32, b: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(a..b)
}

/**
 * random password
 */
pub fn random_password() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    rand_string
}

pub fn random_password_from_set(charset: &[u8], password_length: usize) -> String {
    let mut rng = rand::thread_rng();

    let password: String = (0..password_length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();

    password
}

#[cfg(test)]
mod tests {
    use super::*;
    use more_asserts as ma;

    #[test]
    fn it_number_from_int_range() {
        let lower: i32 = 0;
        let upper: i32 = 10;
        let result = generate_random_numbers_range_int(lower, upper);
        ma::assert_ge!(result, lower);
        ma::assert_le!(result, upper);
        assert!(result >= lower && result <= upper);
    }

    #[test]
    fn it_number_from_float_range() {
        let lower: f32 = 5.5;
        let upper: f32 = 6.5;
        let result = generate_random_numbers_range_float(lower, upper);
        assert!(result >= lower && result <= upper);
    }

    #[test]
    fn random_pass_from_range() {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
        const LEN: usize = 20;
        let result = random_password_from_set(CHARSET, LEN);
        assert!(result.len() == LEN);
    }
}
