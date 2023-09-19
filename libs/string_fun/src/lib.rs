pub fn first_word_in_string(s: &String) -> &str {
    let b = s.as_bytes();

    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

pub fn first_word_in_string_slice(s: &str) -> &str {
    let b = s.as_bytes();

    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}
/**
// let s = "Hello world"
// the type of s is &str !!!!!
// AND IT IS AN IMMUTABLE REFERENCE
If we have a string slice, we can pass that directly. If we have a String, we can pass a slice of the String or a reference to the String.
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_word_in_string_test() {
        let sentence = String::from("How now brown cow?");
        assert_eq!(first_word_in_string(&sentence), "How");
    }

    #[test]
    fn first_word_in_string_slice_test() {
        assert_eq!(first_word_in_string_slice("Foo Bar Baz"), "Foo");
    }

    #[test]
    fn first_word_in_string_slice_test_with_string() {
        let s = String::from("Four score and seven years ago...");
        assert_eq!(first_word_in_string_slice(&s[..]), "Four");
    }
}
