struct Person {
    name: String,
    last_name: String,
    age: Option<u32>,
}

fn main() {
    let person1 = Person {
        name: "John".to_string(),
        last_name: "Wick".to_string(),
        age: Some(32),
    };

    println!(
        "Person1: {} {}, Age: {}",
        person1.name,
        person1.last_name,
        match person1.age {
            Some(age) => age.to_string(),
            None => "N/A".to_string(),
        }
    );

}

