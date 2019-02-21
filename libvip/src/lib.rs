#![allow(dead_code, unused_imports)]

mod sentence_models;
mod word_models;
mod context;

mod dict;

#[no_mangle]
pub extern fn shit() -> i32 {
    return 3;
}