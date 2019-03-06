use crate::sentence_models::SentenceModel;
use crate::word_models::WordModel;
use crate::dict::Encoding;
use crate::utils::*;

const CHECKPOINT_STEP: usize = 3;

pub struct Context<SM: SentenceModel, WM: WordModel> {
    fuck: std::marker::PhantomData<(SM, WM)>,
    input: Box<[u8]>,
    hist: Box<[u16]>,
    enc: Encoding,
    sm: Option<SM>,
    sms: Vec<SM>,
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
            sm: None,
            sms: vec![],
            smdata: SM::load(data),
            wmdata: WM::load(data)
        }
    }

    pub fn get_candidates(&mut self) -> Vec<(usize, String)> {
        // TODO: keep only the one consuming most tokens for each candidate
        let sentence = self.sm.as_ref().and_then(|x| x.get_sentence(&self.enc, &self.smdata));
        let mut all = if let Some(sentence) = sentence {
            vec![(self.input.len(), sentence.iter().map(|x| self.enc.id[*x as usize - 1]).collect())]
        } else {
            vec![]
        };

        all.append(&mut self.get_raw_matches());
        all
    }

    pub fn set_input(&mut self, input: &[u8]) {
        let mut i = 0;
        while i < input.len() && i < self.input.len() && input[i] == self.input[i] {
            i += 1;
        }

        self.sms.truncate(i / CHECKPOINT_STEP);

        let mut sm = self.sms.last().map(|x| x.clone()).unwrap_or(SM::new(&self.enc, &self.smdata).apply_owned(|x| x.set_history(self.hist.iter().map(|x| *x))));

        for i in self.sms.len()*CHECKPOINT_STEP..input.len() { // todo: perform updates in batch
            sm.append(&self.enc, &self.smdata, input[i]);
            if (i+1) % CHECKPOINT_STEP == 0 {
                self.sms.push(sm.clone());
            }
        }

        self.sm = Some(sm);
        self.input = Box::from(input);
    }

    pub fn set_hist(&mut self, hist: &[char]) {
        self.hist = hist.iter().map(|x| self.enc.code[x]).collect();
        self.sms.clear();
        self.sm = None;
    }

    fn get_raw_matches(&self) -> Vec<(usize, String)> {
        let mut matches = self.enc.prefix_prefix(&self.input);
        // TODO: just sort by id?
        matches.sort_by(|(l1, x1), (l2, x2)| l1.cmp(l2).then(self.enc.freq[*x1 as usize - 1].partial_cmp(&self.enc.freq[*x2 as usize - 1]).unwrap()).reverse());
        matches.into_iter().map(|(l, x)| (l, self.enc.id[x as usize - 1].to_string())).collect()
    }
}
