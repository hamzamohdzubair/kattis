mod mod_read;

// fn read_line() -> String {
//     std::io::stdin()
//         .lock()
//         .lines()
//         .next()
//         .unwrap()
//         .unwrap()
// }

fn main() {
    let input = mod_read::read_line::<String>();
    println!("{} {} {}", input, input, input);

}