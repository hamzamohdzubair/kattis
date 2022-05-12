use std::io::BufRead;

fn read_line() -> String {
    std::io::stdin()
    .lock()
    .lines()
    .next()
    .unwrap()
    .unwrap()
}

fn main() {
    let input = read_line();
    if &input[0..3] == "555" {
        println!("1");
    }
    else {
        println!("0");
    }
}