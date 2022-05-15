use std::io::BufRead;

fn read_vec<T>() -> Vec<T> 
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    std::io::stdin()
    .lock()
    .lines()
    .next()
    .unwrap()
    .unwrap()
    .trim()
    .split_whitespace()
    .map(|s| s.parse::<T>().unwrap())
    .collect::<Vec<T>>()
}

fn main() {
    let num_inputs = read_vec::<usize>()[0];
    let mut transactions = vec![0; num_inputs];
    for ind in 0..num_inputs {
        transactions[ind] = read_vec::<i32>()[0];
    }
    let mut sum = 0;
    let mut min_balance = 0;
    for val in transactions.iter() {
        sum = sum + val;
        if min_balance > sum {
            min_balance = sum;
        } 
    }
    println!("{}", min_balance.abs());
}