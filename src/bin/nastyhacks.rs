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

fn read_int() -> i32 {
    let rv = read_line()
        .trim()
        .parse::<i32>()
        .unwrap()
        ;

    rv
}

fn read_vec() -> Vec<i32> {
    let rv = read_line()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        ;
    rv
}

fn main(){

    let num_cases = read_int();
    for _ in 0..num_cases {
        let values = read_vec();
        let diff = values[1] - values[2] - values[0];
        if diff > 0 {
            println!("advertise")
        }
        else if diff == 0 {
            println!("does not matter")
        }
        else {
            println!("do not advertise")
        }
    }
}