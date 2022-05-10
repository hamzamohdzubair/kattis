use std::io::BufRead;

fn read_line() -> String {
    std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
}

fn main(){
    let input = read_line();
    println!("h{}y", std::iter::repeat("e").take(2*(input.chars().count()-2)).collect::<String>())
}