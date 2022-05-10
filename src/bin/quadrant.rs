use std::io::{self, BufRead};

fn main(){

    let mut coords = vec![0; 2];
    for x in 0..2 {
        let mut coord = String::new();
        io::stdin().read_line(&mut coord).unwrap();
        coords[x] = coord.trim().parse::<i32>().unwrap();
    }
    // println!("{:?}", coords);

    if coords[0] > 0 && coords[1] > 0 {
        println!("1");
    }
    else if coords[0] < 0 && coords[1] > 0 {
        println!("2");
    }
    else if coords[0] < 0 && coords[1] < 0 {
        println!("3");
    }
    else {
        println!("4");
    }


    // let mut stdin = io::stdin();
    // let stdin = io::stdin();
    // for line in stdin.lock().lines() {
    //     println!("{}", line.unwrap());
    // }
    // println!("{:?}", stdin.lock().lines().unwrap())

}