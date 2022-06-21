
mod mod_read;
fn main() {
    let _ = mod_read::read_line::<usize>();
    let vec_amount = mod_read::read_vec::<isize>();
    let sum = vec_amount.into_iter().filter(|&x| x<0 ).sum::<isize>();
    println!("{}", sum.abs());


}