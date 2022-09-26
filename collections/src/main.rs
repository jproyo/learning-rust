#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in &row {
        println!("{:?}", i);
    }

    for i in "hello".to_string().as_bytes() {
        println!("{}", *i as char)
    }

    let s = String::from("Hello");

    let s1 = &s[0..4];

    println!("{:?}", s1.chars().nth(0))
}
