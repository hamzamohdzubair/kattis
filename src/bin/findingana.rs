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
    let index = input.find('a').unwrap();
    println!("{}", &input[index..])
}