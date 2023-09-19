use structs::person::person::Person;

pub fn sort_int_vector(mut v: Vec<i32>) -> Vec<i32> {
    v.sort();
    v
}

pub fn sort_float_vector(mut v: Vec<f32>) -> Vec<f32> {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v
}

pub fn sort_people_by_name(mut people: Vec<Person>) -> Vec<Person> {
    people.sort_by(|a, b| a.name.cmp(&b.name));
    people
}

pub fn sort_people_by_age_desc(mut people: Vec<Person>) -> Vec<Person> {
    people.sort_by(|a, b| b.age.cmp(&a.age));
    people
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_vector_sort() {
        let input = vec![1, 2, 7, 8, 3, 4];
        let expected = vec![1, 2, 3, 4, 7, 8];
        assert_eq!(sort_int_vector(input), expected);
    }

    #[test]
    fn float_vector_sort() {
        let input: Vec<f32> = vec![1.0, 2.0, 4.0, 5.0, 3.0, 9.0, 8.0];
        let expected: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 8.0, 9.0];
        assert_eq!(sort_float_vector(input), expected);
    }

    #[test]
    fn person_vector_sort_by_name() {
        let people: Vec<Person> = vec![
            Person::new("Fred".to_string(), 44),
            Person::new("Wopper".to_string(), 35),
            Person::new("Chad".to_string(), 24),
        ];

        let sorted_people = sort_people_by_name(people);
        assert_eq!(
            sorted_people,
            vec![
                Person::new("Chad".to_string(), 24),
                Person::new("Fred".to_string(), 44),
                Person::new("Wopper".to_string(), 35)
            ]
        );
    }

    #[test]
    fn person_vector_sort_by_age() {
        let people: Vec<Person> = vec![
            Person::new("Fred".to_string(), 44),
            Person::new("Wopper".to_string(), 35),
            Person::new("Chad".to_string(), 24),
        ];

        let sorted_people = sort_people_by_age_desc(people);
        assert_eq!(
            sorted_people,
            vec![
                Person::new("Fred".to_string(), 44),
                Person::new("Wopper".to_string(), 35),
                Person::new("Chad".to_string(), 24)
            ]
        );
    }
}
