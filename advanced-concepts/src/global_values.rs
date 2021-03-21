#[cfg(test)]
mod constants {
    const HEADER: &'static [u8; 4] = b"Pbj\0";

    #[test]
    fn test() {
        println!("{:?}", HEADER)
    }
}
