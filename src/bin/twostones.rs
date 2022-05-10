// use std::io::{self, Read, BufRead};
use std::io;

fn main(){

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n).unwrap();
    let n = n.trim().parse::<i32>().unwrap();

    println!("{}", if n % 2 == 0 {"Bob"} else {"Alice"});

    
    // if let Ok(n) = n.trim().parse::<i32>()




}