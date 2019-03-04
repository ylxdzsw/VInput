mod hmm;

use crate::dict::Encoding;
pub use hmm::HMM;

pub trait SentenceModel<'enc, 'd>: Clone {
    type Dict;
    fn load(path: &str) -> Self::Dict;
    fn new<T: Iterator<Item = u8>>(x: T, enc: &'enc Encoding, d: &'d Self::Dict) -> Self;
    fn append(&mut self, c: u8);
    fn get_sentence(&self) -> Option<Vec<u16>>;
}
