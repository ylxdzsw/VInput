extern crate libvip;

use libvip::shit;
use std::env;
use std::cell::RefCell;
use std::thread;
use std::time::Duration;

use std::sync::Mutex;

fn main() {
    let a = Mutex::new(2);
    let b = a.lock().unwrap();
    print!("{:?}" ,shit())
}