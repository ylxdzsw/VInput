#![allow(dead_code, unused_imports)]

mod sentence_models;
mod word_models;
mod context;
mod dict;

use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int};
use std::fmt::Write;

type Context<'enc, 'sd> = context::Context<'enc, 'sd, sentence_models::HMM<'enc, 'sd>, word_models::VKey>;

/// creat a new VInput instance, returns a pointer to be used as the first argument of all other functions
#[no_mangle]
pub extern fn init(data: *const c_char) -> *mut Context<'static, 'static> {
    let data = unsafe { CStr::from_ptr(data) };
    Box::into_raw(Box::new(Context::new(data.to_str().unwrap())))
}

/// destroy and deallowcate an VInput instance.
#[no_mangle]
pub extern fn destroy(ctx: *mut Context) {
    unsafe { Box::from_raw(ctx) };
}

/// get the current candidates list in order
/// the result is a multi-line UTF8 string terminated with \0
/// each line starts with a number indicating how many input tokens are consumed in this candidate
/// followed with a space and then the actual candidate string
#[no_mangle]
pub extern fn get_candidates(ctx: *mut Context) -> *mut c_char { // todo: is it nessicery to be mutable? I did this because the candidate list might be cached and the call to get it updates the cache
    let ctx = unsafe { &mut *ctx };
    let candidates = ctx.get_candidates();

    let mut s = String::new();
    for (n, c) in candidates {
        writeln!(s, "{} {}", n, c).unwrap()
    }

    CString::new(s).unwrap().into_raw()
}

/// free the memory of a candidates list
#[no_mangle]
pub extern fn free_candidates(candidates: *mut c_char) {
    unsafe { CString::from_raw(candidates) };
}

/// set the input sequence, the sequence should be a string terminated with \0 and contains only a-z in ASCII
/// successive calls will be optimized to use corresponding incremental method automatically (if possible)
#[no_mangle]
pub extern fn set_input(ctx: *mut Context, input: *const c_char) {
    let ctx = unsafe { &mut *ctx };
    let input = unsafe { CStr::from_ptr(input) };
    ctx.set_input(input.to_bytes())
}

/// set the history which is a \0 terminated UTF8 string contains characters before current cursor
/// the history should only contain characters that is outputed by previours call to get_candidates
/// setting history of unrecognized characters results in undefined behaviour (mostly segfault)
#[no_mangle]
pub extern fn set_hist(ctx: *mut Context, hist: *const c_char) {
    let ctx = unsafe { &mut *ctx };
    let hist = unsafe { CStr::from_ptr(hist) };
    ctx.set_hist(&hist.to_str().unwrap().chars().collect::<Vec<_>>())
}