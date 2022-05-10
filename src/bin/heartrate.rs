use std::io::BufRead;

fn read_line() -> String {
    std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
}

fn read_int() -> i32 {
    read_line()
        .parse::<i32>()
        .unwrap()
}

fn read_vec() -> Vec<f32> {
    read_line()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<f32>().unwrap())
        .collect::<Vec<f32>>()
}

fn main(){
    let num_lines = read_int();
    // let case_vec = vec![0_f32; 2];
    for _ in 0..num_lines as usize {
        let case_vec = read_vec();
        let diff = 60.0/case_vec[1];
        let bpm = 60.0 * case_vec[0]/case_vec[1];
        println!("{} {} {}", bpm - diff, bpm , bpm + diff );
    }
}