#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


#[cfg(test)]
mod strings {
    #[test]
    fn test()
    {
        let _a = "Hello".to_string();
        let _b = String::from("Hello");
        let c = "World".to_owned();
        let _d = c.clone();
    }
}
