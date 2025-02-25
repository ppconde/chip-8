pub struct Ram {
    memory: [u8; 4096],
}

impl Ram {
    pub fn new() -> Self {
        Self { memory: [0; 4096] }
    }
}
