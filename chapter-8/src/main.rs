fn main() {
    let s1 = String::from("hello, ");
    let s2 = String::from("world");
    let s3 = format!("{}-{}", s1, s2);
    println!("s3: {}", s3);
    println!("s1: {}", s1);
}
