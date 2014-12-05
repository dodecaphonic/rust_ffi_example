use std::ptr;

mod magic {
    #[repr(C)]
    pub struct LinkedList {
        pub value: i64,
        pub next: *const LinkedList
    }

    #[link(name = "magic")]
    extern {
        fn magic_multiply(list: *const LinkedList) -> i64;
        fn magic_pow(n: f64, p: f64) -> f64;
    }

    pub fn pow(n: f64, p: f64) -> f64 {
        unsafe {
            magic_pow(n, p)
        }
    }

    pub fn multiply(ll: &LinkedList) -> i64 {
        unsafe {
            magic_multiply(&*ll)
        }
    }
}

fn main() -> () {
    println!("{}", magic::pow(102030f64, 0.42));

    let ll = magic::LinkedList {
        value: 10,
        next: &magic::LinkedList {
            value: 20,
            next: &magic::LinkedList {
                value: 3,
                next: ptr::null()
            }
        }
    };

    println!("{}", magic::multiply(&ll));
}
