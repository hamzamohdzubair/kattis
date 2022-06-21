use std::io::BufRead;

fn read_line<T>() -> T
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
        .parse::<T>()
        .unwrap()
}

fn read_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    read_line::<String>()
        .split_whitespace()
        .map(|s| s.parse::<T>().unwrap())
        .collect::<Vec<T>>()
        
}

fn main() {
    let input = read_line::<String>();
    println!("{}", if input.len() as f64 / 2.0 == input.find(')').unwrap() as f64 {"correct"} else {"fix"});

}