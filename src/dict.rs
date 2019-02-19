use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;
use std::ops::{Index, IndexMut};
use std::collections::HashMap;
use std::cmp;

mod mmaped_array {
    use std::fs::File;
    use memmap::Mmap;

    #[derive(Debug)]
    pub struct MmapedArray<T: Copy> {
        file: File, // Mmap is valid only when the original file object alive
        data: memmap::Mmap
    }

    impl<T: Copy> MmapedArray<T> {
        pub fn new(file: &str) -> std::io::Result<MmapedArray<T>> {
            let file = File::open(file)?;
            let data = unsafe { Mmap::map(&file)? };
            Ok(MmapedArray { file, data: data })
        }
    }

    impl<T> std::ops::Deref for MmapedArray<T> {
        type Target = [T];

        fn deref(&self) -> &[T] {
            unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const T, self.data.len() / std::mem::size_of::<T>() ) }
        }
    }
}

use mmaped_array::MmapedArray;

// TODO: we wasted a column since the id 0 word only appears as the first element
// TODO: reform the table as a N+4 x N rather than the current N+1 x N+1
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Skip4 {
    N: usize,
    data: MmapedArray<f32>
}

impl Skip4 {
    fn load(N: usize, path: &str) -> Skip4 {
        let data = MmapedArray::new(path).unwrap();
        assert_eq!((N+1)*(N+1), data.len());
        Skip4 { N, data }
    }
}

impl Index<(usize, usize)> for Skip4 {
    type Output = (f32, f32, f32, f32);

    fn index(&self, (x, y): (usize, usize)) -> &f32 {
        &self.data[x * self.N + y]
    }
}

impl IndexMut<(usize, usize)> for Skip4 {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut f32 {
        &mut self.data[x * self.N + y]
    }
}

// impl<T: Clone + FromStr> Skip4<T> where <T as FromStr>::Err: std::fmt::Debug {
//     #[allow(non_snake_case)]
//     fn new(x: T, N: usize) -> Skip4<T> {
//         let data = vec![x; N*N].into_boxed_slice();
//         Skip4{ N, data }
//     }

//     fn load<R: Read>(&mut self, io: R) {
//         for line in BufReader::new(io).lines().map(|x| x.unwrap()) {
//             let mut iter = line.split(' ');
//             let x: usize = iter.next().unwrap().parse().unwrap();
//             let y: usize = iter.next().unwrap().parse().unwrap();
//             let v: T = iter.next().unwrap().parse().unwrap();
//             assert_eq!(None, iter.next());
//             self[(x, y)] = v;
//         }
//     }
// }

pub struct Encoding {
    max_len: usize,
    map: HashMap<Vec<u8>, Vec<u16>>, // TODO: it might be better stored in a Trie tree
    id: Vec<char>,
    freq: Vec<f32> // the rationale of include unigram to encoding is it is required to sort the perfect encoding matches: all language models only consider frequent chars and the most infrequent ones can only be input with the fallback perfect encoding
}

impl Encoding {
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
