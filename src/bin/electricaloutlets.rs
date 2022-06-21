mod mod_read;

fn main() {
    let input = mod_read::read_line::<i32>();
    for _ in 0..input {
        let outlets = mod_read::read_vec::<i32>();
        println!("{}", outlets[1..].iter().sum::<i32>() - outlets[2..].len() as i32);

    }
}