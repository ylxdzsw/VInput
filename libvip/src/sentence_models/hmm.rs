use std::rc::Rc;
use std::collections::HashMap;
use std::cmp;

use super::SentenceModel;
use crate::dict::{Encoding, Skip4, FREQ_THRESHOLD};

#[derive(Clone)]
pub struct HMM {
    tokens: Vec<u8>, // only the last N tokens, N is the max_len of encoding
    states: Vec<Vec<([u16; 4], Rc<State>)>>, // each element is a "generation", i.e., states that have the same .len
    len: u16, // current length
}

impl SentenceModel for HMM {
    type Dict = Skip4;

    fn load(path: &str) -> Skip4 {
        Skip4::load(path).unwrap()
    }

    fn new(enc: &Encoding, _dict: &Skip4) -> Self {
        HMM { tokens: Vec::with_capacity(enc.max_len), len: 0, states: vec![] }
    }

    // each state either do not move, or move to the last char
    fn append(&mut self, enc: &Encoding, dict: &Skip4, c: u8) {
        self.len += 1;

        // 1. rotate the buffers
        if self.tokens.len() == enc.max_len {
            self.tokens[0] = c;
            self.tokens.rotate_left(1);
        } else {
            self.tokens.push(c);
        }

        // 2. generate a new generation
        let mut new_states: HashMap<[u16; 4], Rc<State>> = HashMap::new();
        for (i, gen) in self.states.iter().enumerate() {
            let tlen = cmp::min(self.len as usize - 1, enc.max_len) - i;
            let mut appendable = enc.prefix_exact(&self.tokens[self.tokens.len() - tlen..]);
            appendable.retain(|x| *x <= FREQ_THRESHOLD as u16);
            for (key, state) in gen {
                for x in &appendable {
                    let mut nk = key.clone();
                    nk[0] = *x;
                    nk.rotate_left(1);

                    let ns = Rc::new(State {
                        total_len: self.len,
                        total_p: state.total_p + p(dict, *x, key),
                        len: tlen as u16,
                        id: *x,
                        parent: Some(state.clone())
                    });
                    new_states.entry(nk).and_modify(|s| if ns.total_p > s.total_p { *s = ns.clone() }).or_insert(ns); // the unnecessary clone is because Rust think `ns` is moved twice
                }
            }
        }
        if self.len <= enc.max_len as u16 { // first several generations
            for id in enc.prefix_exact(&self.tokens).into_iter().filter(|x| *x <= FREQ_THRESHOLD as u16) {
                let ns = Rc::new(State {
                    total_len: self.len,
                    total_p: p(dict, id, &[0,0,0,0]),
                    len: self.len as u16,
                    id: id,
                    parent: None
                });
                new_states.entry([0,0,0,id]).and_modify(|s| if ns.total_p > s.total_p { *s = ns.clone() }).or_insert(ns);
            }
        }

        let mut new_states: Vec<_> = new_states.into_iter().collect();
        new_states.sort_by(|(_, x), (_, y)| x.total_p.partial_cmp(&y.total_p).unwrap().reverse());
        new_states.truncate(32);

        // 4. rotate generations
        if self.states.len() == enc.max_len {
            self.states[0] = new_states;
            self.states.rotate_left(1);
        } else {
            self.states.push(new_states);
        }
    }

    fn get_sentence(&self, _enc: &Encoding, _dict: &Skip4) -> Option<Vec<u16>> {
        let mut best = None;
        for (_, state) in self.states.last()? {
            match &best {
                None => best = Some(state),
                Some(s) if s.total_p < state.total_p => best = Some(state),
                _ => ()
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

fn p(d: &Skip4, x: u16, h: &[u16; 4]) -> f32 {
    let (a1, a2, a3, a4) = (d[(h[3], x)][0].exp(), d[(h[2], x)][1].exp(), d[(h[1], x)][2].exp(), d[(h[0], x)][3].exp()); // Rust have no .map for arrays?
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