extern crate rust_project_structure;

use crate::rust_project_structure::module::a::*;
use crate::rust_project_structure::module::b::*;

fn main() {
    let a = A { a: 42, };
    let b = B { b: 33, };
    println!("a={:?}", a.a);
    println!("b={:?}", b.b);
}