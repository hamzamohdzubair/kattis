use std::io::BufRead;

fn read_line() -> String {
    std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
}

fn read_num<T>() -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    read_line()
        .trim()
        .parse::<T>()
        .unwrap()
}

fn main() {
    let num_lines = read_num::<i32>();
    for ind in 1..=num_lines {
        let input_string = read_line();
        if ind % 2 != 0 {
            println!("{}", input_string);
        }
    }
}