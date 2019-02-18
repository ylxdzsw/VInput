use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;
use std::ops::{Index, IndexMut};
use std::collections::HashMap;
use std::cmp;

// TODO: reform it to 1. Encoding containing an encoding, id list and unigram, 2. Skip4 containing Mmap of skip4
// the rationale of include unigram to encoding is it is required to sort the perfect encoding matches: all language models only consider frequent chars and the most infrequent ones can only be input with the fallback perfect encoding

pub fn load_list<R: Read, T: FromStr>(io: R) -> Vec<T> where <T as FromStr>::Err: std::fmt::Debug{
    BufReader::new(io)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect()
}

// TODO: we wasted a column since the id 0 word only appears as the first element
// TODO: reform the table as a N+4 x N rather than the current N+1 x N+1
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Table<T> {
    N: usize,
    data: Box<[T]>
}

impl<T> Index<(usize, usize)> for Table<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &T {
        &self.data[x * self.N + y]
    }
}

impl<T> IndexMut<(usize, usize)> for Table<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
        &mut self.data[x * self.N + y]
    }
}

impl<T: Clone + FromStr> Table<T> where <T as FromStr>::Err: std::fmt::Debug {
    #[allow(non_snake_case)]
    fn new(x: T, N: usize) -> Table<T> {
        let data = vec![x; N*N].into_boxed_slice();
        Table{ N, data }
    }

    fn load<R: Read>(&mut self, io: R) {
        for line in BufReader::new(io).lines().map(|x| x.unwrap()) {
            let mut iter = line.split(' ');
            let x: usize = iter.next().unwrap().parse().unwrap();
            let y: usize = iter.next().unwrap().parse().unwrap();
            let v: T = iter.next().unwrap().parse().unwrap();
            assert_eq!(None, iter.next());
            self[(x, y)] = v;
        }
    }
}

// TODO: it might be better stored in a Trie tree
pub struct Encoding {
    max_len: usize,
    map: HashMap<Vec<u8>, Vec<u16>>,
    id: Vec<char>
}

impl Encoding {
    // fn perfect_perfect(&self, x: &[char]) -> Vec<usize> {
    //     self.code.get(x).map_or(Vec::new(), |x| x.clone())
    // }

    // fn prefix_perfect(&self, x: &[char]) -> Vec<usize> {
    //     (0..self.max_len).map(|i| self.perfect_perfect(&x[..=i])).flatten().collect()
    // }

    // fn perfect_prefix(&self, _x: &[char]) -> Vec<usize> {
    //     unimplemented!()
    // }
}

// pub fn load_encoding<R: Read>(io: R) -> Encoding {
//     let mut enc = HashMap::new();
//     let mut max_len = 0;
//     for line in BufReader::new(io).lines().map(|x| x.unwrap()) {
//         let mut iter = line.split(' ');
//         let k: Vec<char> = iter.next().unwrap().chars().collect();
//         let v = iter.map(|x| x.parse().unwrap()).collect();
//         max_len = cmp::max(max_len, k.len());
//         enc.insert(k, v);
//     }
//     Encoding { max_len, map: enc }
// }
