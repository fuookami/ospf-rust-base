use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

pub struct GeneratorIterator<G>(pub G);

impl<G: Generator + Unpin> Iterator for GeneratorIterator<G> {
    type Item = G::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match Pin::new(&mut self.0).resume(()) {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(_) => None,
        }
    }
}
