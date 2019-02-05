pub mod lesson1 {
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
}

pub mod lesson2 {

    #[derive(Debug)]
    struct FooBar(i32);

    impl Drop for FooBar {
        fn drop(&mut self) {
            println!("Dropping FooBar with value {}", self.0);
        }
    }

    pub mod exercise1 {
        use crate::lessons::lesson2::FooBar;

        pub fn drop<T>(_x: T) {
        }

        pub fn run() {
            println!("Before everything");
            let x = FooBar(1);
            drop(x);
            println!("After drop(x)");
            let _y = FooBar(2);
            println!("End");
        }
    }

    pub mod exercise2 {
        use crate::lessons::lesson2::FooBar;

        impl FooBar {
            fn use_it(& self) {
                println!("I consumed a FooBar with value {}", self.0);
            }
        }

        fn uses_foobar(foobar: &FooBar) {
            foobar.use_it();
        }

        pub fn run() {
            let foobar = FooBar(1);
            uses_foobar(&foobar);
        }
    }

    pub mod exercise3 {
        #[derive(Debug)]
        struct Foobar(i32);

        impl Copy for Foobar { }

        impl Clone for Foobar {
            fn clone(&self) -> Foobar {
                *self
            }
        }

        fn uses_foobar(foobar: Foobar) {
            println!("I consumed a Foobar: {:?}", foobar);
        }

        pub fn run() {
            let x = Foobar(1);
            uses_foobar(x);
            uses_foobar(x);
        }
    }
}
