// package useless;

// import terms.*;
// import generators.*;
use treebag_generators::{TreeGenerator, TreeTransducer};
use treebag_terms::{RankedSymbol, Term, TermHandle};

pub struct TrivialTransducer1 {
    output: [[TermHandle; 2]; 2],
    current_term: TermHandle,
    current_pair: usize,
    next_index: usize,
}

impl Default for TrivialTransducer1 {
    fn default() -> Self {
        let output = [
            [
                Term::new_handle(RankedSymbol::new_handle("a", 0)),
                Term::new_handle(RankedSymbol::new_handle("b", 0)),
            ],
            [
                Term::new_handle(RankedSymbol::new_handle("c", 0)),
                Term::new_handle(RankedSymbol::new_handle("d", 0)),
            ],
        ];
        let current_term = output[0][0].clone();
        Self {
            output,
            current_term,
            current_pair: 0,
            next_index: 0,
        }
    }
}

impl TreeGenerator for TrivialTransducer1 {
    fn current_term(&self) -> TermHandle {
        return self.current_term.clone();
    }
}

impl TreeTransducer for TrivialTransducer1 {
    fn apply(&mut self, t: TermHandle) -> TermHandle {
        self.current_term = self.output[self.current_pair][self.next_index].clone();
        self.next_index = 1 - self.next_index;
        return self.current_term.clone();
    }
}
