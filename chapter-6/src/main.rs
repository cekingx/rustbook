#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let quit = Message::Quit;
    let moved = Message::Move { x: 1, y: 1 };
    let write = Message::Write(String::from("message"));
    let color = Message::ChangeColor(0, 255, 0);

    println!("quit: {:?}", quit);
    println!("move: {:?}", moved);
    println!("write: {:?}", write);
    println!("color: {:?}", color);
}
