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

fn main() {
    let input = read_line::<String>();
    
}