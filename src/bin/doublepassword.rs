mod mod_read;
fn main() {
    let line1 = mod_read::read_line::<String>();
    let line2 = mod_read::read_line::<String>();

    // println!("{}", line1);
    // println!("{}", line2);
    // for char_from_line1 in line1.chars() {

    // }
    let mut diff_digits = 0;
    for ind in 0..4 {
        if line1.chars().nth(ind).unwrap() != line2.chars().nth(ind).unwrap() {
            diff_digits += 1;
        }
    }

    println!("{}", 2_i32.pow(diff_digits));



}