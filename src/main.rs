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

enum Dest{ Terminal, Network(u16) }

fn init_dest() -> Dest {
    let x = std::env::args().skip(1).next();
    if let Some(x) = x {
        if x == "-" || x == "" {
            Dest::Terminal
        } else if let Ok(x) = x.parse() {
            Dest::Network(x)
        } else {
            panic!()
        }
    } else {
        Dest::Terminal
    }
}

fn send(dest: &Dest, msg: &str) {
    match dest {
        Dest::Terminal => {
            write!(stderr(), "\r\n{}{}\r\n", clear::CurrentLine, msg).unwrap();
        }
        Dest::Network(port) => {
            std::net::UdpSocket::bind("0.0.0.0:0").unwrap().send_to(msg.as_bytes(), format!("127.0.0.1:{}", port)).unwrap();
        }
    }
}

fn main() {
    let dest = init_dest();
    let enc = dict::Encoding::load("data/pinyin", "data/id", "data/freq").unwrap();

    let mut buf: Vec<u8> = Vec::new();
    let mut candidate: Vec<String> = vec![];
    let mut page = 0;
    let mut stderr = stderr().into_raw_mode().unwrap();
    let mut dirty = true;

    for c in stdin().keys() {
        match c.unwrap() {
            Key::Char(c) => match c {
                'a'...'z' => { buf.push(c as u8); dirty = true }
                '1'...'9' | ' ' | '\n' => {
                    let i = if c == '\n' || c == ' ' {
                        1
                    } else {
                        c as usize - '0' as usize
                    };
                    if let Some(x) = candidate.get(10 * page + i - 1) {
                        send(&dest, x);
                        buf.clear();
                        dirty = true
                    }
                }
                _ => continue,
            }
            Key::Backspace => { buf.pop(); dirty = true }
            Key::Ctrl('c') | Key::Ctrl('d') => break,
            Key::PageUp => { if page > 0 { page -= 1 } }
            Key::PageDown => { if 10 * page + 10 <= candidate.len() { page += 1 } }
            _ => continue,
        }

        if dirty {
            candidate = enc.prefix_perfect(&buf).into_iter().map(|x| format!("{} {}", enc.id[(x-1) as usize], enc.freq[(x-1) as usize].exp())).collect();
            page = 0;
            dirty = false
        }
        let view = &candidate[10*page..std::cmp::min(candidate.len(), 10*page+10)];
        render(&mut stderr, &buf.iter().map(|x| *x as char).collect::<Vec<_>>(), view)
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
