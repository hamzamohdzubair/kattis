use std::io::BufRead;

fn read_line() -> String {
    let rv = std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        ;
    rv
}

fn main() {
    let input = read_line();

}