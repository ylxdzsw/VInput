use vip::shit;
use std::fs::File;

mod util;

fn main() {
    let f = File::open("list.txt").unwrap();
    let q: Vec<f32> = util::load_list(f);
    println!("{:?}", q[2])
}