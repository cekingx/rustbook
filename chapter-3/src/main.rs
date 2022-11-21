fn one() -> i32 {
    1
}

fn main() {
    let y = {
        let x = 3;

        x + one()
    };

    println!("The value of y is: {}", y);
}