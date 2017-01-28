use std::collections::BTreeMap;
use std::mem;

type NodeMap<V> = BTreeMap<u8, Box<Node<V>>>;

enum Node<V> {
    Trie {
        children: NodeMap<V>,
    },
    TrieWithValue {
        children: NodeMap<V>,
        value: V,
    },
    PrefixWithValue {
        key: Vec<u8>,
        value: V,
    },
    PrefixWithChild {
        key: Vec<u8>,
        child: Box<Node<V>>,
    },
    PrefixWithValueAndChild {
        key: Vec<u8>,
        value: V,
        child: Box<Node<V>>,
    },
}

impl<V> Node<V> {
    fn get(&self, key: &[u8]) -> Option<&mut V> {
        match *self {
            Node::Trie { ref children } => {
                get_children(key, children)
            }
            Node::TrieWithValue { ref children, ref value } => {
                if key.is_empty() {
                    value.as_ref()
                } else {
                    get_children(key, children)
                }
            }
            Node::PrefixWithValue { key: ref prefix_key, ref value } => {
                if key == prefix_key {
                    Some(value)
                } else {
                    None
                }
            }
            Node::PrefixWithChild { key: ref prefix_key, ref child } => {
                if prefix_key.len() < key.len() && key.startswith(prefix_key) {
                    child.get(key[prefix_key.len()..])
                } else {
                    None
                }
            }
            Node::PrefixWithValueAndChild { key: ref prefix_key, ref value, ref child } => {
                match classify(key, prefix_key) {
                    Classify::NotEqual | Classify::Subset => None,
                    Classify::Equal => Some(value),
                    Classify::Superset => child.get(key[prefix_key.len()..]),
                }
            }
        }
    }

    fn insert(&mut self, key: &[u8], value: V) {
        if key.is_empty() {
            let insert = match *self {
                Node::TrieWithValue { ref mut self_value, .. }
                | Node::PrefixWithValue { ref mut self_value, .. }
                | Node::PrefixWithValueAndChild { ref mut self_value, .. } => {
                    *self_value = value;
                    return;
                }
                Node::Trie { children: ref mut self_children } => {
                    let mut children = BTreeMap::new();
                    mem::swap(&mut children, self_children);
                    InsertOp::Trie(children)
                }
                Node::PrefixWithChild { key: ref mut self_key, ref mut child } => {
                    let mut k = Vec::new();
                    mem::swap(&mut k, self_key);

                    let mut c = Box::new(Node::Trie {
                        children: BTreeMap::new(),
                    };
                }
                _ => 
            }
        } else {
            match *self {
                Node::Trie { ref children } => {
                }
                Node
            }
        }
    }
}

enum InsertOp<V> {
    Trie(NodeMap<V>),
    Prefix(Key, Box<Node<V>>),
}

enum Classify {
    NotEqual,
    Equal,
    Subset,
    Superset,
}

fn classify(subset: &[u8], superset: &[u8]) -> Classify {
    let subset = subset.iter();
    let superset = superset.iter();

    loop {
        match (subset.next(), superset.next()) {
            (Some(lhs), Some(rhs)) => {
                if lhs != rhs { return Classify::NotEqual; }
            }
            (Some(_), None) => { return Classify::Superset; }
            (None, Some(_)) => { return Classify::Subset; }
            (None, None) => { return Classify::Equal; }
        }
    }
}

fn get_children(key: &[u8], children: &NodeMap<V>) -> Option<&mut V> {
    let byte = key[0];
    match children.get(byte) {
        Some(child) => child.get(key[1..]),
        None => None,
    }
}
