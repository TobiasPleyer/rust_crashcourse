fn main() {
    let val: String = String::from("Hello, World!");
    printer(val.clone());
    printer(val);

    let val2: String = String::from("Hello, World!");
    printer_ref(&val2);
    printer_ref(&val2);
}

fn printer(val: String) {
    println!("The value is: {}", val);
}

fn printer_ref(val: &String) {
    println!("The value is: {}", val);
}
