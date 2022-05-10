use std::io::BufRead;

fn read_line_int() -> i32{
    let rv = std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap()
        ;
    return rv
}

fn main(){
    let plan = read_line_int();
    let months = read_line_int();
    let mut consumed = vec![0; months as usize];

    for month in 0..months as usize {
        consumed[month] = read_line_int();
    }

    println!("{:?}", (plan * (months+1)) - consumed.iter().sum::<i32>());
}