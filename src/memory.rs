pub trait Addressable {
    fn read(&self, address: u16) -> Option<u8>;
    fn write(&mut self, address: u16, value: u8) -> bool;

    fn read16(&self, address: u16) -> Option<u16> {
        if let Some(x0) = self.read(address) {
            if let Some(x1) = self.read(address + 1) {
                return Some((x0 as u16) | ((x1 as u16) << 8));
            }
        }
        None
    }

    fn write16(&mut self, address: u16, value: u16) -> bool {
        let lower = value & 0xff;
        let upper = (value & 0xff00) >> 8;

        return self.write(address, lower as u8) && self.write(address + 1, upper as u8);
    }

    fn copy(&mut self, from: u16, to: u16, size: usize) -> bool {
        for i in 0..size {
            if let Some(x) = self.read((from + (i as u16)) as u16) {
                if !self.write((to + (i as u16)) as u16, x) {
                    return false;
                }
            }
        }
        true
    }
}

pub struct MachineMemory {
    bytes: Vec<u8>,
    size: usize,
}

impl MachineMemory {
    pub fn new(u: usize) -> Self {
        Self {
            bytes: vec![0; u],
            size: u,
        }
    }
}

impl Addressable for MachineMemory {
    fn read(&self, address: u16) -> Option<u8> {
        self.bytes.get(address as usize).copied()
    }

    fn write(&mut self, address: u16, value: u8) -> bool {
        if (address as usize) < self.size {
            self.bytes[address as usize] = value;
            true
        } else {
            false
        }
    }
}
