//trait

trait Log {
    fn display_info(&self);
    // fn alert_some() {
    //     println!("Default implementation of alert_some()");
    // }
    fn alert_some_self(&self) {
        println!("Self implementation of alert_some()");
    }
}

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

struct Animal(String, u32, String);


impl Log for Person {
    fn display_info(&self) {
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

        fn alert_some_self(&self) {
        println!("Person implementation of alert_some()");
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
    person1.alert_some_self();
    // Person::alert_some();

    //PersonId::IdnentityCard(12, 23, 412)
    check_person_id(person1.id);
    check_person_id(person2.id);

    let animal1 = Animal("dog".to_string(), 4, "bulldog".to_string());
    let animal2 = Animal("cat".to_string(), 2, "persian".to_string());

    animal1.display_info();
    animal2.display_info();
    animal1.alert_some_self();
    // Animal::alert_some();

}

fn check_person_id(id: PersonId) {
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

fn log_info(val: impl Log) {
    val.alert_some_self();
}

fn log_info_2(val: &dyn Log) {
    val.alert_some_self();
}