mod hmm;

use crate::dict::Encoding;
pub use hmm::HMM;

pub trait SentenceModel<'enc, 'd>: Clone {
    type Dict;
    fn load(data: &str) -> Self::Dict;
    fn new<T: Iterator<Item = char>>(x: T, enc: &'enc Encoding, d: &'d Self::Dict) -> Self;
    fn append(&mut self, c: char);
    fn get_sentence(&self) -> Option<&str>;
}
