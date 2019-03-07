use std::rc::Rc;
use std::collections::HashMap;
use std::cmp;

use super::SentenceModel;
use crate::dict::{Encoding, Skip4, FREQ_THRESHOLD};
use crate::utils::*;

const KEEP_N_BEST: usize = 256;

#[derive(Clone)]
pub struct HMM {
    tokens: Vec<u8>, // only the last N tokens, N is the max_len of encoding
    states: Vec<Vec<([u16; 4], Rc<State>)>>, // each element is a "generation", i.e., states that have the same .len
    hist: [u16; 4],
    len: u16, // current length
}

impl SentenceModel for HMM {
    type Dict = Skip4;

    fn load(path: &str) -> Skip4 {
        Skip4::load(path).unwrap()
    }

    fn new(enc: &Encoding, _dict: &Skip4) -> Self {
        HMM { tokens: Vec::with_capacity(enc.max_len), len: 0, hist: [0; 4], states: vec![] }
    }

    // each state either do not move, or move to the last char
    fn append(&mut self, enc: &Encoding, dict: &Skip4, c: u8) {
        self.len += 1;
        rotate_insert(&mut self.tokens, enc.max_len, c);

        if c == '\'' as u8 {
            // prefix cannot extend across diaeresises, thus all generations before can be dropped
            let last = self.states.last();
            if let Some(last) = last {
                let new_states = last.iter().map(|(k, s)| (k.clone(), Rc::new(State { total_len: s.total_len+1, ..<State as Clone>::clone(s) }))).collect(); // is the clone necessary? Not fully understand the Rust object spreading syntax.
                self.states = vec![new_states];
            } else { // it should not happen? empty states imply empty input, at which time the front-end will forward diaeresises as punctuations.
                self.states = vec![];
            }
        } else {
            let new_states = self.derive_new_generation(enc, dict);
            rotate_insert(&mut self.states, enc.max_len, new_states)
        }
    }

    fn set_history<T: Iterator<Item = u16>>(&mut self, hist: T) {
        let hist: Vec<_> = hist.collect();
        let n = hist.len();
        let mut buf = [0; 4];
        for i in 1..=4 {
            if n < i { break }
            buf[4-i] = hist[n-i];
        }
        self.hist = buf;
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

impl HMM {
    fn derive_new_generation(&self, enc: &Encoding, dict: &Skip4) -> Vec<([u16; 4], Rc<State>)> {
        let mut new_states: HashMap<[u16; 4], Rc<State>> = HashMap::new();
        for (i, gen) in self.states.iter().rev().enumerate() {
            let len = i + 1;
            let mut appendable = enc.prefix_exact(&self.tokens[self.tokens.len() - len..]);
            appendable.retain(|x| *x <= FREQ_THRESHOLD as u16);
            for (key, state) in gen {
                for x in &appendable {
                    let mut nk = key.clone();
                    nk[0] = *x;
                    nk.rotate_left(1);

                    let ns = Rc::new(State {
                        total_len: self.len,
                        total_p: state.total_p + p(dict, *x, key),
                        len: len as u16,
                        id: *x,
                        parent: Some(state.clone())
                    });
                    new_states.entry(nk).and_modify(|s| if ns.total_p > s.total_p { *s = ns.clone() }).or_insert(ns); // the unnecessary clone is because Rust think `ns` is moved twice
                }
            }
        }
        if self.len <= enc.max_len as u16 { // first several generations // TODO: leading daeresis? It should not happend since the front-end will directly forward leading daeresises. remenber to also remove leading daeresises on candidate selection.
            for id in enc.prefix_exact(&self.tokens).into_iter().filter(|x| *x <= FREQ_THRESHOLD as u16) {
                let ns = Rc::new(State {
                    total_len: self.len,
                    total_p: p(dict, id, &self.hist),
                    len: self.len as u16,
                    id: id,
                    parent: None
                });
                let key = [self.hist[1],self.hist[2],self.hist[3],id]; // does Rust has a good way to express that?
                new_states.entry(key).and_modify(|s| if ns.total_p > s.total_p { *s = ns.clone() }).or_insert(ns);
            }
        }

        let mut new_states: Vec<_> = new_states.into_iter().collect();
        new_states.sort_by(|(_, x), (_, y)| x.total_p.partial_cmp(&y.total_p).unwrap().reverse());
        new_states.truncate(KEEP_N_BEST);
        new_states
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
    (0.4 * a1 + 0.3 * a2 + 0.2 * a3 + 0.1 * a4).ln()
}

fn trace_sequence(s: &State) -> Vec<u16> {
    if let Some(p) = &s.parent {
        trace_sequence(p).apply_owned(|x| x.push(s.id))
    } else {
        vec![s.id]
    }
}

fn rotate_insert<T>(vec: &mut Vec<T>, len: usize, new: T) {
    if vec.len() == len {
        vec[0] = new;
        vec.rotate_left(1);
    } else {
        vec.push(new);
    }
}
