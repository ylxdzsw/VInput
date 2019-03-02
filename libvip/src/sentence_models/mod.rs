mod hmm;

pub use hmm::HMM;

pub trait SentenceModel {
    type Dict;
    fn load(data: &str) -> Self::Dict;
    fn new<T: Iterator<Item = char>>(x: T) -> Self;
    fn append(&mut self, c: char);
    fn get_sentence(&self) -> Option<&str>;
}
