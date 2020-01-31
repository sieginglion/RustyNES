pub struct Bus {
    ram: [u8; 1024 * 64]
}

impl Bus {
    pub fn new() -> Bus {
        return Bus {
            ram: [0; 1024 * 64]
        }
    }
    pub fn read(&self, addr: u16) -> u8 {
        return self.ram[addr as usize]
    }
    pub fn write(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data
    }
}
