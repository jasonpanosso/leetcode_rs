use std::collections::HashMap;

// NOTE: Refactor to BTree?

#[derive(Debug)]
pub struct Trie {
    char_map: HashMap<char, Trie>,
    is_full_word: bool,
}

impl Default for Trie {
    fn default() -> Self {
        Trie::new()
    }
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            char_map: HashMap::new(),
            is_full_word: false,
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut cur = self;

        for ch in word.chars() {
            cur = cur.char_map.entry(ch).or_default();
        }

        cur.is_full_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut cur = self;

        for ch in word.chars() {
            if !cur.char_map.contains_key(&ch) {
                return false;
            }

            cur = &cur.char_map.get(&ch).unwrap();
        }

        cur.is_full_word
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut cur = self;

        for ch in prefix.chars() {
            if !cur.char_map.contains_key(&ch) {
                return false;
            }

            cur = &cur.char_map.get(&ch).unwrap();
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut trie = Trie::new();

        trie.insert(String::from("Test"));
        trie.insert(String::from("Temp"));

        println!("{:?}", trie);
    }

    #[test]
    fn test_search() {
        let mut trie = Trie::new();

        trie.insert(String::from("Test"));
        trie.insert(String::from("Temp"));

        let test1 = trie.search(String::from("Test"));
        let test2 = trie.search(String::from("Trst"));
        assert!(test1);
        assert!(!test2);
    }

    #[test]
    fn test_starts_with() {
        let mut trie = Trie::new();

        trie.insert(String::from("Test"));
        trie.insert(String::from("Temp"));
        trie.insert(String::from("Tsac"));

        let test1 = trie.starts_with(String::from("T"));
        let test2 = trie.starts_with(String::from("Te"));
        let test3 = trie.starts_with(String::from("X"));

        assert!(test1);
        assert!(test2);
        assert!(!test3);
    }
}
