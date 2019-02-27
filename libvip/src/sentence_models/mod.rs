mod hmm;

pub use hmm::HMM;

pub trait SentenceModel {
    fn new<T: Iterator<Item = char>>(x: T) -> Self;
    fn append(&mut self, c: char);
    fn get_sentence(&self) -> Option<&str>;
}
