#![allow(dead_code, unused_imports)]

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
    let enc = dict::Encoding::load("data/pinyin", "data/id", "data/freq").unwrap();

    let mut buf: Vec<u8> = Vec::new();
    let mut candidate: Vec<String> = vec![];
    let mut stderr = stderr().into_raw_mode().unwrap();

    for c in stdin().keys() {
        match c.unwrap() {
            Key::Char(c) => match c {
                'a'...'z' => { buf.push(c as u8); }
                '1'...'9' | ' ' | '\n' => {
                    let i = if c == '\n' || c == ' ' {
                        1
                    } else {
                        c as usize - '0' as usize
                    };
                    write!(stderr, "\r\n{}{}\r\n", clear::CurrentLine, candidate.get(i).cloned().unwrap_or_default()).unwrap();
                    buf.clear();
                }
                _ => continue,
            }
            Key::Backspace => { buf.pop(); },
            Key::Ctrl('c') | Key::Ctrl('d') => break,
            _ => continue,
        }

        candidate = enc.prefix_perfect(&buf).into_iter().take(10).map(|x| enc.id[(x-1) as usize].to_string()).collect();
        render(&mut stderr, &buf.iter().map(|x| *x as char).collect::<Vec<_>>(), &candidate)
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
