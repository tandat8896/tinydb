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
            free_start: 6,
            free_end: PAGE_SIZE as u16,
        }
    }

    pub fn sync_header(&mut self) {
        self.data[0..2].copy_from_slice(&self.num_slots.to_le_bytes());
        self.data[2..4].copy_from_slice(&self.free_start.to_le_bytes());
        self.data[4..6].copy_from_slice(&self.free_end.to_le_bytes());
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
        let pos = 6 + slot_idx as usize * 4;
        let offset = u16::from_le_bytes([self.data[pos], self.data[pos + 1]]);
        let length = u16::from_le_bytes([self.data[pos + 2], self.data[pos + 3]]);
        if offset == 0 {
            return None;
        }
        Some(&self.data[offset as usize..][..length as usize])
    }
    pub fn delete_tuple(&mut self, slot_idx: u16) -> Option<Vec<u8>> {
        // 1. đọc offset, length từ slot entry (giống get_tuple)
        if slot_idx >= self.num_slots() {
            return None;
        }
        // 2. copy data ra Vec: let data = self.data[offset..][..length].to_vec();
        let pos = 6 + slot_idx as usize * 4;
        // 3. set offset = 0 trong slot: self.data[pos..pos+2] = [0, 0];
        let offset = u16::from_le_bytes([self.data[pos], self.data[pos + 1]]);
        let length = u16::from_le_bytes([self.data[pos + 2], self.data[pos + 3]]);
        if offset == 0 {
            return None;
        }
        let tuple_data = self.data[offset as usize..][..length as usize].to_vec();
        self.data[pos..pos + 2].copy_from_slice(&[0, 0]);
        // 4. trả về Some(data)
        Some(tuple_data)
    }
    pub fn free_space(&self) -> usize {
        (self.free_end - self.free_start) as usize
    }

    /// Số slot đang có trong page — cần cho Heap/Table sau này để biết duyệt tới đâu.
    pub fn num_slots(&self) -> u16 {
        self.num_slots
    }

    pub fn from_data(data: [u8; PAGE_SIZE]) -> Self {
        let num_slots = u16::from_le_bytes([data[0], data[1]]);
        let free_start = u16::from_le_bytes([data[2], data[3]]);
        let free_end = u16::from_le_bytes([data[4], data[5]]);
        Page {
            data,
            num_slots,
            free_start,
            free_end,
        }
    }
}

#[cfg(test)]
mod tests;
