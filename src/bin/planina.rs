use std::io::BufRead;

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

    let n_iter = read_int();
    let mut exp_vec = vec![0;n_iter as usize];
    for ind in (0..n_iter as usize).rev() {
        exp_vec[ind] = 2_u32.pow(ind as u32);
        // println!("{}", ind);
    }
    println!("{:?}", (exp_vec.iter().sum::<u32>()+2).pow(2));

}