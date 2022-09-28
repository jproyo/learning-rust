use minigrep::{run, Config};
use std::{env, process, rc::Rc};

fn main() {
    let x: env::Args = env::args();
    let y: Vec<String> = x.into_iter().collect();
    let mut v = vec![];
    for v1 in y.into_iter() {
        v.push(Rc::new(v1));
    }
    //for v1 in y.iter() {
    //    v.push(v1.as_str());
    //}
    let config = Config::build(v.iter().map(|x| x.as_str())).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1)
    });

    if let Err(e) = run(&config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
