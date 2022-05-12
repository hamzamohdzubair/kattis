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
    let input = read_vec::<f64>();
    if input[0] <= input[1] * std::f64::consts::SQRT_2 {
        println!("fits");
    }
    else {
        println!("nope");
    }

}