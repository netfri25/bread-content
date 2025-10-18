use derive_more::Display;

#[derive(Clone, Copy, Display)]
#[display("{_0:06X}")]
pub struct Color(pub u32);

impl Color {
    pub const RED: Self = Self(0xaa2222);
    pub const YELLOW: Self = Self(0x888800);
    pub const GREEN: Self = Self(0x22aa22);
    pub const GRAY: Self = Self(0x181818);
}
