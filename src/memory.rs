use thiserror::Error;

#[derive(Debug, Error)]
pub enum MemoryError {
    #[error("Address out of bounds: 0x{0:08x}")]
    OutOfBounds(u32),
    #[error("Misaligned access at 0x{0:08x}")]
    MisalignedAccess(u32),
}

pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Memory {
            data: vec![0; size],
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn read_byte(&self, addr: u32) -> Result<i8, MemoryError> {
        let addr = addr as usize;
        if addr >= self.size() {
            return Err(MemoryError::OutOfBounds(addr as u32));
        }
        Ok(self.data[addr] as i8)
    }

    pub fn write_byte(&mut self, addr: u32, val: u8) -> Result<(), MemoryError> {
        let addr = addr as usize;
        if addr >= self.size() {
            return Err(MemoryError::OutOfBounds(addr as u32));
        }
        self.data[addr] = val;
        Ok(())
    }
    pub fn read_halfword(&self, addr: u32) -> Result<i16, MemoryError> {
        if addr & 1 != 0 {
            return Err(MemoryError::MisalignedAccess(addr));
        }
        let addr = addr as usize;
        if addr + 1 >= self.size() {
            return Err(MemoryError::OutOfBounds(addr as u32));
        }
        Ok(i16::from_le_bytes([self.data[addr], self.data[addr + 1]]))
    }

    pub fn write_halfword(&mut self, addr: u32, val: u16) -> Result<(), MemoryError> {
        if addr & 1 != 0 {
            return Err(MemoryError::MisalignedAccess(addr));
        }
        let addr = addr as usize;
        if addr + 1 >= self.size() {
            return Err(MemoryError::OutOfBounds(addr as u32));
        }
        let bytes = val.to_le_bytes();
        self.data[addr] = bytes[0];
        self.data[addr + 1] = bytes[1];
        Ok(())
    }
    pub fn read_word(&self, addr: u32) -> Result<i32, MemoryError> {
        if !addr.is_multiple_of(4) {
            return Err(MemoryError::MisalignedAccess(addr));
        }
        let addr = addr as usize;
        if addr + 3 >= self.size() {
            return Err(MemoryError::OutOfBounds(addr as u32));
        }
        Ok(i32::from_le_bytes([
            self.data[addr],
            self.data[addr + 1],
            self.data[addr + 2],
            self.data[addr + 3],
        ]))
    }
    pub fn write_word(&mut self, addr: u32, val: i32) -> Result<(), MemoryError> {
        if !addr.is_multiple_of(4) {
            return Err(MemoryError::MisalignedAccess(addr));
        }
        let addr = addr as usize;
        if addr + 3 >= self.size() {
            return Err(MemoryError::OutOfBounds(addr as u32));
        }
        let bytes = val.to_le_bytes();
        self.data[addr] = bytes[0];
        self.data[addr + 1] = bytes[1];
        self.data[addr + 2] = bytes[2];
        self.data[addr + 3] = bytes[3];
        Ok(())
    }
}
