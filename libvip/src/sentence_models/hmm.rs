use super::SentenceModel;
use crate::dict::{Skip4};

pub struct HMM {

}

impl SentenceModel for HMM {
    type Dict = Skip4;
    fn load(data: &str) -> Skip4 {
        Skip4::load(data).unwrap()
    }
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