pub mod string_fun_mod {
    use common::separator;
    use string_fun::{first_word_in_string, first_word_in_string_slice};

    pub fn kitchen_sink(visible: bool) {
        if visible {
            let s1 = String::from("All your worlds are belong to us.");
            let s2 = "How high can you get?";
            let s3 = String::from("A man, a plan, a canal.  Panama.");
            separator("String Fun".to_string(), false);

            println!(
                "First word should be \"All\": {}",
                first_word_in_string(&s1)
            );
            println!(
                "First word should be \"How\": {}",
                first_word_in_string_slice(s2)
            );
            println!(
                "First word should be \"A\": {}",
                first_word_in_string_slice(&s3[..])
            );
        }
    }
}
