use std::io::BufRead;
mod mod_read;

fn main() {
    let input = mod_read::read_line::<String>();
    println!("{}{}", &input[1..2], &input[0..1]);


}