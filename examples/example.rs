#![feature(plugin)]
#![plugin(trie_match)]

fn main() {
    let x = "z";
    let y = trie_match!(x {
        "aaa" => 1,
        "aab" => 2,
        "aac" => 3,
        "aba" => 4,
        "abb" => 5,
        "abc" => 6,
        "cba" => 7,
        "cbb" => 8,
        "cbc" => 9,
        _ => 99,
    });
    println!("{}", y);
}
