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
        let bytes = self.heap.get(*tid)?;
        match bytes {
            Some(b) => {
                let row = Row::decode(b)?;

                return Ok(Some(row));
            }
            None => Ok(None),
        }
        // Ok(Some(row)) sai
    }

    pub fn open(path: &str) -> Result<Self, DbError> {
        let heap = Heap::open(path)?;
        let mut index = BPlusTree::new(4);
        for page_no in 0..heap.num_pages() {
            let num_slots = heap.get_page_slots(page_no).unwrap_or(0);
            for slot in 0..num_slots {
                let tid = (page_no, slot);
                if let Some(bytes) = heap.get(tid)? {
                    let row = Row::decode(bytes)?;
                    index.insert(row.id, tid);
                }
            }
        }
        Ok(Self { heap, index })
        let row = Row::decode(&bytes)?;
        Ok(Some(row))
        let bytes = self.heap.get(*tid)?;
        match bytes {
            Some(b) => {
                let row = Row::decode(b)?;

                return Ok(Some(row));
            }
            None => Ok(None),
        }
        // Ok(Some(row)) sai
    }

    pub fn open(path: &str) -> Result<Self, DbError> {
        let heap = Heap::open(path)?;
        let mut index = BPlusTree::new(4);
        for page_no in 0..heap.num_pages() {
            let num_slots = heap.get_page_slots(page_no).unwrap_or(0);
            for slot in 0..num_slots {
                let tid = (page_no, slot);
                if let Some(bytes) = heap.get(tid)? {
                    let row = Row::decode(bytes)?;
                    index.insert(row.id, tid);
                }
            }
        }
        Ok(Self { heap, index })
    }
}

#[cfg(test)]
mod tests;
