fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("hello world");
    let word_one = first_word(&my_string[..]);
    let my_string_literal = "hello world";
    let word_two = first_word(&my_string_literal[..]);
    let word_three = first_word(my_string_literal);

    println!("word {}, {}, {}", word_one, word_two, word_three);
}