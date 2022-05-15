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
    let input = read_vec::<i32>();
    let result = (input[1] + input[2]) / input[0];
    println!("{}", if result % 2 == 0 {"paul"} else {"opponent"});

}