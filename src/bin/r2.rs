use std::io::BufRead;

fn read_vec() -> Vec<i32> {
    let vec = std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        ;
    return vec;
}

fn main() {

    let first_line = read_vec();
    let r1 = first_line[0];
    let s = first_line[1];
    let r2 = (2*s) - r1;

    println!("{}", r2);

}