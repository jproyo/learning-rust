fn something(str: &String) {
    println!("Something {}", &str);
}

fn borrowing() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
    println!("{}", x)
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn no_dangle() -> String {
    String::from("hello")
}

fn first_word(s: &str) -> &str {
    let b = s.as_bytes();

    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s
}

fn second_word(s: &str) -> &str {
    let b = s.as_bytes();
    let mut count = 0;
    for (i, &item) in b.iter().enumerate() {
        if item == b' ' && count > 0 {
            return &s[count..i];
        } else if item == b' ' {
            count = i + 1;
        }
    }

    &s[count..]
}

fn main() {
    let s1 = String::from("hello");
    let mut s2 = s1;

    println!("{}, world!", s2);

    something(&s2);
    s2.push_str(", other");
    something(&s2);
    println!("{}", s2);

    borrowing();

    let mut s = String::from("hello");

    change(&mut s);

    let _a = no_dangle();

    println!("{}", first_word(&String::from("hello world")));
    println!("{}", second_word(&String::from("hello world and other")));
}
