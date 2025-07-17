use library::module::A;
use library::module::B;

fn main() {
    let a = A { a: 42, };
    let b = B { b: 33, };
    println!("a={:?}", a.a);
    println!("b={:?}", b.b);
}