use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct WordDictionary {
    char_map: HashMap<char, WordDictionary>,
    is_full_word: bool,
}

impl WordDictionary {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_word(&mut self, word: String) {
        let mut cur = self;

        for ch in word.chars() {
            cur = cur.char_map.entry(ch).or_default();
        }

        cur.is_full_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        self.search_helper(&word.chars().collect::<Vec<char>>(), 0)
    }

    fn search_helper(&self, chars: &Vec<char>, index: usize) -> bool {
        if index == chars.len() {
            return self.is_full_word;
        }

        let ch = chars[index];

        if ch == '.' {
            for val in self.char_map.values() {
                if val.search_helper(chars, index + 1) {
                    return true;
                }
            }

            false
        } else if let Some(val) = self.char_map.get(&ch) {
            val.search_helper(chars, index + 1)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut dict = WordDictionary::new();

        dict.add_word(String::from("Test"));
        dict.add_word(String::from("Temp"));

        println!("{:?}", dict);
    }

    #[test]
    fn test_search() {
        let mut dict = WordDictionary::new();

        dict.add_word(String::from("Test"));
        dict.add_word(String::from("Temp"));

        let test1 = dict.search(String::from("Tes."));
        let test2 = dict.search(String::from("T.st"));
        let test3 = dict.search(String::from("Tesm"));
        assert!(test1);
        assert!(test2);
        assert!(!test3);
    }
}
