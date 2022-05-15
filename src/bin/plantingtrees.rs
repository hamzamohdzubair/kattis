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
    let _ = read_vec::<i32>();
    let input = read_vec::<i32>();
    let mut greatest = 0;
    for (ind, val) in input.iter().enumerate() {
        let sum = ind as i32 + val;
        if sum > greatest {
            greatest = sum;
        }

    }
    println!("{}", greatest + 1);

}