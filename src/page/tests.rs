use super::*;

#[test]
fn insert_and_get_roundtrip() {
    let mut p = Page::new();
    let slot = p.insert_tuple(b"hello").unwrap();
    assert_eq!(p.get_tuple(slot), Some(&b"hello"[..]));
}

#[test]
fn multiple_inserts_keep_correct_data_per_slot() {
    let mut p = Page::new();
    let s1 = p.insert_tuple(b"first").unwrap();
    let s2 = p.insert_tuple(b"second-longer").unwrap();
    let s3 = p.insert_tuple(b"3").unwrap();

    // Đọc lại theo thứ tự khác để chắc chắn không bị lẫn data giữa các slot
    assert_eq!(p.get_tuple(s3), Some(&b"3"[..]));
    assert_eq!(p.get_tuple(s1), Some(&b"first"[..]));
    assert_eq!(p.get_tuple(s2), Some(&b"second-longer"[..]));
}

#[test]
fn insert_fails_when_full_and_does_not_corrupt_state() {
    let mut p = Page::new();
    let big = vec![7u8; 1000];
    let mut count = 0;
    while p.insert_tuple(&big).is_some() {
        count += 1;
    }
    assert!(count > 0, "phải insert được ít nhất 1 lần trước khi đầy");

    // Insert thêm sau khi đầy: phải trả None, không panic, không tăng num_slots
    let slots_before = p.num_slots();
    assert_eq!(p.insert_tuple(&big), None);
    assert_eq!(
        p.num_slots(),
        slots_before,
        "insert thất bại không được làm tăng num_slots"
    );
}

#[test]
fn get_tuple_out_of_range_returns_none() {
    let p = Page::new();
    assert_eq!(p.get_tuple(0), None); // chưa insert gì, slot 0 chưa tồn tại
}

#[test]
fn num_slots_tracks_successful_inserts_only() {
    let mut p = Page::new();
    assert_eq!(p.num_slots(), 0);
    p.insert_tuple(b"a").unwrap();
    p.insert_tuple(b"b").unwrap();
    assert_eq!(p.num_slots(), 2);
}
#[test]
fn delete_tuple_marks_slot_as_dead() {
    let mut p = Page::new();
    let slot = p.insert_tuple(b"Hello").unwrap();
    assert_eq!(p.get_tuple(slot), Some(&b"Hello"[..]));
    let deleted = p.delete_tuple(slot).unwrap();
    assert_eq!(deleted, b"Hello");
    assert_eq!(p.get_tuple(slot), None);
}
#[test]
fn delete_does_not_reclaim_free_space() {
    let mut p = Page::new();
    let slot = p.insert_tuple(b"Hello").unwrap();
    let before = p.free_space();
    p.delete_tuple(slot).unwrap();
    let after = p.free_space();
    assert_eq!(before, after, "deleted hien tai chua reclaim space");
}

#[test]
fn delete_same_slot_twice() {
    let mut p = Page::new();
    let slot = p.insert_tuple(b"Hello").unwrap();
    assert!(p.delete_tuple(slot).is_some());
    assert_eq!(p.delete_tuple(slot), None);
}


#[test]
fn delete_out_of_range() {
    let mut p = Page::new();
    let slot = p.insert_tuple(b"Hello").unwrap();
    assert_eq!(p.delete_tuple(99), None);
}
