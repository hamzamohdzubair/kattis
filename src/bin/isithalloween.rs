use std::io::BufRead;

fn read_line() -> String {
    let rv = std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        ;
    rv
}

fn main(){

    let input = read_line();//.trim();

    if input == "OCT 31" || input == "DEC 25" {
        println!("yup");
    }
    else {
        println!("nope");
    }

}