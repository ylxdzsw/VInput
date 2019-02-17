#![allow(dead_code, unused_imports)]

extern crate termion;

use termion::cursor;
use termion::clear;
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use vip::shit;
use std::fs::File;
use std::io::{Write, stdin, stderr};

mod dict;

fn main() {
    let f = File::open("data/test.txt").unwrap();
    let q: Vec<String> = util::load_list(f);

    let mut buf: Vec<char> = Vec::new();
    let mut stderr = stderr().into_raw_mode().unwrap();

    for c in stdin().keys() {
        match c.unwrap() {
            Key::Char(c) => match c {
                'a'...'z' => { buf.push(c); }
                '1'...'9' | ' ' | '\n' => {
                    let i = if c == '\n' || c == ' ' {
                        1
                    } else {
                        c as usize - '0' as usize
                    };
                    write!(stderr, "\r\n{}{}\r\n", clear::CurrentLine, q[i]).unwrap();
                    buf.clear();
                }
                _ => continue,
            }
            Key::Backspace => { buf.pop(); },
            Key::Ctrl('c') | Key::Ctrl('d') => break,
            _ => continue,
        }

        render(&mut stderr, &buf, &q[..10])
    }
}

fn render(io: &mut Write, buf: &[char], menu: &[String]) {
    // 1. clear and move cursor to the start
    write!(io, "\r{}", clear::AfterCursor).unwrap();
    // 2. render the input line
    write!(io, ">{}\r\n", buf.iter().collect::<String>()).unwrap();
    // 3. render the candidate list
    for str in menu {
        write!(io, "{}\r\n", str).unwrap();
    }
    // 4. move the cursor back
    write!(io, "{}\r{}", cursor::Up(1+menu.len() as u16), cursor::Right(1+buf.len() as u16)).unwrap();
}
