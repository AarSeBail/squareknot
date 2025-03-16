use std::io::BufRead;

use itertools::Itertools;

/// This is a utility for use in parsers
/// It handles the common scenario where the remaining lines in a file are pairs of numbers
pub struct PairIterator<B: BufRead> {
    reader: B,
    buffer: String,
}

impl<B: BufRead> PairIterator<B> {
    pub fn new(reader: B) -> Self {
        Self {
            reader,
            buffer: String::with_capacity(16),
        }
    }
}

impl<B: BufRead> Iterator for PairIterator<B> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.clear();
        if let Ok(_) = self.reader.read_line(&mut self.buffer) {
            self.buffer
                .split_whitespace()
                .map(|s| s.parse::<usize>().ok())
                .take(2)
                .filter_map(|n| n)
                .collect_tuple::<(usize, usize)>()
        } else {
            None
        }
    }
}
