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


#[cfg(test)]
mod const_fn {
    const fn salt(a: u32) -> u32 {
        0xDEADBEEF ^ a
    }

    const CHECKSUM: u32 = salt(23);

    #[test]
    fn test() {
        println!("{}", CHECKSUM);
    }
}

#[cfg(test)]
mod const_fn_file {
    const fn read_header(a: &[u8]) -> (u8, u8, u8, u8) {
        (a[0], a[1], a[2], a[3])
    }

    // 编译器完成
    const FILE_HEADER: (u8, u8, u8, u8) =
        read_header(include_bytes!("./global_values.rs"));

    #[test]
    fn test() {
        println!("{:?}", FILE_HEADER);
    }
}

// #[cfg(test)]
// mod lazy_static_demo {
//     lazy_static! {
//         static ref ITEMS: Mutex<Vec<u64>> = {
//             let mut v = vec![];
//             v.push(9);
//             v.push(2);
//             v.push(1);
//             Mutext::new(v)
//         }
//
//     }
// }