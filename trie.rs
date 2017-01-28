use std::collections::{BTreeMap, btree_map};
use std::mem;

#[derive(Debug)]
pub struct TrieMap<T> {
    root: BTreeMap<usize, Node<T>>,
}

impl<T> TrieMap<T> {
    pub fn new() -> Self {
        TrieMap {
            root: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: &[u8], value: T) {
        match self.root.entry(key.len()) {
            btree_map::Entry::Occupied(ref mut entry) => {
                entry.get_mut().insert(key, value);
            }
            btree_map::Entry::Vacant(entry) => {
                let child = PrefixNode::new(key.to_owned(), value);
                entry.insert(Node::from(child));
            }
        }
    }
}

#[derive(Debug)]
enum Node<T> {
    Trie(TrieNode<T>),
    Prefix(PrefixNode<T>),
}

impl<T> From<TrieNode<T>> for Node<T> {
    fn from(node: TrieNode<T>) -> Self {
        Node::Trie(node)
    }
}

impl<T> From<PrefixNode<T>> for Node<T> {
    fn from(node: PrefixNode<T>) -> Self {
        Node::Prefix(node)
    }
}

impl<T> Node<T> {
    fn insert(&mut self, key: &[u8], value: T) {
        let trie = match *self {
            Node::Trie(ref mut node) => {
                node.insert(key, value);
                return;
            }
            Node::Prefix(ref mut node) => {
                match node.insert(key, value) {
                    InsertResult::Ok => { return; }
                    InsertResult::Burst(trie) => trie,
                }
            }
        };

        *self = Node::Trie(trie);
    }
}

#[derive(Debug)]
struct TrieNode<T> {
    children: BTreeMap<u8, Box<Node<T>>>,
    value: Option<T>,
}

impl<T> TrieNode<T> {
    fn empty() -> Self {
        TrieNode {
            children: BTreeMap::new(),
            value: None,
        }
    }

    fn insert(&mut self, key: &[u8], value: T) {
        match key.first() {
            Some(byte) => {
                let key = &key[1..];

                match self.children.entry(*byte) {
                    btree_map::Entry::Occupied(mut entry) => {
                        entry.get_mut().insert(key, value);
                    }
                    btree_map::Entry::Vacant(entry) => {
                        let node = PrefixNode::new(key.to_owned(), value);
                        entry.insert(Box::new(Node::from(node)));
                    }
                }
            }
            None => {
                self.value = Some(value);
            }
        }
    }
}

#[derive(Debug)]
struct PrefixNode<T> {
    key: Vec<u8>,
    value: Option<T>,
    child: Option<Box<Node<T>>>,
}

enum InsertResult<T> {
    Ok,
    Burst(TrieNode<T>),
}

impl<T> PrefixNode<T> {
    fn new(key: Vec<u8>, value: T) -> Self {
        PrefixNode {
            key: key,
            value: Some(value),
            child: None,
        }
    }

    fn insert(&mut self, key: &[u8], value: T) -> InsertResult<T> {
        println!("inserting: {:?}", ::std::str::from_utf8(key).unwrap());

        let pos = key.iter()
            .zip(self.key.iter())
            .position(|(lhs, rhs)| lhs != rhs);

        match pos {
            Some(pos) => {
                let trie = self.burst_and_insert(pos, key, value);

                // If we bursted on the first byte, then transform this node into a trie.
                if pos == 0 {
                    InsertResult::Burst(trie)
                } else {
                    self.child = Some(Box::new(Node::from(trie)));
                    InsertResult::Ok
                }
            }
            None => {
                // If we have an exact collision, then just update the value. Otherwise
                // insert the remaining key into our child.
                if key.len() == self.key.len() {
                    self.value = Some(value);
                } else {
                    let key = &key[self.key.len()..];

                    if let Some(ref mut child) = self.child {
                        child.insert(key, value);
                    } else {
                        let child = PrefixNode::new(key.to_owned(), value);
                        self.child = Some(Box::new(Node::from(child)));
                    }
                }

                InsertResult::Ok
            }
        }
    }

    /// Split the current node's key at position `pos`, and insert it and the current value into a
    /// trie.
    fn burst(&mut self, pos: usize) -> TrieNode<T> {
        let mut self_value = None;
        mem::swap(&mut self_value, &mut self.value);

        let mut self_child = None;
        mem::swap(&mut self_child, &mut self.child);

        let self_byte = self.key[pos];
        let self_suffix_node = PrefixNode {
            key: self.key[pos + 1..].to_owned(),
            value: self_value,
            child: self_child,
        };

        let mut trie = TrieNode::empty();
        trie.children.insert(self_byte,  Box::new(Node::from(self_suffix_node)));

        println!("burst: {:?} {:?} {:?}",
                 pos,
                 self.key);

        self.key.truncate(pos);

        println!("burst: {:?} {:?} {:?}",
                 pos,
                 self.key);

        trie
    }

    /// Burst the current node 
    fn burst_and_insert(&mut self, pos: usize, key: &[u8], value: T) -> TrieNode<T> {
        println!("burst_and_insert: {:?} {:?} {:?}",
                 pos,
                 ::std::str::from_utf8(&self.key).unwrap(),
                 ::std::str::from_utf8(key).unwrap());

        let mut trie = self.burst(0);

        let other_byte = key[pos];
        let other_suffix_node = PrefixNode {
            key: key[pos + 1..].to_owned(),
            value: Some(value),
            child: None,
        };

        trie.children.insert(other_byte, Box::new(Node::from(other_suffix_node)));

        trie
    }
}

#[test]
fn test_insert() {

}

fn main() {
    let mut trie = TrieMap::new();
    trie.insert(b"aaa", 1);
    trie.insert(b"aab", 2);
    /*
    trie.insert(b"aac", 3);
    trie.insert(b"aba", 4);
    trie.insert(b"abb", 5);
    trie.insert(b"abc", 6);
    trie.insert(b"bba", 7);
    trie.insert(b"bbb", 8);
    trie.insert(b"bbc", 9);
    */

    println!("{:?}", trie);
}
