#![allow(dead_code, unused_imports)]

use termion::cursor;
use termion::clear;
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use std::fs::File;
use std::io::{Write, stdin, stderr};
use std::ffi::CStr;
use vip;
use vip::utils::*;

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

// fn get_tokens() -> Option<Vec<u8>> {
//     Some(std::env::args().skip(2).next()?.into_bytes())
// }

fn send(dest: &Dest, msg: &str) {
    match dest {
        Dest::Terminal => write!(stderr(), "\r\n{}{}\r\n", clear::CurrentLine, msg).unwrap(),
        Dest::Network(port) => std::net::UdpSocket::bind("0.0.0.0:0").unwrap().send_to(msg.as_bytes(), format!("127.0.0.1:{}", port)).ignore()
    }
}

fn send_raw(dest: &Dest, c: char) {
    match dest {
        Dest::Terminal => (),
        Dest::Network(_) => send(dest, &c.to_string())
    }
}

// control contains two bytes: it starts with \0, then followed with
// 1: up, 2: right, 3: down, 4: left, 5: backspace, 6: delete, 7: home, 8: end
fn control(dest: &Dest, key: Key) {
    let code: u8 = match key {
        Key::Up => 1, Key::Right => 2, Key::Down => 3, Key::Left => 4,
        Key::Backspace => 5, Key::Delete => 6, Key::Home => 7, Key::End => 8,
        _ => unreachable!()
    };
    match dest {
        Dest::Terminal => (),
        Dest::Network(_) => send(dest, &format!("\0{}", code as char))
    }
}

fn main() {
    let dest = init_dest();
    let ctx = vip::init("data\0".as_ptr() as *mut i8);

    let mut buf: Vec<u8> = Vec::new();
    let mut candidate: Vec<(usize, String)> = vec![];
    let mut page = 0;
    let mut stderr = stderr().into_raw_mode().unwrap();
    let mut dirty = true; // indicate if buf changed and should update the candidates
    let mut hist: Vec<u8> = vec![0];

    // for c in "ka'ka1".chars().map(|x| Some(Key::Char(x))) { //stdin().keys() {
    for c in stdin().keys() {
        match c.unwrap() {
            Key::Char(c) => match c {
                'a'...'z' => { buf.push(c as u8); dirty = true }
                '0'...'9' | ' ' => {
                    let i = match c {
                        ' ' => 1,
                        '0' => 10,
                        _ => c as usize - '0' as usize
                    };

                    if buf.is_empty() {
                        send_raw(&dest, c)
                    } else if let Some((len, x)) = candidate.get(10 * page + i - 1) {
                        send(&dest, x);
                        buf.drain(0..*len);
                        if buf.len() > 0 && buf[0] == '\'' as u8 {
                            buf.remove(0); // ensure no leading diaeresis after selection
                        }
                        append_hist(&mut hist, x.as_bytes());
                        vip::set_hist(ctx, hist.as_ptr() as *const i8);
                        dirty = true
                    }
                }
                '\'' => if buf.is_empty() {
                    send_raw(&dest, c)
                } else {
                    buf.push(c as u8);
                    dirty = true
                }
                '\n' => if buf.is_empty() {
                    send_raw(&dest, c)
                } else {
                    send(&dest, &String::from_utf8_lossy(&buf));
                    buf.clear();
                    dirty = true;
                }
                _ => send_raw(&dest, c),
            }
            key @ Key::Backspace => if let Some(_) = buf.pop() {
                dirty = true
            } else {
                control(&dest, key)
            }
            Key::Ctrl('c') | Key::Ctrl('d') => break,
            Key::PageUp => if page > 0 { page -= 1 }
            Key::PageDown => if 10 * page + 10 <= candidate.len() { page += 1 }
            Key::Esc => { buf.clear(); dirty = true }
            key@Key::Up | key@Key::Right | key@Key::Down | key@Key::Left | key@Key::Home | key@Key::End | key@Key::Delete => control(&dest, key),
            _ => continue,
        }

        if dirty {
            if buf.len() == 0 {
                hist.clear();
                hist.push(0);
                vip::set_hist(ctx, hist.as_ptr() as *const i8);
            }
            let mut x = buf.clone();
            x.push(0);
            vip::set_input(ctx, x.as_ptr() as *const i8);
            let ptr = vip::get_candidates(ctx);
            let raw = unsafe { CStr::from_ptr(ptr).to_str().unwrap() };
            candidate = raw.lines().map(|line| {
                let mut parts = line.split(' ');
                let len: usize = parts.next().unwrap().parse().unwrap();
                let content = parts.next().unwrap().to_owned();
                (len, content)
            }).collect();
            vip::free_candidates(ptr);
            page = 0;
            dirty = false
        }
        let view = &candidate[10*page..std::cmp::min(candidate.len(), 10*page+10)];
        render(&mut stderr, &buf.iter().map(|x| *x as char).collect::<Vec<_>>(), view)
    }
}

fn render(io: &mut Write, buf: &[char], menu: &[(usize, String)]) {
    // 1. clear and move cursor to the start
    write!(io, "\r{}", clear::AfterCursor).unwrap();
    // 2. render the input line
    write!(io, ">{}\r\n", buf.iter().collect::<String>()).unwrap();
    // 3. render the candidate list
    for (_, s) in menu {
        write!(io, "{}\r\n", s).unwrap();
    }
    // 4. move the cursor back
    write!(io, "{}\r{}", cursor::Up(1+menu.len() as u16), cursor::Right(1+buf.len() as u16)).unwrap();
}

fn append_hist(hist: &mut Vec<u8>, new: &[u8]) { // ensure terminating zero
    hist.pop();
    for c in new {
        hist.push(*c)
    }
    hist.push(0)
}