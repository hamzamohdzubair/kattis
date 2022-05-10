use std::io::BufRead;

fn read_int() -> i32 {
    let rv = std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap()
        ;
    rv
}

fn main() {
    let input = read_int();
    println!("{}", 100.0 / input as f32);
    println!("{}", 100.0 / (100.0 - input as f32));
}