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
    let lower_limit = read_line::<i32>();
    let upper_limit = read_line::<i32>();
    let sum_result = read_line::<i32>();
    let mut all_numbers = Vec::new();
    for val in lower_limit..=upper_limit {
        let mut sum = 0;
        let mut num = val.clone();
        while num > 0 {
            sum = sum + (num % 10);
            num = num / 10;
        }
        if sum == sum_result {
            // println!("{}", val);
            all_numbers.push(val);
        }
    }
    println!("{}", all_numbers[0]);
    println!("{}", all_numbers.last().unwrap());
}