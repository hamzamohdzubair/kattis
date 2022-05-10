use std::{io::BufRead, vec};

fn read_int() -> i32 {
    let rv = std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<i32>()
        .unwrap()
        ;

    rv
}

fn main(){

    let num_lines = read_int();
    let mut vec_lines = vec![0; num_lines as usize];
    for ind in 0..num_lines as usize {
        vec_lines[ind] = read_int();
    }
    println!("{}", vec_lines.iter().sum::<i32>() - (vec_lines.len() as i32 - 1))

}