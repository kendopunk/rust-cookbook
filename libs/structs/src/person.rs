pub mod person {

    #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
    pub struct Person {
        pub name: String,
        pub age: u32,
    }

    impl Person {
        pub fn new(name: String, age: u32) -> Self {
            Person { name, age }
        }

        pub fn get_name(&self) -> String {
            self.name.to_owned()
        }

        pub fn get_age(&self) -> u32 {
            self.age.to_owned()
        }
    }
}
