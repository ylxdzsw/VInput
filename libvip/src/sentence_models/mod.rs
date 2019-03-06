mod hmm;

use crate::dict::Encoding;
pub use hmm::HMM;

pub trait SentenceModel: Clone {
    type Dict;
    fn load(path: &str) -> Self::Dict;
    fn new(enc: &Encoding, dict: &Self::Dict) -> Self;
    fn append(&mut self, enc: &Encoding, d: &Self::Dict, c: u8);
    fn append_all<T: Iterator<Item = u8>>(&mut self, enc: &Encoding, d: &Self::Dict, tokens: T) {
        for c in tokens {
            self.append(enc, d, c)
        }
    }
    fn set_history<T: Iterator<Item = u16>>(&mut self, _hist: T);
    fn get_sentence(&self, enc: &Encoding, d: &Self::Dict) -> Option<Vec<u16>>;
}
