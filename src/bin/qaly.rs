use std::io::BufRead;

fn read_line() -> String {
    let rv = std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        ;
    rv
}

fn read_int() -> i32 {
    let rv = read_line()
        .trim()
        .parse::<i32>()
        .unwrap()
        ;
    rv
}

fn read_vec() -> Vec<f32> {
    let rv = read_line()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<f32>().unwrap())
        .collect::<Vec<f32>>()
        ;
    rv
}

fn main(){

    let num_periods = read_int();
    let mut products_vec = vec![0_f32;num_periods as usize];
    for ind in 0..num_periods as usize {
        products_vec[ind] = read_vec().iter().product::<f32>();
    }
    println!("{:?}", products_vec.iter().sum::<f32>());
}