pub trait Address: Copy {
    fn addr(&self) -> u64;

    fn add_offset(&self, offset: u64) -> u64 {
        self.addr().saturating_add(offset)
    }
    fn sub_offset(&self, offset: u64) -> u64 {
        self.addr().saturating_sub(offset)
    }
}

impl Address for u64 {
    fn addr(&self) -> u64 {
        *self
    }
}