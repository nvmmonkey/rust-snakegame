//trait

pub mod learning_rust {
    mod top_level {
        pub fn hi_there() {
            println!("Hi there!");
        }

        pub mod low_level {
            pub fn hello_world() {
                println!("Hello world!");
            }
        }
    }

    pub trait Log {
        fn display_info(&self);
        // fn alert_some() {
        //     println!("Default implementation of alert_some()");
        // }
        fn alert_some_self(&self) {
            println!("Self implementation of alert_some()");
        }
    }

    #[derive(Debug)]
    pub enum PersonId {
        Passport(u32),
        IdnentityCard(u32, u32, u32),
    }

    pub struct Person {
        pub name: String, //fields
        pub last_name: String,
        pub age: u32,
        pub id: PersonId,
    }

    pub struct Animal(pub String, pub u32, pub String);

    impl Log for Person {
        fn display_info(&self) {

            // absolute path import
            // crate point to -> src/lib.rs and src/main.rs
            println!("====crate point to -> src/lib.rs and src/main.rs====");
            crate::learning_rust::top_level::hi_there();
            crate::learning_rust::top_level::low_level::hello_world();

            println!("====relative path import====");
            // relative path import
            top_level::hi_there();
            top_level::low_level::hello_world();

            println!(
                " {} {} {} {:?}",
                self.name, self.last_name, self.age, self.id
            );
        }
    }

    impl Log for Animal {
        fn display_info(&self) {
            println!(
                "Animal1 Type: {}, Age: {}, Breed: {}",
                self.0, self.1, self.2
            );
        }

        fn alert_some_self(&self) {
            println!("Animal implementation of alert_some()");
        }
    }

    impl Person {
        pub fn new() -> Person {
            Person {
                name: "Default".to_string(),
                last_name: "Default".to_string(),
                age: 0,
                id: PersonId::IdnentityCard(12, 23, 412),
            }
        }

        pub fn from(name: String, last_name: String, age: u32, id: PersonId) -> Person {
            Person {
                name,
                last_name,
                age,
                id,
            }
        }

        pub fn change_age(&mut self, new_age: u32) {
            self.age = new_age
        }

        fn alert_some_self(&self) {
            println!("Person implementation of alert_some()");
        }
    }

    pub fn check_person_id(id: PersonId) {
        if let PersonId::Passport(num) = id {
            println!("It matching Passport with number: {}", num);
        } else if let PersonId::IdnentityCard(x, y, z) = id {
            println!("It matching IdentityCard: {}-{}-{}", x, y, z);
        }

        //PersonId::IdnentityCard(12, 23, 412)
        let res = match id {
            PersonId::IdnentityCard(x, y, z) => y,

            //PersonId::Passport(12312412)
            PersonId::Passport(x) => x,
        };

        println!("Result: {}", res);
        // //PersonId::IdnentityCard(12, 23, 412)
        // match id {
        //     PersonId::IdnentityCard(x,y ,z )=> {
        //         println!("ID Card: 1st value: {} ", x);
        //         println!("ID Card: 2nd value: {} ", y);
        //         println!("ID Card: 3rd value: {} ", z);
        //     },

        //     //PersonId::Passport(12312412)
        //     PersonId::Passport(x)=>{
        //         println!("Passport - {} ", x);
        //     }
        // }
    }

    pub fn log_info(val: impl Log) {
        val.alert_some_self();
    }

    pub fn log_info_2(val: &dyn Log) {
        val.alert_some_self();
    }
}
