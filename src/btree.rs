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

impl<K: Ord + Clone, V> BPlusTree<K, V> {
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
                    let i = match keys.binary_search(key) {
                        Ok(i) => i + 1, // key bằng separator → rẽ phải
                        Err(i) => i,
                    };
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
    pub fn insert_data(&mut self, node_id: NodeId, key: K, value: V) -> Option<(K, NodeId)> {
        // "plan" = None (không tràn) hoặc Some((separator, node mới CẦN push, chưa push))
        let plan: Option<(K, Node<K, V>)>;

        let is_leaf = matches!(self.arena[node_id], Node::Leaf { .. });

        if is_leaf {
            // ---------- Nhánh LEAF ----------
            plan = match &mut self.arena[node_id] {
                Node::Leaf { keys, values, next } => {
                    let i = keys.binary_search(&key).unwrap_or_else(|i| i);
                    keys.insert(i, key);
                    values.insert(i, value);

                    if keys.len() <= self.order {
                        None
                    } else {
                        let mid = self.order / 2;
                        let new_keys = keys.split_off(mid);
                        let new_values = values.split_off(mid);
                        let separator = new_keys[0].clone();
                        let new_leaf = Node::Leaf {
                            keys: new_keys,
                            values: new_values,
                            next: *next,
                        };
                        Some((separator, new_leaf))
                    }
                }
                Node::Internal { .. } => unreachable!(),
            };
        } else {
            // ---------- Nhánh INTERNAL ----------
            // Bước 1: đọc i, child_id — borrow ngắn, tự kết thúc ngay dòng này
            let (i, child_id) = match &self.arena[node_id] {
                Node::Internal { keys, children } => {
                    let i = keys.binary_search(&key).unwrap_or_else(|i| i);
                    (i, children[i])
                }
                Node::Leaf { .. } => unreachable!(),
            };

            // Bước 2: đệ quy — hoàn toàn tự do, không borrow nào treo
            let result = self.insert_data(child_id, key, value);

            // Bước 3: mượn lại (LẦN 2, borrow mới, tách biệt bước 1) để chèn sep/new_child_id
            plan = match result {
                None => None,
                Some((sep, new_child_id)) => match &mut self.arena[node_id] {
                    Node::Internal { keys, children } => {
                        keys.insert(i, sep);
                        children.insert(i + 1, new_child_id);

                        if keys.len() <= self.order {
                            None
                        } else {
                            let mid = self.order / 2;
                            let new_keys = keys.split_off(mid);
                            let new_children = children.split_off(mid + 1);
                            let separator = keys.pop().unwrap();
                            let new_internal = Node::Internal {
                                keys: new_keys,
                                children: new_children,
                            };
                            Some((separator, new_internal))
                        }
                    }
                    Node::Leaf { .. } => unreachable!(),
                },
            };
        }

        // ---------- Ngoài mọi match/borrow rồi: giờ mới push ----------
        match plan {
            None => None,
            Some((separator, new_node)) => {
                let new_node_id = self.arena.len();
                self.arena.push(new_node);
                Some((separator, new_node_id))
            }
        }
    }

    /// Hàm public — bọc ngoài insert_data(), xử lý 2 việc insert_data không tự lo được:
    /// (1) cây rỗng lúc đầu (root = None) -> tự tạo 1 Leaf rỗng làm root trước.
    /// (2) insert_data() báo root bị split -> tạo root MỚI, cây cao thêm 1 tầng.
    pub fn insert(&mut self, key: K, value: V) {
        let root_id = match self.root {
            Some(id) => id,
            None => {
                let id = self.arena.len();
                self.arena.push(Node::Leaf {
                    keys: Vec::new(),
                    values: Vec::new(),
                    next: None,
                });
                self.root = Some(id);
                id
            }
        };

        if let Some((separator, new_id)) = self.insert_data(root_id, key, value) {
            let new_root_id = self.arena.len();
            self.arena.push(Node::Internal {
                keys: vec![separator],
                children: vec![root_id, new_id],
            });
            self.root = Some(new_root_id);
        }
    }
}

#[cfg(test)]
mod tests;
