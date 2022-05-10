use std::io::BufRead;

fn read_input() -> i32 {
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
    return rv;

}

fn main() {

    let number = read_input();
    for index in 1..number+1 {
        println!("{} Abracadabra", index);
    }

}