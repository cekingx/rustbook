struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point {x: 0, y: 7};

    let Point { x: a, y: b } = p;
    println!("a: {a}, b: {b}");
}
