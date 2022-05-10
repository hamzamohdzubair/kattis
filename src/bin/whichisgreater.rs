use std::io::BufRead;
fn main(){
    let numbers: Vec<i32> = std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
        ;

    if numbers[0] > numbers[1] {
        println!("1");
    }
    else {
        println!("0");
    }
}