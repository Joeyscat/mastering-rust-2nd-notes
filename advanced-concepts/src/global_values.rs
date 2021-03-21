#[cfg(test)]
mod constants {
    const HEADER: &'static [u8; 4] = b"Pbj\0";

    #[test]
    fn test() {
        println!("{:?}", HEADER)
    }
}

#[cfg(test)]
mod statics {
    static mut BAZ: u32 = 4;
    static FOO: u8 = 9;


    #[test]
    fn test() {
        unsafe {
            println!("bax is {}", BAZ);
            BAZ = 42;
            println!("bax is now {}", BAZ);
            println!("foo is {}", FOO);
        }
    }
}