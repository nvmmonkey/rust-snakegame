#[derive(Debug)]
enum PersonId {
    Passport(u32),
    IdnentityCard(u32, u32, u32),
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
            id: PersonId::IdnentityCard(12, 23, 412),
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

    fn display_info(&self) {
        println!(
            " {} {} {} {:?}",
            self.name, self.last_name, self.age, self.id
        );
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
        PersonId::Passport(12312412),
    );
    person1.change_age(50);
    person1.display_info();

    //PersonId::IdnentityCard(12, 23, 412)
    check_person_id(person1.id);
    check_person_id(person2.id);
}

fn check_person_id(id: PersonId) {
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
