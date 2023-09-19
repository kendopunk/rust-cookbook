pub mod vector_sort_mod {
    use common::separator;
    use structs::person::person::Person;
    use vector_sort::{
        sort_float_vector, sort_int_vector, sort_people_by_age_desc, sort_people_by_name,
    };

    pub fn kitchen_sink(visible: bool) {
        if visible {
            let unsorted_int_v = vec![8, 7, 6, 1, 2, 3];
            let unsorted_float_v = vec![1.0, 3.0, 5.0, 2.0, 4.0, 6.0];
            let people1: Vec<Person> = vec![
                Person::new("Fred".to_string(), 75),
                Person::new("Clute".to_string(), 45),
                Person::new("Chad".to_string(), 33),
                Person::new("Angie".to_string(), 41),
            ];
            let people2: Vec<Person> = vec![
                Person::new("Fred".to_string(), 75),
                Person::new("Clute".to_string(), 45),
                Person::new("Chad".to_string(), 33),
                Person::new("Angie".to_string(), 41),
            ];

            separator("vector_sort".to_string(), false);
            println!("Sorted int vector: {:?}", sort_int_vector(unsorted_int_v));
            println!(
                "Sorted float vector: {:?}",
                sort_float_vector(unsorted_float_v)
            );
            println!(
                "Sorted Person vector (by name):\n{:?}",
                sort_people_by_name(people1)
            );
            println!(
                "Sorted Person vector (by age DESC):\n{:?}",
                sort_people_by_age_desc(people2)
            );
        }
    }
}
