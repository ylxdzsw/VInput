use super::WordModel;
use crate::dict;

pub struct VKey {

}

impl WordModel for VKey {
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