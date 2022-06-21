use std::io::BufRead;

fn read_line() -> String {
    std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
}

fn main() {
    let input = read_line();
    let actual = input.len() /3;
    let sub1 = &input[0..actual];
    let sub2 = &input[actual..actual*2];
    let sub3 = &input[2*actual..actual*3];
    // println!("{}--{}--{}", sub1, sub2, sub3);
    if sub1 == sub2 {println!("{}", sub1)}
    else if sub2 == sub3 {println!("{}", sub2)}
    else if sub1 == sub3 {println!("{}", sub1)}

}