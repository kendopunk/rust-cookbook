pub mod person;

#[cfg(test)]
mod tests {
    use super::*;

    const NAME: &str = "Markus";
    const AGE: u32 = 50;

    #[test]
    fn it_verifies_person_name() {
        let p = person::person::Person::new(NAME.to_string(), AGE);
        assert_eq!(p.get_name(), NAME.to_string());
    }

    #[test]
    fn it_verifies_person_age() {
        let p = person::person::Person::new(NAME.to_string(), AGE);
        assert_eq!(p.get_age(), AGE);
    }
}
