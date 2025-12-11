use core::net;

struct Person {
    name: String, //fields
    last_name: String,
    age: u32,
}

impl Person {
    fn new() -> Person {
        Person {
            name: "Default".to_string(),
            last_name: "Default".to_string(),
            age: 0,
        }
    }

    fn from(name: String, last_name: String, age: u32) -> Person {
        Person {
            name,
            last_name,
            age,
        }
    }

    fn change_age(&mut self, new_age: u32) {
        self.age = new_age
    }
}

fn main() {
    // let mut person1 = Person {
    //     name: "John".to_string(),
    //     last_name: "Wick".to_string(),
    //     age: 32,
    // };
    let mut person1 = Person::new();
    let person2 = Person::from("Kath".to_string(), "Nath".to_string(), 22);
    person1.change_age(50);

    println!(
        "Person1: {} {}, Age: {}",
        person1.name, person1.last_name, person1.age
    );
    println!(
        "Person1: {} {}, Age: {}",
        person2.name, person2.last_name, person2.age
    );
}
