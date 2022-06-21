use std::collections::HashMap;
mod mod_read;

fn main() {
    let input = mod_read::read_line::<String>();
    let mut char_hash: HashMap<char, usize> = HashMap::new();
    for ch in input.chars() {
        *char_hash.entry(ch).or_insert(0) += 1;
    }
    println!("{:?}", !char_hash.values().any(|&x| x > 1) as i32);

}