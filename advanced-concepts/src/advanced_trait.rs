#[cfg(test)]
mod object_safety {
    trait Foo {
        // fn foo();
        fn foo(&self);
    }

    fn generic(val: &Foo) {}

    #[test]
    fn test() {}
}

#[cfg(test)]
mod ufcs {
    trait Driver {
        fn drive(&self) {
            println!("Driver's driving!");
        }
    }

    struct MyCar;

    impl MyCar {
        fn drive(&self) {
            println!("I'm driving!");
        }
    }

    impl Driver for MyCar {}

    #[test]
    fn test() {
        let car = MyCar;
        car.drive();
    }
}
