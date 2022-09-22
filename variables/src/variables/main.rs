const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;

    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("The value of constant time is: {THREE_HOURS_IN_SECONDS}");

    let y = i32::from_str_radix("32", 16);
    println!("value y {y:?}");

    let y: char = '\u{10FFF0}';
    println!("Print char {}", y);

    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");
}
