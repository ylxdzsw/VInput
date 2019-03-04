mod hmm;

use crate::dict::Encoding;
pub use hmm::HMM;

pub trait SentenceModel<'a, 'b>: Clone {
    type Dict;
    fn load(data: &str) -> Self::Dict;
    fn new<T: Iterator<Item = char>>(x: T, d: &'a Self::Dict, enc: &'b Encoding) -> Self;
    fn append(&mut self, c: char);
    fn get_sentence(&self) -> Option<&str>;
}
