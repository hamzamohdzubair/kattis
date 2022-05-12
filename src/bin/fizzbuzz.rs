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
    let input = read_vec::<i32>();
    for ind in 1..=input[2] {
        let mut flag = false;

        if ind % input[0] == 0 {print!("Fizz"); flag = true;}
        if ind % input[1] == 0 {print!("Buzz"); flag = true;}
        if flag == false {print!("{}", ind);}

        println!("")
    }
}