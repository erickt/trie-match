use std::collections::{BTreeMap, btree_map};
use std::fmt::{self, Debug};
use std::iter;
use std::mem;

use quickcheck;

#[derive(Debug, Clone)]
pub struct TrieMap<V> {
    root: TrieNode<V>,
    len: usize,
}

impl<V: Debug> TrieMap<V> {
    pub fn new() -> Self {
        TrieMap {
            root: TrieNode::empty(),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn insert(&mut self, key: &[u8], value: V) -> Option<V> {
        println!("TrieMap.insert: {:?}", key);

        let old_value = self.root.insert(key, value);

        if old_value.is_none() {
            self.len += 1;
        }

        old_value
    }

    pub fn get<'a>(&'a mut self, key: &[u8]) -> Option<&'a V> {
        println!("TrieMap.get: {:?}", key);

        self.root.get(key)
    }

    /*
    pub fn iter(&self) -> Iter<'a, V> {
        Iter {
            key: Vec::new(),
            iter: self.root.iter(Vec::new())
        }
    }
    */
}

/*
enum IterState<'a, V> {
    Trie(TrieIter<'a, V>),
    Prefix(PrefixNode<'a, V>),
}

pub struct Iter<'a, V> {
    key: Vec<u8>,
    iter_state: IterState<'a, V>,
}

impl<'a, V> Iterator for Iter<'a, V> {
    type Item = (Vec<u8>, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
*/

impl<'a, V: Debug> iter::FromIterator<(&'a [u8], V)> for TrieMap<V> {
    fn from_iter<I: IntoIterator<Item=(&'a [u8], V)>>(iterator: I) -> Self {
        let mut map = TrieMap::new();
        for (key, value) in iterator.into_iter() {
            map.insert(key, value);
        }
        map
    }
}

/*
impl<V> fmt::Debug for TrieMap<V>
    where V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<V> quickcheck::Arbitrary for TrieMap<V>
    where V: Clone + quickcheck::Arbitrary,
{
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> TrieMap<V> {
        use std::collections::HashMap;

        let items: HashMap<Vec<u8>, V> = quickcheck::Arbitrary::arbitrary(g);
        TrieMap::from_iter(items.into_iter())
    }

    fn shrink(&self) -> Box<Iterator<Item=TrieMap<V>>> {
        use std::collections::HashMap;

        let items: HashMap<Vec<u8>, V> = self.iter()
            .map(|(key, value)| (key.to_owned(), value.clone()))
            .collect();

        Box::new(items.shrink()
            .map(|items| items.into_iter().collect::<TrieMap<V>>()))
    }
}
*/

#[derive(Clone, Debug)]
enum Node<V> {
    Trie(TrieNode<V>),
    Prefix(PrefixNode<V>),
}

enum InsertResult<V> {
    Ok(Option<V>),
    Burst(TrieNode<V>),
}

impl<V: Debug> Node<V> {
    fn insert(&mut self, key: &[u8], value: V) -> Option<V> {
        println!("Node.insert: {:?}", key);

        let trie = match *self {
            Node::Trie(ref mut node) => {
                return node.insert(key, value);
            }
            Node::Prefix(ref mut node) => {
                match node.insert(key, value) {
                    InsertResult::Ok(value) => { return value; }
                    InsertResult::Burst(trie) => trie,
                }
            }
        };

        *self = Node::Trie(trie);

        None
    }

    fn get<'a>(&'a self, key: &[u8]) -> Option<&'a V> {
        println!("Node.get: {:?}", key);

        match *self {
            Node::Trie(ref node) => node.get(key),
            Node::Prefix(ref node) => node.get(key),
        }
    }

    fn len(&self) -> usize {
        match *self {
            Node::Trie(ref node) => node.len(),
            Node::Prefix(ref node) => node.len(),
        }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /*
    fn iter(&'a self, key: Vec<u8>) -> NodeIter<'a, V> {
        match *self {
            Trie(node) => Iter::Trie(node.iter(key)),
            Prefix(node) => Iter::Prefix(node.iter(key)),
        }
    }
    */
}

/*
enum NodeIter<'a, V> {
    Trie(TrieIter<'a, V>),
    Prefix(PrefixIter<'a, V>),
}

impl<'a, V> Iterator for Iter<'a, V> {
    type Item = (Vec<u8>, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            NodeIter::Trie(iter) => iter.next(),
            NodeIter::Prefix(iter) => iter.next(),
        }
    }
}
*/

impl<V: Debug> From<TrieNode<V>> for Node<V> {
    fn from(node: TrieNode<V>) -> Self {
        Node::Trie(node)
    }
}

impl<V: Debug> From<PrefixNode<V>> for Node<V> {
    fn from(node: PrefixNode<V>) -> Self {
        Node::Prefix(node)
    }
}

#[derive(Clone, Debug)]
struct TrieNode<V> {
    children: BTreeMap<u8, Box<Node<V>>>,
    value: Option<V>,
}

impl<V: Debug> TrieNode<V> {
    fn empty() -> Self {
        TrieNode {
            children: BTreeMap::new(),
            value: None,
        }
    }

    fn len(&self) -> usize {
        self.children.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn insert(&mut self, key: &[u8], value: V) -> Option<V> {
        match key.first() {
            Some(byte) => {
                println!("TrieNode.insert: {:?}", byte);

                let key = &key[1..];

                match self.children.entry(*byte) {
                    btree_map::Entry::Occupied(mut entry) => {
                        println!("TrieNode.insert: update: {:?}", key);

                        entry.get_mut().insert(key, value)
                    }
                    btree_map::Entry::Vacant(entry) => {
                        println!("TrieNode.insert: prefix: {:?}", key);

                        let node = PrefixNode::new(key.to_owned(), value);
                        entry.insert(Box::new(Node::from(node)));
                        None
                    }
                }
            }
            None => {
                println!("TrieNode.insert: empty");

                let mut old_value = None;
                mem::swap(&mut self.value, &mut old_value);

                self.value = Some(value);

                old_value
            }
        }
    }

    fn get<'a>(&'a self, key: &[u8]) -> Option<&'a V> {
        println!("TrieNode.get: {:?}", key);

        match key.first() {
            Some(byte) => {
                match self.children.get(byte) {
                    Some(child) => child.get(&key[1..]),
                    None => None,
                }
            }
            None => {
                self.value.as_ref()
            }
        }
    }
}

/*
struct TrieIter<'a, V> {
    key: &'a mut Vec<u8>,
    iter: btree_map::Iter<'a, V>,
    value: Option<&'a V>,
    node_iter: Option<Iter<'a, V>>,
}

impl<'a, V> TrieIter<'a, V> {
    fn new(key: &'a mut Vec<u8>,
           value: Option<&'a V>,
           iter: btree_map::Iter<'a, V>) -> Self {
        TrieIter {
            key: key,
            value: value,
            iter: iter,
            node_iter: None,
        }
    }
}

impl<'a, V> Iterator for TrieIter<'a, V> {
    type Item = (Vec<u8>, &'a V);

    fn next(&mut self) -> Option<Item::Self> {
        // First, optionally et=
        match self.value.take() {
            Some(value) => {
            }
            None => { }
        }

        loop {

            match self.node_iter {
                Some(ref mut node_iter) => {
                    match node_iter.next() {
                        Some(item) => { return Some(item); }
                        None => {
                            // Reclaim the key back from the node iterator and remove the byte we
                            // pushed onto the key.
                            mem::swap(self.key, node_iter.key);
                            key.pop();
                        }
                    }
                }
                None => { }
            }

            match self.iter.next() {
                Some((byte, node)) => {
                    self.key.push(byte);
                    self.node_iter = Some(node.iter());
                }
                None => { return None; }
            }
        }
    }
}
*/

#[derive(Clone, Debug)]
struct PrefixNode<V> {
    key: Vec<u8>,
    value: Option<V>,
    child: Option<Box<Node<V>>>,
}


impl<V: Debug> PrefixNode<V> {
    fn new(key: Vec<u8>, value: V) -> Self {
        PrefixNode::with_child(key, value, None)
    }

    fn with_child(key: Vec<u8>, value: V, child: Option<Box<Node<V>>>) -> Self {
        PrefixNode {
            key: key,
            value: Some(value),
            child: child,
        }
    }

    fn len(&self) -> usize {
        if self.child.is_some() { 1 } else { 0 }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn find_difference(&self, key: &[u8]) -> Option<usize> {
        key.iter()
            .zip(self.key.iter())
            .position(|(lhs, rhs)| lhs != rhs)
    }

    fn insert(&mut self, key: &[u8], value: V) -> InsertResult<V> {
        println!("PrefixNode.insert: self.key: {:?} other.key: {:?}", self.key, key);

        match self.find_difference(key) {
            Some(pos) => {
                let trie = self.burst_and_insert(pos, key, value);

                // If we bursted on the first byte, then transform this node into a trie.
                if pos == 0 {
                    InsertResult::Burst(trie)
                } else {
                    self.child = Some(Box::new(Node::from(trie)));
                    InsertResult::Ok(None)
                }
            }
            None => {
                // If we have an exact collision, then just update the value. Otherwise
                // insert the remaining key into our child.
                if key.len() == self.key.len() {
                    println!("PrefixNode.insert: updating key");

                    let mut old_value = None;
                    mem::swap(&mut self.value, &mut old_value);

                    self.value = Some(value);

                    InsertResult::Ok(old_value)
                } else {
                    println!("PrefixNode.insert2: self.key: {:?} other.key: {:?}", self.key, key);
                    let key = &key[self.key.len()..];
                    println!("PrefixNode.insert3: splitting other.key: {:?}", key);

                    if let Some(ref mut child) = self.child {
                        InsertResult::Ok(child.insert(key, value))
                    } else {
                        let child = PrefixNode::new(key.to_owned(), value);
                        self.child = Some(Box::new(Node::from(child)));

                        InsertResult::Ok(None)
                    }
                }
            }
        }
    }

    fn get<'a>(&'a self, key: &[u8]) -> Option<&'a V> {
        println!("PrefixNode.get: self.key: {:?} other.key: {:?}", self.key, key);

        match self.find_difference(key) {
            Some(_) => None,
            None => {
                if key.len() == self.key.len() {
                    self.value.as_ref()
                } else {
                    match self.child {
                        Some(ref child) => child.get(&key[self.key.len()..]),
                        None => None,
                    }
                }
            }
        }
    }

    fn split_front(&mut self) -> TrieNode<V> {
        assert!(!self.key.is_empty());

        let mut value = None;
        mem::swap(&mut value, &mut self.value);

        let mut child = None;
        mem::swap(&mut child, &mut self.child);

        let byte = self.key[0];
        let node = PrefixNode::with_child(self.key[1..].to_owned(), value, child);

        let mut trie: TrieNode<V> = TrieNode::<V>::empty();
        trie.children.insert(byte, Box::new(Node::from(node)));

        trie
    }

    fn burst_into_trie(&mut self) -> TrieNode<V> {
        // There are three cases that we need to handle when we burst a prefix node into a trie
        // node:
        //
        // (1): The prefix is empty and it does not have a child. This
        // must be converted into an empty trie node.
        //
        // (2): The prefix is empty, and it has a child. This must be converted
        // into a trie node, where the child is reinserted into the trie node.
        //
        // (3): The prefix is not empty. The prefix[1..] is cloned into a new prefix node, and is
        // inserted into the trie node.

        let mut self_value = None;
        mem::swap(&mut self_value, &mut self.value);

        let mut self_child = None;
        mem::swap(&mut self_child, &mut self.child);

        if self.key.is_empty() {
            match self_child {
                Some(node) => {
                    match *node {
                        Node::Trie(node) => {
                            assert!(!node.is_empty());
                            assert!(node.value.is_none());

                            node.value = self_value;

                            node
                        }
                        Node::Prefix(node) => {
                            node.split_front()
                        }
                    }
                }
                None => {
                    let mut trie = TrieNode::empty();
                    trie.value = self_value;

                    trie
                }
            }
        } else {
            self.split_front()
        }
    }

    /// Split the current node's key at position `pos`, and insert it and the current value into a
    /// trie.
    fn burst(&mut self, pos: usize) -> TrieNode<V> {
        println!("PrefixNode.burst: pos: {}", pos);

        let mut self_value = None;
        mem::swap(&mut self_value, &mut self.value);

        let mut self_child = None;
        mem::swap(&mut self_child, &mut self.child);

        let mut trie = TrieNode::empty();

        if self.key.is_empty() {
            trie.value = self_value;
        } else {
            let self_byte = self.key[pos];
            println!("byte: {:?}", self_byte);

            let node = Node::from(PrefixNode {
                key: self.key[pos..].to_owned(),
                value: self_value,
                child: self_child,
            });

            println!("making node: {:?}", node);

            trie.children.insert(self_byte, Box::new(node));
        }

        println!("PrefixNode.burst2: {:?} {:?}",
                 pos,
                 self.key);

        self.key.truncate(pos + 1);

        println!("PrefixNode.burst3: {:?} {:?}",
                 pos,
                 self.key);

        trie
    }

    /// Burst the current node 
    fn burst_and_insert(&mut self, pos: usize, key: &[u8], value: V) -> TrieNode<V> {
        println!("burst_and_insert: {:?} {:?} {:?}", pos, self.key, key);
        println!("--");
        println!("{:?}", self);

        let mut trie = self.burst(pos - 1);

        println!("{:?}", trie);

        println!("");

        let other_byte = key[pos];
        let other_suffix_node = PrefixNode {
            key: key[pos..].to_owned(),
            value: Some(value),
            child: None,
        };

        println!("burst_and_insert2: {:?}", other_suffix_node);
        println!("--\n");

        trie.children.insert(other_byte, Box::new(Node::from(other_suffix_node)));

        trie
    }
}

/*
struct PrefixIter<'a, V> {
    key: Vec<u8>,
    value: Option<&'a V>,
}
*/

