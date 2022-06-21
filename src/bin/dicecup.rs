mod mod_read;

fn main() {
    let input = mod_read::read_vec::<usize>();
    let mut score = std::collections::HashMap::new();
    for i in 1..=input[0] {
        for j in 1..=input[1] {
            let sum = i+j;
            *score.entry(sum).or_insert(0) += 1;
        }
    }
    println!("{:#?}",
    score
    .iter()
    .max_by_key(|entry| entry.0)
    // .max_by(|a, b| a.1.cmp(&b.1))
    .unwrap()
    // .0
    // .map(|k, _v| k)
    
    );
}