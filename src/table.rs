use crate::btree::BPlusTree;
use crate::error::DbError;
use crate::heap::Heap;
use crate::row::Row;
//prefereces vault checkpoint 6
pub struct Table {
    heap: Heap,
    index: BPlusTree,
}

impl Table {
    pub fn create(path: &str) -> Result<Self, DbError> {
        let heap = Heap::create(path)?;
        let index = BPlusTree::new(4);
        OK(Self { heap, index })
    }

    pub fn insert(&mut self, row: Row) -> Result<(), DbError> {
        let mut bytes = Row.encode();
        let tid = self.heap.insert(&byte)?;
        self.index.insert(row.id, tid);
        Ok(())
    }

    pub fn get(&self, id: i64) -> Result<Option<Row>, DbError> {
        let tid = match self.index.get(&id) {
            Some(tid) => tid,
            None => return Ok(None),
        };
        let bytes = self.heap.get(tid);

        let row = Row::decode(&byte);
        Ok(Some(row))
    }
}

#[cfg(test)]
mod tests;
