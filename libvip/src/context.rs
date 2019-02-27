use crate::sentence_models::SentenceModel;
use crate::word_models::WordModel;

pub struct Context<SM: SentenceModel, WM: WordModel> {
    fuck: std::marker::PhantomData<(SM, WM)>
}

impl<SM: SentenceModel, WM: WordModel> Context<SM, WM> {
    pub fn new() -> Self {
        Self { fuck: std::marker::PhantomData }
    }

    pub fn get_candidates(&mut self) -> Vec<(usize, String)> {
        unimplemented!()
    }

    pub fn set_input(&mut self, input: &[u8]) {
        unimplemented!()
    }

    pub fn set_hist(&mut self, hist: &[char]) {
        unimplemented!()
    }
}
