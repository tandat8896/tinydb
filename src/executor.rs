use crate::error::DbError;
use crate::query::Stmt;
use crate::row::Row;
use crate::table::Table;

pub fn execute(stmt: Stmt, table: &mut Table) -> Result<String, DbError> {
    match stmt {
        Stmt::Insert { table: _, id, fields } => {
            let row = Row { id, fields };
            table.insert(row)?;
            Ok(format!("Inserted row with id={}", id))
        }
        Stmt::Select { table: _, where_id } => match where_id {
            Some(id) => match table.get(id)? {
                Some(row) => Ok(format!("{:?}", row)),
                None => Ok(format!("Row with id={} not found", id)),
            },
            None => Ok("SELECT * not implemented yet".into()),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::table::Table;

    #[test]
    fn test_execute_insert() {
        let mut table = Table::create("/tmp/test_executor").unwrap();
        let stmt = Stmt::Insert {
            table: "users".into(),
            id: 1,
            fields: vec!["alice".into()],
        };
        let result = execute(stmt, &mut table).unwrap();
        assert!(result.contains("1"));
    }

    #[test]
    fn test_execute_select() {
        let mut table = Table::create("/tmp/test_executor2").unwrap();
        let stmt = Stmt::Insert {
            table: "users".into(),
            id: 1,
            fields: vec!["alice".into()],
        };
        execute(stmt, &mut table).unwrap();

        let stmt = Stmt::Select {
            table: "users".into(),
            where_id: Some(1),
        };
        let result = execute(stmt, &mut table).unwrap();
        assert!(result.contains("alice"));
    }
}
