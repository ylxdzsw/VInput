use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

pub fn load_list<R: Read, T: FromStr>(io: R) -> Vec<T> where <T as FromStr>::Err: std::fmt::Debug{
    BufReader::new(io)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect()
}
