struct Person {
    name: String, //fields
    last_name: String,
    age: u32
}




impl Person {
    // associated function
    fn some_func() {
       println!("some func!");
    }

    //method
    // 1st parameter is always self, represents the instance of the struct
    // method is being called on
    // within an impl block, the tyle Self is an alias for the current struct type
    fn display_age(&self){
        println!("Current Age: {}", self.age);
    }
}

fn main() {
    Person::some_func();

    let person1 = Person {
        name: "John".to_string(),
        last_name: "Wick".to_string(),
        age: 32,
    };

        let person2 = Person {
        name: "Kath".to_string(),
        last_name: "Nath".to_string(),
        age: 22,    
    };

    person1.display_age();
    person2.display_age();

    println!(
        "Person1: {} {}, Age: {}",
        person1.name,
        person1.last_name,
        person1.age
    );

}

