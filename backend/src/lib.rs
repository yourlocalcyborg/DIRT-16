mod u24_module;
use u24_module::u24;

struct CpuState {
    r0: u16,
    r1: u16,
    r2: u16,
    r3: u16,
    pc: u24,
    sp: u24,
    sr: u8, //PQRSNVZC
    mem: [u8; 0x1000000],
}

impl CpuState {
    pub fn new(r0: u16, r1: u16, r2: u16, r3: u16, pc: u24, sp: u24, sr: u8, mem: [u8; 0x1000000]) -> Self {
	CpuState {r0, r1, r2, r3, pc, sp, sr, mem}
    }

    pub fn get_mem(self, addr: u24) -> u8 {
	self.mem[<u24 as Into<usize>>::into(addr)]
    }

    pub fn get_mem_u16(self, addr: u24) -> u16 {
	((self.mem[<u24 as Into<usize>>::into(addr)] as u16) << 8) + (self.mem[<u24 as Into<usize>>::into(addr)]) as u16
    }

    pub fn set_mem(&mut self, addr: u24, val: u8) {
	self.mem[<u24 as Into<usize>>::into(addr)] = val;
    }

    pub fn set_mem_u16(&mut self, addr: u24, val: u16) {
	self.set_mem(addr, (val >> 8) as u8);
	self.set_mem(addr + u24::new(1), (val & 0xff) as u8);
    }
}

#[cfg(tests)]
mod tests {
    #[test]
    fn new() {
	let state: CpuState = CpuState::new(0, 0, 0, 0, u24::new(0), u24::new(0), 0, [0; 0x1000000]);

	assert_eq!(state.r0, 0);
	assert_eq!(state.r1, 0);
	assert_eq!(state.r2, 0);
	assert_eq!(state.r3, 0);
	assert_eq!(state.pc, u24::new(0));
	assert_eq!(state.sp, u24::new(0));
	assert_eq!(state.sr, 0);
	assert_eq!(state.mem, [0; 0x1000000]);
    }

    fn mem() {
	let state: CpuState = CpuState::new(0, 0, 0, 0, u24::new(0), u24::new(0), 0, [0; 0x1000000]);

	assert_eq!(state.get_mem(0xff), 0);
	assert_eq!(state.get_mem_u16(0xff), 0);
	state.set_mem(0xff, 27);
	assert_eq!(state.get_mem(0xff), 27);
	state.set_mem_u16(0xff, 0xABCD);
	assert_eq!(state.get_mem_u16(0xff), 0xABCD);
    }
}
