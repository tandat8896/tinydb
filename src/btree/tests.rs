use super::*;

#[test]
fn get_missing_key_returns_none() {
    let tree: BPlusTree<i32, &str> = BPlusTree::new(4);
    assert_eq!(tree.get(&99), None);
}

#[test]
fn insert_then_get_returns_value() {
    let mut tree = BPlusTree::new(4);
    tree.insert(42, "hello");
    assert_eq!(tree.get(&42), Some(&"hello"));
}
