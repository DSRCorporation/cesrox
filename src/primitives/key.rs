use zeroize::Zeroize;

#[derive(Debug, Clone, PartialEq)]
pub struct Key {
    value: Vec<u8>,
}

impl Key {
    pub fn new(value: &[u8]) -> Self {
        Key { value: value.to_vec() }
    }

    pub fn value(&self) -> &[u8] {
        self.value.as_slice()
    }
}

impl Drop for Key {
    fn drop(&mut self) {
        self.value.zeroize()
    }
}
