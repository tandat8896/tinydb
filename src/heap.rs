use crate::error::DbError;
use crate::page::{PAGE_SIZE, Page};
use std::io::{Seek, SeekFrom, Write};
use std::usize;

pub type Tid = (u32, u16); // (page_no, slot)

pub struct Heap {
    file: std::fs::File,
    pages: Vec<Page>, // cache RAM — LUÔN đồng bộ 1-1 với file (write-through, xem insert() bên dưới)
}

impl Heap {
    pub fn create(path: &str) -> Result<Self, DbError> {
        // 1. Mở file tại `path` bằng std::fs::OpenOptions.
        // 2. `pages` khởi tạo rỗng: Vec::new() — Heap mới toanh, high-water mark = 0,
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
        // 3. Trả Ok(Heap { file, pages }).
        Ok(Heap {
            file,
            pages: Vec::new(),
        })
    }

    /// Thuật toán (đã chốt trong plan — free-list tuyến tính + high-water mark):
    pub fn insert(&mut self, bytes: &[u8]) -> Result<Tid, DbError> {
        // 1. Duyệt tuần tự self.pages (free-list đơn giản nhất): với từng page ở index i,
        for (page_numberof, page) in self.pages.iter_mut().enumerate() {
            if let Some(slot) = page.insert_tuple(bytes) {
                let offset = page_numberof as u64 * PAGE_SIZE as u64;
                self.file.seek(SeekFrom::Start(offset))?;
                self.file.write_all(&page.data)?;

            }
        }
        // 2. Nếu duyệt hết self.pages mà không page nào nhận được (hoặc self.pages đang rong
        let mut new_page = Page::new();
        let slot = new_page.insert_tuple(bytes).unwrap();
        self.pages.push(new_page);
        let page_numberof = self.pages.len() - 1;
        // 3. Ghi PAGE đã thay đổi (page_no ở trên) xuống file — write-through, ngay lập tức,không đợi gì cả:
        let offset = page_numberof as u64 * PAGE_SIZE as u64;
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.write_all((&self.pages[page_numberof].data))?;
        // 4. Trả Ok((page_no as u32, slot)).
        return Ok((page_numberof as u32, slot));
    }

    pub fn get(&self, tid: Tid) -> Result<Option<&[u8]>, DbError> {
        // 1. Destructure tid thành (page_no, slot).
        let (page_numberof, slot) = tid;
        // 2. Kiểm tra page_no có hợp lệ không: page_no as usize >= self.pages.len()
        //    => trả Ok(None) (không panic vì index ngoài phạm vi).
        if page_numberof as usize >= self.pages.len() {
            return Ok(None);
        }
        // 3. Lấy &self.pages[page_no as usize], gọi .get_tuple(slot) — hàm này đã có sẵn
        //    ở page.rs rồi, trả thẳng ra Option<&[u8]> luôn, chỉ cần bọc Ok(...) lại.
        return Ok(self.pages[page_numberof as usize].get_tuple(slot));

        // Chú ý: KHÔNG cần đọc lại từ file — vì insert() ở trên đã write-through, nghĩa là
        // self.pages trong RAM luôn khớp 100% với file, đọc thẳng từ RAM là đủ và nhanh hơn.
    }

    /// Cần cho Checkpoint 7 (REINDEX khi open): Table::open() phải biết heap có bao nhiêu
    /// page để duyệt hết. Chỉ đơn giản là độ dài của self.pages.
    pub fn num_pages(&self) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod tests;
