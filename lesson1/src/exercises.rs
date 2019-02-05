pub mod exercise1 {
    pub fn run() {
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
}

pub mod exercise2 {
    pub fn run() {
        let mut i = 1;

        loop {
            println!("i == {}", i);
            if i >= 10 {
                break;
            } else {
                i += 1;
            }
        }
    }
}

pub mod exercise4 {
    pub fn run() {
        println!("FizzBuzz");
        for i in 1..100 {
            fizz_buzz(i);
        }
    }

    fn fizz_buzz(i: i32) {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}
