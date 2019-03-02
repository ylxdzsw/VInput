mod vkey;

pub use vkey::*;

pub trait WordModel {
    type Dict;
    fn load(data: &str) -> Self::Dict;
    fn new<T: Iterator<Item = char>>(x: T) -> Self;
    fn get_words(&self) -> Vec<(i32, &str)>; // token length and word
    fn append(self, c: char);
}