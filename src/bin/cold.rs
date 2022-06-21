
mod mod_read;

fn main() {
    let _ = mod_read::read_line::<String>();
    let input_vec = mod_read::read_vec::<i32>();
    let mut count = 0;
    for elem in input_vec {
        if elem < 0 {
            count += 1;
        }
    }
    println!("{}", count);

    
}