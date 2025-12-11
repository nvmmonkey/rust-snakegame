#[derive(Debug)]
enum PersonId {
    Passport,
    IdnentityCard,
}

struct Person {
    name: String, //fields
    last_name: String,
    age: u32,
    id: PersonId,
}

impl Person {
    fn new() -> Person {
        Person {
            name: "Default".to_string(),
            last_name: "Default".to_string(),
            age: 0,
            id: PersonId::IdnentityCard,
        }
    }

    fn from(name: String, last_name: String, age: u32, id: PersonId) -> Person {
        Person {
            name,
            last_name,
            age,
            id,
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
    let person2 = Person::from(
        "Kath".to_string(),
        "Nath".to_string(),
        22,
        PersonId::Passport,
    );
    person1.change_age(50);

    println!("{:?}", person1.id);
    println!("{:?}", person2.id);
}
