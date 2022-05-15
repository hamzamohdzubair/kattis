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

fn sum_of_n(input: &str, iterations: i32) -> i32 {
    let mut start = if input == "even" {2} else {1};
    let increment = if input == "all" {1} else {2};
    let mut i = 1;
    let mut sum = 0;
    while i <= iterations {
        i = i + 1;
        sum = sum + start;
        start = start + increment;
    }
    sum
}

fn main() {
    let input = read_vec::<usize>()[0];
    for ind in 1..=input {
        let iterations = read_vec::<i32>()[1];
        println!("{} {} {} {}", 
        ind,
        sum_of_n("all", iterations),
        sum_of_n("odd", iterations),
        sum_of_n("even", iterations))
    }
}