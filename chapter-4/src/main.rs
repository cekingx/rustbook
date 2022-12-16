fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn main() {
    let mut s1 = String::from("hello");

    change(&mut s1);

    println!("string: {}", s1);
}