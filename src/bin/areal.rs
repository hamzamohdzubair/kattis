use std::io::BufRead;

fn read_num<T>() -> T
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
    .parse::<T>()
    .unwrap()
}

fn main() {
    let input = read_num::<f32>();
    println!("{}", input.sqrt()*4.0);
}