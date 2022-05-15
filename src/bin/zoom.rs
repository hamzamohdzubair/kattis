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
    let input = read_vec::<usize>();
    let mut ser = read_vec::<i32>();
    ser.insert(0, 0);
    for val in ser.iter().step_by(input[1]).skip(1) {print!("{} ", val);}
}