use std::usize;

pub const PAGE_SIZE: usize = 8192;

pub struct Page {
    pub data: [u8; PAGE_SIZE],
    num_slots: u16,
    free_start: u16,
    free_end: u16,
}

impl Page {
    pub fn new() -> Self {
        Page {
            data: [0; PAGE_SIZE],
            num_slots: 0,
            free_start: 0,
            free_end: PAGE_SIZE as u16,
        }
    }
    pub fn insert_tuple(&mut self, bytes: &[u8]) -> Option<u16> {
        let slot_size: u16 = 4;
        let slot_idx = self.num_slots;
        let needed = slot_size + bytes.len() as u16;
        let pos = self.free_start as usize;
        if self.free_start + needed > self.free_end {
            return None;
        }
        self.free_end -= bytes.len() as u16;

        self.data[self.free_end as usize..][..bytes.len()].copy_from_slice(bytes);
        self.data[pos..pos + 2].copy_from_slice(&self.free_end.to_le_bytes());
        self.data[pos + 2..pos + 4].copy_from_slice(&(bytes.len() as u16).to_le_bytes());
        self.free_start += slot_size;
        self.num_slots += 1;
        Some(slot_idx)
    }
    pub fn get_tuple(&self, slot_idx: u16) -> Option<&[u8]> {
        if slot_idx >= self.num_slots {
            return None;
        }
        let pos = slot_idx as usize * 4;
        let offset = u16::from_le_bytes([self.data[pos], self.data[pos + 1]]);
        let length = u16::from_le_bytes([self.data[pos + 2], self.data[pos + 3]]);
        if offset == 0 {
            return None;
        }
        Some(&self.data[offset as usize..][..length as usize])
    }
    pub fn delete_tuple(&mut self, slot_idx: u16) -> Option<&[u8]> {
        todo!()
        // 1. đọc offset, length từ slot entry (giống get_tuple)
        // 2. copy data ra Vec: let data = self.data[offset..][..length].to_vec();
        // 3. set offset = 0 trong slot: self.data[pos..pos+2] = [0, 0];
        // 4. trả về Some(data)
    }
    pub fn free_space(&self) -> usize {
        (self.free_end - self.free_start) as usize
    }

    /// Số slot đang có trong page — cần cho Heap/Table sau này để biết duyệt tới đâu.
    pub fn num_slots(&self) -> u16 {
        self.num_slots
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(p.num_slots(), slots_before, "insert thất bại không được làm tăng num_slots");
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
}
