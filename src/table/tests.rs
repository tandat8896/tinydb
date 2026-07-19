se super::*;

#[test]
fn create_insert_get() {
    let mut table = Table::create("/tmp/test_table").unwrap();
    let row = Row {
        id: 1,
        fields: vec!["hello".to_string()],
    };
    table.insert(row).unwrap();
    let result = table.get(1).unwrap();
    assert_eq!(result.unwrap().id, 1);
}

#[test]
fn open_reindex() {
    let path = "/tmp/test_open_reindex";
    let mut table = Table::create(path).unwrap();
    table
        .insert(Row {
            id: 1,
            fields: vec!["hello".to_string()],
        })
        .unwrap();
    drop(table);
    let table = Table::open(path).unwrap();
    let row = table.get(1).unwrap().unwrap();
    assert_eq!(row.id, 1);
    assert_eq!(row.fields[0], "hello")

use super::*;

#[test]
fn create_insert_get() {
    let mut table = Table::create("/tmp/test_table").unwrap();
    let row = Row {
        id: 1,
        fields: vec!["hello".to_string()],
    };
    table.insert(row).unwrap();
    let result = table.get(1).unwrap();
    assert_eq!(result.unwrap().id, 1);
}
