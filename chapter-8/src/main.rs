fn main() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    let number = match row[0] {
        SpreadsheetCell::Int(some) => some,
        _ => 0
    };

    println!("number: {}", number);
    println!("row: {:?}", row);
}
