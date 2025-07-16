
struct A {
    a: i32,
}

struct B {
    b: i32,
}


fn main() {
    let a = A { a: 42, };
    let b = B { b: 33, };
    println!("a={:?}", a.a);
    println!("b={:?}", b.b);
}