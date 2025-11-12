use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
pub struct Counter {
    pub count: u8,
    _padding: [u8; 7],
}

impl Counter {
    pub const COUNTER_SEEDS: &[u8] = b"counter";
    pub const LEN: usize = core::mem::size_of::<Self>();
}
