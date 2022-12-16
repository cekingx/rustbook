fn takes_ownership(some_string: String) {
    println!("string: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("number: {}", some_integer);
}

fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);

    println!("main string: {}", s);
    println!("main number: {}", x);
}