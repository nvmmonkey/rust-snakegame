fn main() {
    let mut msg = String::from("Hello");
    // let slice = &msg[2..4]; 
    
    let msg3 = msg.clone();
    msg.clear();

    println!("{}", msg); // empty string cleared
    println!("{}", msg3); // cloned string
}

