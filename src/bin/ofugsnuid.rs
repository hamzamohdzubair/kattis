use std::io::BufRead;


fn read_line() -> String {
    std::io::stdin()
    .lock()
    .lines()
    .next()
    .unwrap()
    .unwrap()
}

fn read_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    read_line()
    .trim()
    .split_whitespace()
    .map(|s| s.parse::<T>().unwrap())
    .collect::<Vec<T>>()
}

fn main() {
    let input = read_vec::<usize>();
    let mut line_vec = vec![0;input[0]];
    for ind in 0..input[0] {
        line_vec[ind] = read_vec::<i32>()[0];
    }
    line_vec.reverse();
    for val in line_vec.iter() {
        println!("{}", val)
    }

}