use std::io::BufRead;
use std::result;
use std::str::FromStr;
use std::error::Error;

fn read_line<T>() -> T
where
    T: FromStr,
    <T as FromStr>::Err: Error,
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

fn read_vec<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Error,
{
    std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<T>().unwrap())
        .collect::<Vec<T>>()
        

}


fn main() {
    let ref_vec = vec![1, 1, 2, 2, 2, 8];
    let input_vec = read_vec::<i32>();
    // let mut result_vec = Vec::new();

    let a = ref_vec.iter().zip(input_vec);

    for elem in a {
        // result_vec.push(elem.0-elem.1);
        print!("{} ", elem.0 - elem.1);
    }

    // println!("{:?}", result_vec);

}