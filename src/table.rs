use crate::btree::BPlusTree;
use crate::error::DbError;
use crate::heap::Heap;
use crate::heap::Tid;
use crate::row::Row;

pub struct Table {
    heap: Heap,
    index: BPlusTree<i64, Tid>,
}

impl Table {
    pub fn create(path: &str) -> Result<Self, DbError> {
        let heap = Heap::create(path)?;
        let index = BPlusTree::new(4);
        Ok(Self { heap, index })
    }

    pub fn insert(&mut self, row: Row) -> Result<(), DbError> {
        let bytes = row.encode();
        let tid = self.heap.insert(&bytes)?;
        self.index.insert(row.id, tid);
        Ok(())
    }

    pub fn get(&self, id: i64) -> Result<Option<Row>, DbError> {
        let tid = match self.index.get(&id) {
            Some(tid) => tid,
            None => return Ok(None),
        };
        let bytes = self.heap.get(tid)?;
        let row = Row::decode(&bytes)?;
        Ok(Some(row))
    }
}

#[cfg(test)]
mod tests;
