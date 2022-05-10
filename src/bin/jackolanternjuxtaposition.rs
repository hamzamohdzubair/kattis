use std::io::BufRead;

fn read_vec() -> Vec<i32> {

    let rv = std::io::stdin()
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
    rv
}

fn main() {

    let input_line = read_vec();
    println!("{}", input_line.iter().product::<i32>());

}