mod trie {
    pub struct Tree {
        pub children: Vec<Vec<i32>>,
        pub is_word: Vec<bool>,
    }
    impl Tree {
        pub fn new() -> Self {
            Self {
                children: vec![vec![-1; 26]; 1],
                is_word: vec![false],
            }
        }
        pub fn insert(&mut self, word: String) {
            let mut node = 0;
            for c in word.chars() {
                let c = c as usize - 'a' as usize;
                if self.children[node][c] == -1 {
                    self.children[node][c] = self.children.len() as i32;
                    self.children.push(vec![-1; 26]);
                    self.is_word.push(false);
                }
                node = self.children[node][c] as usize;
            }
            self.is_word[node] = true;
        }
        pub fn search(&self, word: String) -> bool {
            let mut node = 0;
            for c in word.chars() {
                let c = c as usize - 'a' as usize;
                if self.children[node][c] == -1 {
                    return false;
                }
                node = self.children[node][c] as usize;
            }
            self.is_word[node]
        }
        pub fn delete(&mut self, word: String) {
            let mut node = 0;
            for c in word.chars() {
                let c = c as usize - 'a' as usize;
                if self.children[node][c] == -1 {
                    return;
                }
                node = self.children[node][c] as usize;
            }
            self.is_word[node] = false;
        }
    }
}
fn main() {
    let mut tree = trie::Tree::new();
    tree.insert("apple".to_string());
    tree.insert("app".to_string());
    println!("{}", tree.search("app".to_string()));
    tree.delete("app".to_string());
    println!("{}", tree.search("app".to_string()));
}
