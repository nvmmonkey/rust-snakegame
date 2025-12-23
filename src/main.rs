use udemy::learning_rust::{Animal, Log, Person, PersonId, check_person_id};

// local private import not able to call in the main.rs
// use udemy::learning_rust::top_level;


fn main() {
    let person1 = Person::new();
    person1.display_info();
}
