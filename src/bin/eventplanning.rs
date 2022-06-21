mod mod_read;

fn main() {
    let part_bgt_htl_wks = mod_read::read_vec::<usize>();
    let mut min_cost = usize::MAX;
    for _ in 0..part_bgt_htl_wks[2] {
        let price = mod_read::read_line::<usize>();
        let availability = mod_read::read_vec::<usize>();
        for rooms in availability.iter() {
            if rooms >= &part_bgt_htl_wks[0] {
                let cost = part_bgt_htl_wks[0] * price;
                if cost <= part_bgt_htl_wks[1] {
                    min_cost = usize::min(min_cost, cost);
                }
            }
        }

    }
    if min_cost < usize::MAX {
        println!("{}", min_cost);
    }
    else {
        println!("stay home")
    }
}