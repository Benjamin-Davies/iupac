//! Idea: run a DFA on the lower 4 bits of each byte of input, then check each
//! state that we've visited for a full match. The best case retrieval
//! (and hopefully average case for sparse results) is then O(n) to find
//! the end node plus O(n) to check the first mapping.

use std::num::NonZeroUsize;

/// DFA that can only match against fixed prefixes.
#[derive(Debug)]
pub struct Automaton<O> {
    states: Vec<State<O>>,
}

#[derive(Debug)]
struct State<O> {
    transitions: [Option<NonZeroUsize>; 16],
    parent: usize,
    mappings: Vec<(&'static str, O)>,
}

fn low_nibble(c: u8) -> usize {
    c as usize & 0xf
}

impl<O> Automaton<O> {
    pub fn new() -> Self {
        Automaton { states: Vec::new() }
    }

    /// `self.states` must not be empty
    fn traverse(&self, input: &str) -> usize {
        let mut cursor = 0;
        for byte in input.bytes() {
            let nibble = low_nibble(byte);
            if let Some(next) = self.states[cursor].transitions[nibble] {
                cursor = next.get();
            } else {
                break;
            }
        }
        cursor
    }

    fn traverse_or_create(&mut self, input: &str) -> usize {
        if self.states.is_empty() {
            self.states.push(State::default());
        }

        let mut cursor = 0;
        for byte in input.bytes() {
            let nibble = low_nibble(byte);
            if let Some(next) = self.states[cursor].transitions[nibble] {
                cursor = next.get();
            } else {
                let next = self.states.len();
                self.states.push(State::default());
                self.states[cursor].transitions[nibble] = NonZeroUsize::new(next);
                self.states[next].parent = cursor;
                cursor = next;
            }
        }
        cursor
    }

    pub fn insert(&mut self, input: &'static str, output: O) {
        let index = self.traverse_or_create(input);
        self.states[index].mappings.push((input, output));
    }

    /// Finds the longest matching prefix of `input`. If a match was found, then
    /// returns the length of the prefix that was matched and the output.
    /// Otherwise, returns None.
    pub fn get_by_prefix(&self, input: &str) -> Option<(usize, &O)> {
        if self.states.is_empty() {
            return None;
        }

        let mut cursor = self.traverse(input);
        loop {
            let state = &self.states[cursor];
            for (prefix, output) in &state.mappings {
                if input.starts_with(prefix) {
                    return Some((prefix.len(), output));
                }
            }

            let next = state.parent;
            if next == cursor {
                return None;
            }
            cursor = next;
        }
    }
}

impl<O> Default for Automaton<O> {
    fn default() -> Self {
        Automaton::new()
    }
}

impl<O> Default for State<O> {
    fn default() -> Self {
        State {
            transitions: [None; 16],
            parent: 0,
            mappings: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Automaton;

    #[test]
    fn test_get_by_prefix() {
        let mut automaton = Automaton::new();
        automaton.insert("a", 0);
        automaton.insert("aa", 1);
        automaton.insert("ab", 2);
        automaton.insert("abc", 3);

        assert_eq!(automaton.get_by_prefix(""), None);
        assert_eq!(automaton.get_by_prefix("a"), Some((1, &0)));
        assert_eq!(automaton.get_by_prefix("aa"), Some((2, &1)));
        assert_eq!(automaton.get_by_prefix("aaa"), Some((2, &1)));
        assert_eq!(automaton.get_by_prefix("aba"), Some((2, &2)));
        assert_eq!(automaton.get_by_prefix("abc"), Some((3, &3)));
    }

    #[test]
    fn test_get_by_prefix_empty_automaton() {
        let automaton = Automaton::<()>::new();

        assert_eq!(automaton.get_by_prefix("abc"), None);
    }

    #[test]
    fn test_get_by_prefix_matching_empty_input() {
        let mut automaton = Automaton::new();
        automaton.insert("", 0);
        automaton.insert("abc", 1);

        assert_eq!(automaton.get_by_prefix(""), Some((0, &0)));
        assert_eq!(automaton.get_by_prefix("a"), Some((0, &0)));
        assert_eq!(automaton.get_by_prefix("a"), Some((0, &0)));
        assert_eq!(automaton.get_by_prefix("abc"), Some((3, &1)));
    }

    #[test]
    fn test_chemistry_terms() {
        let mut automaton = Automaton::new();
        automaton.insert("But", 0);
        automaton.insert("Butyl", 1);
        automaton.insert("Butane", 2);

        assert_eq!(automaton.get_by_prefix("But"), Some((3, &0)));
        assert_eq!(automaton.get_by_prefix("Butane"), Some((6, &2)));
        assert_eq!(automaton.get_by_prefix("Butene"), Some((3, &0)));
    }
}
