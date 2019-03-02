use super::WordModel;
use crate::dict::{Skip4};

pub struct VKey {

}

impl WordModel for VKey {
    type Dict = Skip4;
    fn load(data: &str) -> Skip4 {
        Skip4::load(data).unwrap()
    }
    fn new<T: Iterator<Item = char>>(x: T) -> Self {
        unimplemented!()
    }
    fn get_words(&self) -> Vec<(i32, &str)> {
        unimplemented!()
    }
    fn append(self, c: char) {
        unimplemented!()
    }
}