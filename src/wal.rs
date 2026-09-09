use crate::error::DbError;
use crate::row::Row;
use crate::table::Table;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufWriter};

pub struct Wal {
    writer: BufWriter<File>,
}

impl Wal {
    pub fn open(path: &str) -> Result<Self, DbError> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        Ok(Self {
            writer: BufWriter::new(file),
        })
    }

    pub fn append_insert(&mut self, row: &Row) -> Result<(), DbError> {
        let bytes = row.encode();
        self.writer.write_all(&(bytes.len() as u32).to_le_bytes())?;
        self.writer.write_all(&bytes)?;
        self.writer.flush()?;
        // fsync for durability
        self.writer.get_ref().sync_all()?;
        Ok(())
    }
}

pub struct Transaction<'a> {
    table: &'a mut Table,
    wal: &'a mut Wal,
    pending: Vec<Row>,
}

impl<'a> Transaction<'a> {
    pub fn begin(table: &'a mut Table, wal: &'a mut Wal) -> Self {
        Self {
            table,
            wal,
            pending: Vec::new(),
        }
    }

    pub fn insert(&mut self, row: Row) {
        self.pending.push(row);
    }

    pub fn commit(mut self) -> Result<(), DbError> {
        // Write-Ahead: WAL FIRST
        for row in &self.pending {
            self.wal.append_insert(row)?;
        }
        // THEN apply to heap/index
        for row in self.pending {
            self.table.insert(row)?;
        }
        Ok(())
    }

    pub fn rollback(self) {
        // Drop buffer — no changes applied
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_rollback() {
        let path = "/tmp/test_wal_rollback";
        let mut table = Table::create(path).unwrap();
        let mut wal = Wal::open(&format!("{}.wal", path)).unwrap();

        let mut txn = Transaction::begin(&mut table, &mut wal);
        txn.insert(Row {
            id: 1,
            fields: vec!["alice".into()],
        });
        txn.rollback();

        assert!(table.get(1).unwrap().is_none());
    }

    #[test]
    fn test_transaction_commit() {
        let path = "/tmp/test_wal_commit";
        let mut table = Table::create(path).unwrap();
        let mut wal = Wal::open(&format!("{}.wal", path)).unwrap();

        let mut txn = Transaction::begin(&mut table, &mut wal);
        txn.insert(Row {
            id: 1,
            fields: vec!["alice".into()],
        });
        txn.commit().unwrap();

        let row = table.get(1).unwrap().unwrap();
        assert_eq!(row.id, 1);
        assert_eq!(row.fields[0], "alice");
    }

    #[test]
    fn test_wal_multiple_inserts() {
        let path = "/tmp/test_wal_multi";
        let mut table = Table::create(path).unwrap();
        let mut wal = Wal::open(&format!("{}.wal", path)).unwrap();

        let mut txn = Transaction::begin(&mut table, &mut wal);
        for i in 1..=5 {
            txn.insert(Row {
                id: i,
                fields: vec![format!("user_{}", i)],
            });
        }
        txn.commit().unwrap();

        for i in 1..=5 {
            let row = table.get(i).unwrap().unwrap();
            assert_eq!(row.fields[0], format!("user_{}", i));
        }
    }
}
