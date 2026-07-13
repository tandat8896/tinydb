// use std::collections::btree_map::Keys

pub type NodeId = usize;

pub enum Node<K, V> {
    Internal {
        keys: Vec<K>,
        children: Vec<NodeId>,
    },
    Leaf {
        keys: Vec<K>,
        values: Vec<V>,
        next: Option<NodeId>,
    },
}

pub struct BPlusTree<K, V> {
    order: usize,
    root: Option<NodeId>,
    arena: Vec<Node<K, V>>,
}

impl<K: Ord, V> BPlusTree<K, V> {
    pub fn new(order: usize) -> Self {
        Self {
            order,
            root: None,
            arena: Vec::new(),
        }
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        if self.root == None {
            return None;
        }
        let current = self.root?;
        let mut node_id = current;
        loop {
            match &self.arena[node_id] {
                Node::Internal { keys, children } => {
                    let i = keys.binary_search(key).unwrap_or_else(|i| i);
                    node_id = children[i];
                }
                Node::Leaf { keys, values, .. } => {
                    if let Ok(i) = keys.binary_search(key) {
                        return Some(&values[i]);
                    }
                    return None;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests;
