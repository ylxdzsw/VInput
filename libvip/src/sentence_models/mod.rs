mod hmm;

use crate::dict::Encoding;
pub use hmm::HMM;

pub trait SentenceModel: Clone {
    type Dict;
    fn load(path: &str) -> Self::Dict;
    fn new<T: Iterator<Item = u8>>(x: T, enc: &Encoding, d: &Self::Dict) -> Self;
    fn append(&mut self, enc: &Encoding, d: &Self::Dict, c: u8);
    fn get_sentence(&self, enc: &Encoding, d: &Self::Dict) -> Option<Vec<u16>>;
}
