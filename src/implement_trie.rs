use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    is_end_of_word: bool,
    children: HashMap<char, TrieNode>
}

#[derive(Default)]
pub struct Trie {
    root: TrieNode
}



impl Trie {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }
    
    pub fn insert(&mut self, word: String) {
        let mut curr_node = &mut self.root;
        for c in word.chars().into_iter() {
            curr_node = curr_node.children.entry(c).or_default();
        }
        
        curr_node.is_end_of_word = true;
    }
    
    pub fn search(&self, word: String) -> bool {
        let mut curr_node = &self.root;
        for c in word.chars() {
            if let Some(node) = curr_node.children.get(&c) {
                curr_node = node;
            } else {
                return false;
            }
        }

        curr_node.is_end_of_word
    }
    
    pub fn starts_with(&self, prefix: String) -> bool {
        let mut curr_node = &self.root;
        for c in prefix.chars() {
            if let Some(node) = curr_node.children.get(&c) {
                curr_node = node;
            } else {
                return false;
            }
        }
        true
    }
}