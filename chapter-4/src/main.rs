fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("the length of '{}' is {}", s1, len);
}