use std::{io::BufRead, vec};

fn read_int() -> i32 {
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
    rv
}

fn main() {

    let num_lines = read_int();
    let mut vec_powers = vec![0; num_lines as usize];
    for ind in 0..num_lines as usize {
        let input = read_int();
        let exp = input % 10;
        let base = input / 10;
        vec_powers[ind] = base.pow(exp as u32);
    }
    println!("{}", vec_powers.iter().sum::<i32>());


}