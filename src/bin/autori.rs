mod mod_read;

fn main() {
    let input = mod_read::read_line::<String>();
    let mut capitals: Vec<char> = Vec::new();
    for ch in input.chars() {
        // println!("{}", ch);
        if ch.is_ascii_uppercase() {
            capitals.push(ch);
            // capitals.push('-');
        }
    }
    // capitals.pop();
    println!("{}", capitals.iter().collect::<String>());

}