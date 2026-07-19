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
