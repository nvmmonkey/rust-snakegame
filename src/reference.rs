fn main() {
    let mut msg = String::from("Hello");
    // borrow msg instead of moving it, msg2 is not owning msg
    let msg2= &mut msg; 
    
    // msg2.push_str(" World");
    
    unpredictable_change(msg2);
    let msg3 = &msg;
    println!("{}", msg);
    // println!("{}", msg2);
}

fn unpredictable_change(a: &mut String) {
    a.push_str("_unpredictable");
}
