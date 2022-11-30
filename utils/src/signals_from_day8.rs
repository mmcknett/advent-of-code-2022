use std::collections::BTreeSet;

pub type Signal = BTreeSet<char>;
pub type Puzzle = (Vec<Signal>, Vec<Signal>);
