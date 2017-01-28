//#![feature(test)]

extern crate prefix_trie;
//extern crate quickcheck;

use prefix_trie::TrieMap;

#[test]
fn test_empty() {
    let mut trie = TrieMap::<u32>::new();

    assert!(trie.is_empty());
    assert_eq!(trie.len(), 0);

    trie.insert(b"aaa", 1);
    assert!(!trie.is_empty());
    assert_eq!(trie.len(), 1);

    trie.insert(b"aaa", 2);
    assert!(!trie.is_empty());
    assert_eq!(trie.len(), 1);

    trie.insert(b"aab", 3);
    assert!(!trie.is_empty());
    assert_eq!(trie.len(), 2);
}

#[test]
fn test_insert() {
    let mut trie = TrieMap::new();

    assert_eq!(trie.insert(b"", 0), None);
    assert_eq!(trie.insert(b"", 1), Some(0));

    assert_eq!(trie.insert(b"a", 2), None);
    assert_eq!(trie.insert(b"aa", 3), None);
    assert_eq!(trie.insert(b"aaa", 4), None);

    assert_eq!(trie.insert(b"a", 5), Some(2));
    assert_eq!(trie.insert(b"aa", 6), Some(3));
    assert_eq!(trie.insert(b"aaa", 7), Some(4));

    assert_eq!(trie.insert(b"b", 8), None);
    assert_eq!(trie.insert(b"ab", 9), None);
    assert_eq!(trie.insert(b"aab", 10), None);

    assert_eq!(trie.insert(b"b", 11), Some(8));
    assert_eq!(trie.insert(b"ab", 12), Some(9));
    assert_eq!(trie.insert(b"aab", 13), Some(10));
}

#[test]
fn test_get() {
    let mut trie = TrieMap::new();

    println!("");

    /*
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"a"), None);
    assert_eq!(trie.get(b"aa"), None);
    assert_eq!(trie.get(b"aaa"), None);
    assert_eq!(trie.get(b"b"), None);
    assert_eq!(trie.get(b"ab"), None);
    assert_eq!(trie.get(b"aab"), None);
    assert_eq!(trie.get(b"cca"), None);
    assert_eq!(trie.get(b"ccb"), None);
    assert_eq!(trie.get(b"ccc"), None);
    */

    println!("");

    /*
    trie.insert(b"", 0);
    println!("");
    trie.insert(b"a", 1);
    println!("");
    trie.insert(b"aa", 2);
    println!("");
    trie.insert(b"aaa", 3);
    trie.insert(b"b", 4);
    trie.insert(b"ab", 5);
    trie.insert(b"aab", 6);
    */
    trie.insert(b"cca", 7);
    println!("trie: {:?}", trie);
    println!("");
    trie.insert(b"ccb", 8);
    println!("trie: {:?}", trie);
    /*
    println!("");
    trie.insert(b"ccc", 9);
    println!("trie: {:?}", trie);

    println!("");
    */

    /*
    assert_eq!(trie.get(b""), Some(&0));
    println!("");
    assert_eq!(trie.get(b"a"), Some(&1));
    println!("");
    assert_eq!(trie.get(b"aa"), Some(&2));
    assert_eq!(trie.get(b"aaa"), Some(&3));
    assert_eq!(trie.get(b"b"), Some(&4));
    assert_eq!(trie.get(b"ab"), Some(&5));
    assert_eq!(trie.get(b"aab"), Some(&6));
    assert_eq!(trie.get(b"cca"), Some(&7));
    assert_eq!(trie.get(b"ccb"), Some(&8));
    assert_eq!(trie.get(b"ccc"), Some(&9));
    */
}

/*
#[test]
fn quickcheck_insert() {
    fn prop(trie: TrieMap<u32>) -> bool {
        true
    }

    quickcheck::quickcheck(prop as fn(TrieMap<u32>) -> bool);
}
*/
