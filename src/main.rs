mod module;

use crate::module::a::*;
use crate::module::b::*;

fn main() {
    let a = A { a: 42, };
    let b = B { b: 33, };
    println!("a={:?}", a.a);
    println!("b={:?}", b.b);
}