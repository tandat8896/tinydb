use crate::error::DbError;

#[derive(Debug)]
pub struct Row {
    pub id: i64,
    pub fields: Vec<String>,
}

impl Row {
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend(&self.id.to_le_bytes());
        buf.extend(&(self.fields.len() as u32).to_le_bytes());
        for f in &self.fields {
            buf.extend(&(f.len() as u32).to_le_bytes());
            buf.extend(f.as_bytes());
        }
        buf
    }

    pub fn decode(_bytes: &[u8]) -> Result<Self, DbError> {
        let id = i64::from_le_bytes(_bytes[..8].try_into().unwrap());
        let num = u32::from_le_bytes(_bytes[8..12].try_into().unwrap());
        let mut pos = 12;
        let mut fields = Vec::new();
        for _ in 0..num {
            let len = u32::from_le_bytes(_bytes[pos..pos + 4].try_into().unwrap()) as usize;
            pos += 4;
            let s = String::from_utf8(_bytes[pos..pos + len].to_vec())
                .map_err(|_| DbError::Parse("UTF-8".into()))?;
            fields.push(s);
            pos += len;
        }
        Ok(Row { id, fields })
    }
}
