use std::rc::Rc;

use super::SentenceModel;
use crate::dict::{Encoding, Skip4, FREQ_THRESHOLD};

// TODO: prefer longer sequence

#[derive(Clone)]
pub struct HMM<'enc, 'd> {
    tokens: Vec<u8>, // only the last N tokens, N is the max_len of encoding
    states: Vec<Rc<State>>, // only the "live" states
    len: u16, // current length
    enc: &'enc Encoding,
    dict: &'d Skip4,
}

impl<'enc, 'd> SentenceModel<'enc, 'd> for HMM<'enc, 'd> {
    type Dict = Skip4;
    
    fn load(path: &str) -> Skip4 {
        Skip4::load(path).unwrap()
    }

    fn new<T: Iterator<Item = u8>>(x: T, enc: &'enc Encoding, dict: &'d Skip4) -> Self {
        let mut s = HMM { tokens: Vec::with_capacity(enc.max_len), len: 0, states: vec![], dict, enc };
        for c in x {
            s.append(c);
        }
        s
    }
    
    // each state either do not move, or move to the last char
    fn append(&mut self, c: u8) {
        // 1. rotate the token buffer
        if self.tokens.len() == self.enc.max_len {
            self.tokens.remove(0);
        }

        self.len += 1;
        self.tokens.push(c);

        // 2. remove states that cannot reach last char
        let cut = self.len as u16 - self.enc.max_len as u16;
        self.states.retain(|s| s.total_len >= cut);

        // 3. derive new states from old ones such that the new states reach the last char
        let mut new_states = vec![];
        for state in &self.states {
            let len = self.len as usize - state.total_len as usize;
            new_states.append(&mut branch(state, self.dict, self.enc, &self.tokens[self.tokens.len()-len..]))
        }
        for candidate in new_states {
            insert_pool(&mut self.states, candidate)
        }

        // 4. make states from void at the beginning
        if self.len <= self.enc.max_len as u16 { // first generation
            for id in self.enc.prefix_exact(&self.tokens) {
                insert_pool(&mut self.states, Rc::new(State {
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
        let mut best = None;
        for state in &self.states {
            if state.total_len == self.len {
                match &best {
                    None => best = Some(state),
                    Some(s) if s.total_p < state.total_p => best = Some(state),
                    _ => ()
                }
            }
        }
        best.map(|x| trace_sequence(&x))
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

// insert candidate into pool, for the same states, leave only the one with largest total_p
fn insert_pool(pool: &mut Vec<Rc<State>>, candidate: Rc<State>) {
    for i in 0..pool.len() {
        let x = &pool[i];
        if x.len == candidate.len && similar(x, &candidate, 3) {
            if candidate.total_p > x.total_p {
                pool[i] = candidate;
            }
            return
        }
    }
    pool.push(candidate)
}

fn similar(a: &State, b: &State, c: u8) -> bool {
    if a.id != b.id { return false };
    if c == 0 { return true };
    if a.parent.is_none() && b.parent.is_none() { return true };
    if a.parent.is_none() != b.parent.is_none() { return false };
    similar(&a.parent.as_ref().unwrap(), &b.parent.as_ref().unwrap(), c - 1)
}

fn trace_sequence(s: &State) -> Vec<u16> {
    if let Some(p) = &s.parent {
        let mut x = trace_sequence(p);
        x.push(s.id);
        x
    } else {
        vec![s.id]
    }
}