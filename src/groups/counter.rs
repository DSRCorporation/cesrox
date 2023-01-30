use crate::primitives::codes::Codes;

pub struct Counter {
    payload_type: Codes,
    len: usize,
    data: String,
}

impl Counter {
    pub(crate) fn new(payload_type: Codes, len: usize, data: String) -> Self {
        Self {
            payload_type,
            len,
            data,
        }
    }

    pub(crate) fn pack(self) -> String {
        [
            self.payload_type.adjust_with_num(self.len as u16),
            self.data,
        ]
        .join("")
    }
}
