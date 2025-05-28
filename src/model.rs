use std::collections::HashMap;
use rand::{
    distr::{weighted::WeightedIndex, Distribution}, rng, rngs::ThreadRng, Rng
};

pub trait Dictionary {
    fn get_char(&mut self, character: char) -> Option<&mut HashMap<char, usize>>;
    fn insert_char(&mut self, character: char, hash_map: HashMap<char, usize>);
}

pub trait Analyzer<T: ToString>: Dictionary {
    fn new_word(&mut self, word: T) {
        self.analyze(word);
    }

    fn new_words(&mut self, words: Vec<T>) {
        for word in words {
            self.analyze(word);
        }
    }

    fn analyze(&mut self, word: T) {
        let string: Vec<char> = word.to_string().chars().collect();

        for pair in string.windows(2) {
            let (main_char, next_char) = (pair[0], pair[1]);

            if let Some(h) = self.get_char(main_char) {
                if let Some(c) = h.get_mut(&next_char) {
                    *c += 1;
                } else {
                    h.insert(next_char, 1);
                }
            } else {
                self.insert_char(main_char, HashMap::from([(next_char, 1)]));
            }
        }
    }
}

pub struct Model {
    dictionary: HashMap<char, HashMap<char, usize>>,
    rng: ThreadRng,
}

impl Model {
    pub fn new() -> Self {
        Self {
            dictionary: HashMap::new(),
            rng: rng(),
        }
    }

    pub fn random_word(&mut self, length: usize) -> String {
        let mut word = String::new();
        let mut character = self.first_char();
        word.push(character);

        for _ in 0..(length-2) {
            let mut c: Option<char> = None;
            for _ in 0..self.dictionary.len() {
                if let Some(s_c) = self.random_char(character) {
                    c = Some(s_c);
                    break;
                }
            }
            if c.is_none() {
                c = Some(self.first_char());
            }
            character = c.unwrap();
            word.push(character);
        }
        if let Some(c) = self.last_char(character) {
            word.push(c)
        } else {
            word.push(self.first_char());
        }
        word
    }

    pub fn random_word_with_range(&mut self, range: std::ops::Range<usize>) -> String {
        let length = self.rng.random_range(range);
        self.random_word(length)
    }

    pub fn random_words(&mut self, length: usize, count: usize) -> Vec<String> {
        let mut result = Vec::new();
        for _ in 0..count {
            result.push(self.random_word(length));
        }
        result
    }

    pub fn random_words_with_range(&mut self, range: std::ops::Range<usize>, count: usize) -> Vec<String> {
        let mut result = Vec::new();
        for _ in 0..count {
            let length = self.rng.random_range(range.clone());
            result.push(self.random_word(length));
        }
        result
    }

    fn first_char(&mut self) -> char {
        let chars: Vec<&char> = self.dictionary.keys().collect();
        let length = chars.len();

        *chars[self.rng.random_range(0..length)]
    }

    fn random_char(&mut self, character: char) -> Option<char> {
        let dict: Vec<(&char, &usize)> = self.dictionary.get(&character)?.iter().collect();
        let dist = WeightedIndex::new(
            dict
                .iter()
                .filter(|item| self.dictionary.contains_key(item.0))
                .map(|item| item.1)
        ).unwrap();

        Some(*dict[dist.sample(&mut self.rng)].0)
    }

    fn last_char(&mut self, character: char) -> Option<char> {
        let dict: Vec<(&char, &usize)> = self.dictionary.get(&character)?.iter().collect();
        let dist = WeightedIndex::new(
            dict
                .iter()
                .map(|item| item.1)
        ).unwrap();

        Some(*dict[dist.sample(&mut self.rng)].0)
    }
}

impl Dictionary for Model {
    fn get_char(&mut self, character: char) -> Option<&mut HashMap<char, usize>> {
        self.dictionary.get_mut(&character)
    }

    fn insert_char(&mut self, character: char, hash_map: HashMap<char, usize>) {
        self.dictionary.insert(character, hash_map);
    }
}

impl Analyzer<String> for Model {}
impl Analyzer<&str> for Model {}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = |h: &HashMap<char, usize>| {
            let mut s = String::new();
            for pair in h {
                let (c, u) = pair;
                s += &format!("    {c}: {u}\n");
            }
            s
        };

        let mut string = String::new();
        for pair in &self.dictionary {
            let (c, h) = pair;

            string += &format!("{c}:\n{}", display(h));
        }

        write!(
            f,
            "{}", string,
        )
    }
}