use std::{io::BufRead};
fn main(){
    let vec_numbers: Vec<i32> = std::io::stdin()
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
        
    // let sum: i32 = vec_numbers.iter().sum();
    // println!("{:?}", vec_numbers);
    println!("{}", vec_numbers.iter().sum::<i32>());

}