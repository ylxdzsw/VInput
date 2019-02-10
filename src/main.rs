use vip::shit;

use std::sync::Mutex;

fn main() {
    let a = Mutex::new(2);
    let b = a.lock().unwrap();
    print!("{:?}" ,shit())
}