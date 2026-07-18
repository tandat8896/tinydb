use super::*;
//conflict luc lam viec 
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

#[test]
fn insert_keys_get_data() {
    let mut tree = BPlusTree::new(4);
    tree.insert(10, "a");
    assert_eq!(tree.get(&10), Some(&"a"));
}

#[test]
fn insert_causes_leaf_split() {
    let mut tree = BPlusTree::new(2);
    tree.insert(1, "a");
    tree.insert(2, "b");
    tree.insert(3, "c");
    assert_eq!(tree.get(&1), Some(&"a"));
    assert_eq!(tree.get(&2), Some(&"b"));
    assert_eq!(tree.get(&3), Some(&"c"));
}

#[test]
fn insert_many_keys_causes_multiple_splits() {
    let mut tree = BPlusTree::new(2);
    for i in 1..=6 {
        tree.insert(i, i * 10);
    }
    for i in 1..=6 {
        assert_eq!(tree.get(&i), Some(&(i * 10)));
    }
    assert_eq!(tree.get(&7), None);
}

#[test]
fn get_missing_key_after_split() {
    let mut tree = BPlusTree::new(2);
    tree.insert(1, "a");
    tree.insert(3, "c");
    assert_eq!(tree.get(&2), None);
    assert_eq!(tree.get(&1), Some(&"a"));
    assert_eq!(tree.get(&3), Some(&"c"));
}

#[test]
fn insert_reverse_order() {
    let mut tree = BPlusTree::new(3);
    for i in (1..=5).rev() {
        tree.insert(i, i * 100);
    }
    for i in 1..=5 {
        assert_eq!(tree.get(&i), Some(&(i * 100)));
    }
}

#[test]
fn internal_node_split() {
    let mut tree = BPlusTree::new(2);
    for i in 1..=10 {
        tree.insert(i, i * 10);
    }
    for i in 1..=10 {
        assert_eq!(tree.get(&i), Some(&(i * 10)));
        assert_eq!(tree.get(&0), None);
        assert_eq!(tree.get(&11), None)
    }
}

#[test]
fn insert_random_order() {
    let mut tree = BPlusTree::new(3);
    let keys = [5, 1, 4, 2, 3];
    for &k in &keys {
        tree.insert(k, k * 100);
    }
    for &k in &keys {
        assert_eq!(tree.get(&k), Some(&(k * 100)));
    }
}
