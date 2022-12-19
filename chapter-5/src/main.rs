struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

fn main() {
    let user1 = build_user(String::from("someone@example.com"), String::from("someusername"));
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotheruser"),
        ..user1
    };

    println!("user 1: {}", user1.email);
    println!("user 2: {}", user2.email);
}
