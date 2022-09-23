fn five() -> i32 {
    5
}

fn something(str: &String) {
    println!("Something {}", &str);
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");

    let x = x.to_string();
    something(&x);
    something(&x);
}
