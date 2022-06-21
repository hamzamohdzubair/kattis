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
    let input = read_vec::<String>();
    let str1 = input[0].chars().rev().collect::<String>();
    let str2 = input[1].chars().rev().collect::<String>();

    let num1 = str1.parse::<i32>().unwrap();
    let num2 = str2.parse::<i32>().unwrap();

    if num1 > num2 {println!("{}", num1);}
    else {println!("{}", num2);}


}