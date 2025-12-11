
use udemy::Person;
use udemy::PersonId;
use udemy::Animal;
use udemy::Log;
use udemy::check_person_id;

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

