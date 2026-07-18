use crate::btree::BPlusTree;
use crate::heap::Heap;

pub struct Table {
    heap: Heap,
    index: BPlusTree,
}

#[cfg(test)]
mod tests;
