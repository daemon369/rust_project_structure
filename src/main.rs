
mod module {
    pub struct A {
        pub a: i32,
    }

    pub struct B {
        pub b: i32,
    }
}

use crate::module::*;

fn main() {
    let a = A { a: 42, };
    let b = B { b: 33, };
    println!("a={:?}", a.a);
    println!("b={:?}", b.b);
}