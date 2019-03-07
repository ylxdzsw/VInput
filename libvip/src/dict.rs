use std::io::{self, BufRead, BufReader, Read};
use std::str::FromStr;
use std::fs::File;
use std::ops::{Index, IndexMut};
use std::collections::BTreeMap;
use std::cmp;

pub const FREQ_THRESHOLD: usize = 4095;

mod mmaped_array {
    use std::marker::PhantomData;
    use std::fs::File;
    use memmap::Mmap;

    #[derive(Debug)]
    pub struct MmapedArray<T: Copy> {
        file: File, // Mmap is valid only when the original file object alive
        data: memmap::Mmap,
        fuck: PhantomData<T>,
    }

    impl<T: Copy> MmapedArray<T> {
        pub fn new(file: &str) -> std::io::Result<MmapedArray<T>> {
            let file = File::open(file)?;
            let data = unsafe { Mmap::map(&file)? };
            Ok(MmapedArray { file, data: data, fuck: PhantomData })
        }
    }

    impl<T: Copy> std::ops::Deref for MmapedArray<T> {
        type Target = [T];

        fn deref(&self) -> &[T] {
            unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const T, self.data.len() / std::mem::size_of::<T>()) }
        }
    }
}

use mmaped_array::MmapedArray;

// TODO: we wasted a column since the id 0 word only appears as the first element
// TODO: reform the table as a N+4 x N rather than the current N+1 x N+1
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Skip4 {
    N: usize, // the array is N x N, where N is freq_threshold + 1
    data: MmapedArray<[f32; 4]>
}

impl Skip4 {
    #[allow(non_snake_case)]
    pub fn load(data: &str) -> io::Result<Skip4> {
        let N = FREQ_THRESHOLD + 1;
        let data = MmapedArray::new(&format!("{}/skip4", data))?;
        assert_eq!(N*N, data.len());
        Ok(Skip4{ N, data })
    }
}

impl<T: Into<usize>> Index<(T, T)> for Skip4 {
    type Output = [f32; 4];

    fn index(&self, (x, y): (T, T)) -> &[f32; 4] {
        &self.data[x.into() * self.N + y.into()]
    }
}

pub struct Encoding {
    pub max_len: usize, // maximum encoding length
    pub map: BTreeMap<Vec<u8>, Vec<u16>>, // TODO: it might be better stored in a Trie tree
    pub id: Vec<char>,
    pub code: BTreeMap<char, u16>, // reverse dict for id
    pub freq: Vec<f32>, // the rationale of include unigram to encoding is it is required to sort the exact encoding matches: all language models only consider frequent chars and the most infrequent ones can only be input with the fallback exact encoding
}

impl Encoding {
    pub fn load(data: &str) -> io::Result<Encoding> {
        let (max_len, map) = Self::load_map(&format!("{}/encoding", data))?;
        let id = Self::load_id(&format!("{}/char_id", data))?;
        let freq = Self::load_freq(&format!("{}/freq", data))?;
        let code = id.iter().enumerate().map(|(i, x)| (*x, i as u16 + 1)).collect();
        Ok(Encoding{ max_len, map, id, code, freq })
    }

    fn load_map(path: &str) -> io::Result<(usize, BTreeMap<Vec<u8>, Vec<u16>>)> {
        let mut enc = BTreeMap::new();
        let mut max_len = 0;
        for line in BufReader::new(File::open(path)?).lines().map(|x| x.unwrap()) {
            let mut iter = line.split(' ');
            let k: Vec<_> = iter.next().unwrap().chars().map(|x| x as u8).collect();
            let v = iter.map(|x| x.parse().unwrap()).collect();
            max_len = cmp::max(max_len, k.len());
            enc.insert(k, v);
        }
        Ok((max_len, enc))
    }

    fn load_id(path: &str) -> io::Result<Vec<char>> {
        Ok(BufReader::new(File::open(path)?).lines().map(|x| x.unwrap().chars().next().unwrap()).collect())
    }

    fn load_freq(path: &str) -> io::Result<Vec<f32>> {
        Ok(MmapedArray::new(path)?.to_vec())
    }

    pub fn exact_exact(&self, x: &[u8]) -> Vec<u16> {
        self.map.get(x).map_or(vec![], |x| x.clone())
    }

    pub fn prefix_exact(&self, x: &[u8]) -> Vec<u16> {
        // let x: Vec<_> = x.iter().map(|x| *x).filter(|x| *x != '\'' as u8).collect();
        if x.len() == 0 || x.len() > self.max_len { // TODO: is the logic for len=0 correct?
            return vec![]
        }

        let mut up = x.to_vec();
        *up.last_mut().unwrap() += 1;
        sort_and_dedup(self.map.range(x.to_vec()..up).map(|(_k, v)| v.clone()).flatten().collect())
    }

    pub fn exact_prefix(&self, x: &[u8]) -> Vec<(usize, u16)> {
        let len = cmp::min(x.len(), self.max_len);
        sort_and_dedup((0..len).map(|i| self.exact_exact(&x[..=i]).iter().map(|x| (i, *x)).collect::<Vec<_>>()).flatten().collect())
    }

    pub fn prefix_prefix(&self, x: &[u8]) -> Vec<(usize, u16)> { // TODO: too many? performance?
        let len = cmp::min(x.len(), self.max_len);
        sort_and_dedup((0..len).map(|i| self.prefix_exact(&x[..=i]).iter().map(|x| (i+1, *x)).collect::<Vec<_>>()).flatten().collect())
    }
}

fn sort_and_dedup<T: cmp::Ord>(mut x: Vec<T>) -> Vec<T> {
    x.sort();
    x.dedup();
    x
}
