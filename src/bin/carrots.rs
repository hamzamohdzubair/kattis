use std::io::BufRead;

fn read_line_to_vec() -> Vec<i32> {
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
    return rv;
}

fn read_line() -> String {
    let rv = std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .into()
        ;
    return rv;
}

fn main() {
    let first_line = read_line_to_vec();
    let contestants = first_line[0];
    let problems = first_line[1];
    for _ in 0..contestants {
        read_line();
    }

    println!("{}", problems);

}