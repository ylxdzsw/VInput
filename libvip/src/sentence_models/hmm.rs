use std::rc::Rc;

use super::SentenceModel;
use crate::dict::{Encoding, Skip4, FREQ_THRESHOLD};

// TODO: prefer longer sequence

#[derive(Clone)]
pub struct HMM<'enc, 'd> {
    token: Vec<u8>, // only the last N tokens, N is the max_len of encoding
    state: Vec<Rc<State>>, // only the "live" states
    len: usize, // current length
    enc: &'enc Encoding,
    dict: &'d Skip4,
}

impl<'enc, 'd> SentenceModel<'enc, 'd> for HMM<'enc, 'd> {
    type Dict = Skip4;
    
    fn load(path: &str) -> Skip4 {
        Skip4::load(path).unwrap()
    }

    fn new<T: Iterator<Item = u8>>(x: T, enc: &'enc Encoding, dict: &'d Skip4) -> Self {
        let mut s = HMM { token: Vec::with_capacity(enc.max_len), len: 0, state: vec![], dict, enc };
        for c in x {
            s.append(c);
        }
        s
    }
    
    // each state either do not move, or move to the last char
    fn append(&mut self, c: u8) {
        if !self.token.is_empty() {
            self.token.remove(0);
        }

        self.len += 1;
        self.token.push(c);

        let cut = self.len as u16 - self.enc.max_len as u16;
        self.state.retain(|s| s.total_len >= cut);

        let mut new_states = vec![];
        for state in &self.state {
            let len = self.len - state.total_len as usize;
            new_states.append(&mut branch(state, self.dict, self.enc, &self.token[self.token.len()-len..]))
        }
        for candidate in new_states {
            insert_pool(&mut self.state, candidate)
        }

        if self.len <= self.enc.max_len { // first generation
            for id in self.enc.prefix_exact(&self.token) {
                insert_pool(&mut self.state, Rc::new(State {
                    total_len: self.len as u16,
                    total_p: p(self.dict, id as u16, 0, 0, 0, 0),
                    len: self.len as u16,
                    id: id as u16,
                    parent: None
                }))
            }
        }
    }
    
    fn get_sentence(&self) -> Option<Vec<u16>> {
        unimplemented!()
    }
}

#[derive(Clone, Default)]
struct State {
    total_len: u16,
    total_p: f32,
    len: u16,
    id: u16,
    parent: Option<Rc<State>> // None is treated as Root node; nodes with parent = None are the first generation
}

fn branch(origin: &Rc<State>, d: &Skip4, enc: &Encoding, tokens: &[u8]) -> Vec<Rc<State>> {
    let mut h = [origin.id, 0, 0, 0];
    let mut parent = &origin.parent;
    for i in 0..3 {
        if let Some(x) = parent {
            h[i+1] = x.id;
            parent = &x.parent;
        } else {
            break
        }
    }

    let [h1, h2, h3, h4] = h;

    // TODO: give reward to prefix length
    enc.prefix_exact(tokens).iter().map(|x| Rc::new(State {
        total_len: origin.total_len + tokens.len() as u16,
        total_p: origin.total_p + p(d, *x, h1, h2, h3, h4),
        len: tokens.len() as u16,
        id: *x,
        parent: Some(origin.clone())
    })).collect()
}

fn p(d: &Skip4, x: u16, h1: u16, h2: u16, h3: u16, h4: u16) -> f32 {
    let (a1, a2, a3, a4) = (d[(h1, x)][0].exp(), d[(h2, x)][1].exp(), d[(h3, x)][2].exp(), d[(h4, x)][3].exp());
    // TODO: linear interpolation is bad. try something like softmax?
    (0.6 * a1 + 0.2 * a2 + 0.1 * a3 + 0.1 * a4).ln()
}

fn insert_pool(pool: &mut Vec<Rc<State>>, candidate: Rc<State>) {
    unimplemented!()
}
