pub struct CPU {
    A: u8, // accumulator
    X: u8, // x index
    Y: u8, // y index
    S: u8, // stack pointer
    P: u16, // program counter

    carry: bool,
    zero: bool,
    IRQ_off: bool,
    decimal: bool,
    BRK: bool,
    overflow: bool,
    negative: bool,

    addr_abs: u16,
    addr_rel: u16,
    opcode: u8
}

impl CPU {
    pub fn RES() {

    }
    pub fn IRQ() {

    }
    pub fn NMI() {

    }
    fn fetch() {

    }

    fn IMP(&mut self) {
        self.fetched = self.A
    }
    fn IMM(&mut self) {
        self.addr_abs = self.P++
    }
    fn ZP0(&mut self) {
        self.addr_abs = self.read(self.P);
        self.addr_abs &= 0x00ff;
        self.P++
    }
    fn ZPX(&mut self) {
        self.addr_abs = self.read(self.P) + self.X;
        self.addr_abs &= 0x00ff;
        self.P++
    }
    fn ZPY(&mut self) {
        self.addr_abs = self.read(self.P) + self.Y ;
        self.addr_abs &= 0x00ff;
        self.P++
    }
    fn ABS(&mut self) {
        let lo: u16 = self.read(self.P);
        self.P++;
        let hi: u16 = self.read(self.P);
        self.P++;
        self.addr_abs = hi << 8 | lo
    }
    fn ABX(&mut self) {
        let lo: u16 = self.read(self.P);
        self.P++;
        let hi: u16 = self.read(self.P);
        self.P++;
        self.addr_abs = hi << 8 | lo + self.X
    }
    fn ABY(&mut self) {
        let lo: u16 = self.read(self.P);
        self.P++;
        let hi: u16 = self.read(self.P);
        self.P++;
        self.addr_abs = hi << 8 | lo + self.Y 
    }
    fn IND(&mut self) {
        let lo: u16 = self.read(self.P);
        self.P++;
        let hi: u16 = self.read(self.P);
        self.P++;
        let pointer = hi << 8 | lo;
        self.addr_abs = read(pointer + 1) << 8 | read(pointer)
    }

    fn fetch(&mut self) {
        self.fetched = read(self.addr_abs)
    }
    fn AND(&mut self) {
        self.fetch();
        self.A = self.A && self.read(self.addr_abs);
        self.zero = self.A == 0
    }
    fn BCC(&mut self) {
        if !self.carry {
            self.addr_abs = self.P + self.addr_rel;
            self.P = self.addr_abs
        }
    }
    fn BCS(&mut self) {
        if self.carry {
            self.addr_abs = self.P + self.addr_rel;
            self.P = self.addr_abs
        }
    }
    fn BEQ(&mut self) {
        if self.zero {
            self.addr_abs = self.P + self.addr_rel;
            self.P = self.addr_abs
        }
    }
    fn BNE(&mut self) {
        if !self.zero {
            self.addr_abs = self.P + self.addr_rel;
            self.P = self.addr_abs
        }
    }
    fn BMI(&mut self) {
        if self.negative {
            self.addr_abs = self.P + self.addr_rel;
            self.P = self.addr_abs
        }
    }
    fn BPL(&mut self) {
        if !self.negative {
            self.addr_abs = self.P + self.addr_rel;
            self.P = self.addr_abs
        }
    }
    fn BVS(&mut self) {
        if self.overflow {
            self.addr_abs = self.P + self.addr_rel;
            self.P = self.addr_abs
        }
    }
    fn BVC(&mut self) {
        if !self.overflow {
            self.addr_abs = self.P + self.addr_rel;
            self.P = self.addr_abs
        }
    }
    fn CLC(&mut self) {
        self.carry = false
    }
    fn CLC(&mut self) {
        self.decimal = false
    }
    fn ADC(&mut self) {
        let temp: u16 = self.A as u16 + self.fetched as u16 + self.overflow as u16
        if temp > 255 {
            self.carry = true
        }
        if !(temp & 0x00FF) {
            self.zero = true
        }
        if !((self.A as u16 ^ self.fetched as u16) & (self.A as u16 ^ self.temp)) & 0x0080 {
            self.overflow = true
        }
        self.A = temp & 0x00FF
    }
    fn SBC(&mut self) {
        self.fetch();
        let value: u16 = self.fetched as u16 ^ 0x00FF;
        let temp: u16 = self.A as u16 + value + self.carry as u16;
        if !(temp & 0x00FF) {
            self.zero = true
        }
        if (temp ^ self.A as u16) & (temp ^ value) & 0x0080 {
            self.overflow = true
        }
        if temp & 0x0080 {
            self.negative = true
        }
        self.A = temp & 0x00FF
    }
    fn PHA(&mut self) {
        bus.write(0x0100 + self.S, self.A);
        self.S--
    }
    fn PLA(&mut self) {
        self.S++;
        self.A = bus.read(0x0100 + self.S);
        self.set_zero();
        self.set_negative();
    }
    fn RES() {
        self.A = 0;
        self.X = 0;
        self.Y = 0;
        self.S = 0xFD;
        
    }
}
