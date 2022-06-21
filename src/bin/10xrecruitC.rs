mod mod_read;

fn exhaustive_search(min_window_size: usize, sequence: &str) -> (usize, usize) {
    let mut max_success_rate = 0_f32;
    let mut msr_window_size = 0;
    let mut msr_starting_index = 0;
    'outer: for window_size in min_window_size..=sequence.len(){
        // println!("{}", window_size);
        flame::start("outer loop");
        for ind in 0..=(sequence.len() - window_size) {
            flame::start("inner loop");
            let substr = &sequence[ind..(window_size + ind)];
            // println!("{}", &sequence[ind..(window_size+ind)]);
            // let x = substr.chars().filter_map(|s| s.to_digit(10)).collect::<Vec<u32>>();
            // let success_rate = x.iter().sum::<u32>() as f32 / window_size as f32;
            let success_rate = substr.matches('1').count() as f32 / window_size as f32;
            if success_rate > max_success_rate {
                max_success_rate = success_rate;
                msr_window_size = window_size;
                msr_starting_index = ind;
                flame::note("gt than", Some("sr>msr"));
            }
            if success_rate == 1.0 {
                break 'outer;
            }
            flame::end("inner loop");
        }
        flame::end("outer loop");
    }
    (msr_starting_index, msr_window_size)
}

// fn decreasing_search(min_window_size: usize, sequence: &str) -> (usize, usize) {
//     for window_size in (min_window_size..=sequence.len()).rev() {

//     }
// }

fn main() {
    let min_window_size = mod_read::read_line::<usize>();
    let sequence = mod_read::read_line::<String>();
    let (index, window) = exhaustive_search(min_window_size, &sequence);
    println!("{} {}", index+1, window);
    println!("{} {}", index, index+window-1);
    flame::dump_html(&mut std::fs::File::create("flame_graph.html").unwrap()).unwrap();
}