use crate::sentence_models::SentenceModel;
use crate::word_models::WordModel;
use crate::dict::Encoding;

pub struct Context<SM: SentenceModel, WM: WordModel> {
    fuck: std::marker::PhantomData<(SM, WM)>,
    input: Box<[u8]>,
    hist: Box<[u16]>,
    enc: Encoding,
    smdata: SM::Dict,
    wmdata: WM::Dict
}

impl<SM: SentenceModel, WM: WordModel> Context<SM, WM> {
    pub fn new(data: &str) -> Self {
        Self {
            fuck: std::marker::PhantomData,
            input: Box::new([]),
            hist: Box::new([]),
            enc: Encoding::load(data).unwrap(),
            smdata: SM::load(data),
            wmdata: WM::load(data)
        }
    }

    pub fn get_candidates(&mut self) -> Vec<(usize, String)> {
        self.get_raw_matches()
    }

    pub fn set_input(&mut self, input: &[u8]) {
        self.input = Box::from(input);
    }

    pub fn set_hist(&mut self, hist: &[char]) {
        self.hist = hist.iter().map(|x| self.enc.code[x]).collect()
    }

    fn get_raw_matches(&self) -> Vec<(usize, String)> {
        let mut matches = self.enc.prefix_prefix(&self.input);
        matches.sort_by(|(l1, x1), (l2, x2)| l1.cmp(l2).then(self.enc.freq[*x1 as usize - 1].partial_cmp(&self.enc.freq[*x2 as usize - 1]).unwrap()).reverse());
        matches.iter().map(|(l, x)| (*l, self.enc.id[*x as usize - 1].to_string())).collect()
    }
}
