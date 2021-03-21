// #[cfg(test)]
// mod unsized_types {
//     #[test]
//     fn test() {
//         let a: str = "2048";
//     }
// }


#[cfg(test)]
mod function_types {
    fn add_two(a: u32, b: u32) -> u32 {
        a + b
    }

    #[test]
    fn test() {
        let my_func = add_two;
        let res = my_func(3, 4);
        println!("{:?}", res);
    }
}

#[cfg(test)]
mod unions {
    #[repr(C)]
    union Metric {
        rounded: u32,
        precise: f32,
    }

    #[test]
    fn test() {
        let mut a = Metric { rounded: 323 };
        unsafe {
            println!("{}", a.rounded);
        }
        unsafe {
            println!("{}", a.precise);
        }
        a.precise = 33.3;
        unsafe {
            println!("{}", a.precise);
        }
    }
}
