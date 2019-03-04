use std::rc::Rc;

use super::SentenceModel;
use crate::dict::{Encoding, Skip4};

// TODO: prefer longer sequance

#[derive(Clone)]
pub struct HMM<'a, 'b> {
    token: Vec<u8>, // only the last N tokens, N is the max_len of encoding
    state: Vec<State>, // only the "live" states
    dict: &'a Skip4,
    enc: &'b Encoding
}

impl<'a, 'b> SentenceModel<'a, 'b> for HMM<'a, 'b> {
    type Dict = Skip4;
    
    fn load(data: &str) -> Skip4 {
        Skip4::load(data).unwrap()
    }

    fn new<T: Iterator<Item = char>>(x: T, dict: &'a Skip4, enc: &'b Encoding) -> Self {
        let mut s = HMM { token: Vec::with_capacity(enc.max_len), state: vec![], dict, enc };
        for c in x {
            s.append(c);
        }
        s
    }
    
    // each state either do not move, or move to the last char
    fn append(&mut self, c: char) {
        unimplemented!()
        //self.token.remove(0)
    }
    
    fn get_sentence(&self) -> Option<&str> {
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

impl State {

}

fn p(d: &Skip4, x: u16, h1: u16, h2: u16, h3: u16, h4: u16) -> f32 {
    let (a1, a2, a3, a4) = (d[(h1, x)][0].exp(), d[(h2, x)][1].exp(), d[(h3, x)][2].exp(), d[(h4, x)][3].exp());
    // TODO: linear interpolation is bad. try something like softmax?
    (0.6 * a1 + 0.2 * a2 + 0.1 * a3 + 0.1 * a4).ln()
}
