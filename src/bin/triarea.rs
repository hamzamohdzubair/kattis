use std::io::BufRead;

fn main(){
    let tri_params: Vec<i32> = std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
        ;
    println!("{}", 0.5 * tri_params.iter().product::<i32>() as f64);
}