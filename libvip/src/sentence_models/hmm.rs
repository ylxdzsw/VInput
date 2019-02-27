use super::SentenceModel;
use crate::dict;

pub struct HMM {

}

impl SentenceModel for HMM {
    fn new<T: Iterator<Item = char>>(x: T) -> Self {
        unimplemented!()
    }
    fn append(&mut self, c: char) {
        unimplemented!()
    }
    fn get_sentence(&self) -> Option<&str> {
        unimplemented!()
    }
}