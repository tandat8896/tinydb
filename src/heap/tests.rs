use super::*;

// Helper: mỗi test cần 1 file riêng biệt (vì Heap::create mở file thật trên đĩa),
// xoá file cũ (nếu sót lại từ lần chạy trước) trước khi tạo mới, để test chạy lại
// nhiều lần không bị lẫn dữ liệu cũ.
fn temp_path(name: &str) -> String {
    let mut p = std::env::temp_dir();
    p.push(format!("tinydb_heap_test_{}.db", name));
    let _ = std::fs::remove_file(&p);
    p.to_str().unwrap().to_string()
}

#[test]
fn create_then_insert_and_get_roundtrip() {
    let mut h = Heap::create(&temp_path("roundtrip")).unwrap();
    let tid = h.insert(b"hello").unwrap();
    assert_eq!(h.get(tid).unwrap(), Some(&b"hello"[..]));
}

#[test]
fn insert_enough_to_spill_into_second_page() {
    let mut h = Heap::create(&temp_path("spill")).unwrap();
    let big = vec![7u8; 1000];
    let mut tids = Vec::new();
    for _ in 0..20 {
        tids.push(h.insert(&big).unwrap());
    }
    let spilled_to_page1 = tids.iter().any(|(page_no, _)| *page_no == 1);
    assert!(
        spilled_to_page1,
        "phải có ít nhất 1 Tid nằm ở page thứ 2 (page_no = 1)"
    );
}

#[test]
fn get_reads_correct_data_across_multiple_pages() {
    let mut h = Heap::create(&temp_path("multi_page_get")).unwrap();
    let big = vec![9u8; 1000];
    let mut tids = Vec::new();
    for _ in 0..20 {
        tids.push(h.insert(&big).unwrap());
    }
    for tid in tids {
        assert_eq!(h.get(tid).unwrap(), Some(&big[..]));
    }
}

#[test]
fn num_pages_matches_actual_page_count() {
    let mut h = Heap::create(&temp_path("num_pages")).unwrap();
    assert_eq!(h.num_pages(), 0); // chưa insert gì, HWM = 0

    let big = vec![7u8; 1000];
    for _ in 0..20 {
        h.insert(&big).unwrap();
    }
    assert!(h.num_pages() >= 2, "phải tràn sang ít nhất page thứ 2");
}

#[test]
fn get_with_invalid_page_no_returns_none() {
    let mut h = Heap::create(&temp_path("invalid_tid")).unwrap();
    h.insert(b"hello").unwrap();
    assert_eq!(h.get((99, 0)).unwrap(), None); // page_no 99 chưa từng tồn tại
}
