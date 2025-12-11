fn main() {
    let msg = String::from("Hello");
    // let msg2 = extend_message(msg);
    extend_message(msg);

    let age = 30;
    extend_age(age);

    println!("{}", age);

    println!("{}", msg);
}
fn extend_message(mut a: String) -> String {
    a.push_str(" World");
    a
}

fn extend_age(mut a: u32) {
    a += 10;
    // a
}
