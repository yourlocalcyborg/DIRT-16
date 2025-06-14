#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct u24 {
    val: u32
}

impl u24 {
    pub fn new(val: u32) -> Self {
	u24 { val: val.clamp(0, 0b111111111111111111111111) }
    }
}

impl std::ops::Add for u24 {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
	Self::new(self.val + other.val)
    }
}

impl From<u32> for u24 {
    fn from(val: u32) -> Self {
	Self::new(val)
    }
}

impl From<u16> for u24 {
    fn from(val: u16) -> Self {
	Self::new(val as u32)
    }
}

impl From<usize> for u24 {
    fn from(val: usize) -> Self {
	Self::new(val as u32)
    }
}

impl Into<u32> for u24 {
    fn into(self) -> u32 {
	self.val as u32
    }
}

impl Into<u16> for u24 {
    fn into(self) -> u16 {
	self.val as u16
    }
}

impl Into<usize> for u24 {
    fn into(self) -> usize {
	self.val as usize
    }
}
