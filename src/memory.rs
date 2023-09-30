pub const MEMORY_CAPACITY: usize = 0x50_0000;

pub trait Memory {
    fn read_at_address_byte(&self, address: u32) -> Option<u8>;
    fn read_at_address_word(&self, address: u32) -> Option<u16>;
    fn read_at_address_long(&self, address: u32) -> Option<u32>;

    fn write_at_address_byte(&mut self, address: u32, data: u8) -> Result<(), &str>;
    fn write_at_address_word(&mut self, address: u32, data: u16) -> Result<(), &str>;
    fn write_at_address_long(&mut self, address: u32, data: u32) -> Result<(), &str>;

    fn iter(&mut self, address: u32) -> MemoryIter<Self> {
        MemoryIter {
            mem: self,
            next_address: address,
        }
    }
}

pub struct MemoryIter<'a, M: Memory + ?Sized> {
    mem: &'a mut M,
    next_address: u32,
}

impl<M: Memory + ?Sized> Iterator for MemoryIter<'_, M> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_address % 2 == 0 {
            let current_addr = self.next_address;
            self.next_address += 2;
            Some(
                self.mem
                    .read_at_address_word(current_addr)
                    .ok_or("Failure while reading address")
                    .unwrap(),
            )
        } else {
            todo!("Handle wrong addressing");
        }
    }
}

impl Memory for [u8] {
    fn read_at_address_byte(&self, address: u32) -> Option<u8> {
        let address = address as usize;
        if self.len() >= address {
            Some(self[address])
        } else {
            None
        }
    }

    fn read_at_address_word(&self, address: u32) -> Option<u16> {
        let address = address as usize;
        if self.len() >= address + 1 {
            Some((self[address] as u16) << 8 | self[address + 1] as u16)
        } else {
            None
        }
    }

    fn read_at_address_long(&self, address: u32) -> Option<u32> {
        let address = address as usize;
        if self.len() >= address + 3 {
            Some(
                (self[address] as u32) << 24
                    | (self[address + 1] as u32) << 16
                    | (self[address + 2] as u32) << 8
                    | self[address + 3] as u32,
            )
        } else {
            None
        }
    }

    fn write_at_address_byte(&mut self, address: u32, data: u8) -> Result<(), &str> {
        let address = address as usize;
        if self.len() >= address {
            self[address] = data;
            Ok(())
        } else {
            Err("Address out of Bounds")
        }
    }

    fn write_at_address_word(&mut self, address: u32, data: u16) -> Result<(), &str> {
        let address = address as usize;
        if self.len() >= address + 1 {
            self[address] = (data >> 8) as u8;
            self[address + 1] = data as u8;
            Ok(())
        } else {
            Err("Address out of Bounds")
        }
    }

    fn write_at_address_long(&mut self, address: u32, data: u32) -> Result<(), &str> {
        let address = address as usize;
        if self.len() >= address + 3 {
            self[address] = (data >> 24) as u8;
            self[address + 1] = (data >> 16) as u8;
            self[address + 2] = (data >> 8) as u8;
            self[address + 3] = data as u8;
            Ok(())
        } else {
            Err("Address out of Bounds")
        }
    }
}

impl Memory for [u16] {
    fn read_at_address_byte(&self, address: u32) -> Option<u8> {
        let address = address as usize;
        if self.len() >= address {
            Some((self[address] >> 8) as u8)
        } else {
            None
        }
    }

    fn read_at_address_word(&self, address: u32) -> Option<u16> {
        let address = address as usize;
        if self.len() >= address {
            Some(self[address])
        } else {
            None
        }
    }

    fn read_at_address_long(&self, address: u32) -> Option<u32> {
        let address = address as usize;
        if self.len() >= address + 1 {
            Some((self[address] as u32) << 16 | self[address + 1] as u32)
        } else {
            None
        }
    }

    fn write_at_address_byte(&mut self, address: u32, data: u8) -> Result<(), &str> {
        let address = address as usize >> 1;
        if self.len() >= address {
            self[address] = (self[address] & 0x00FF) | ((data as u16) << 8);
            Ok(())
        } else {
            Err("Address out of Bounds")
        }
    }

    fn write_at_address_word(&mut self, address: u32, data: u16) -> Result<(), &str> {
        let address = address as usize >> 1;
        if self.len() >= address {
            self[address] = data;
            Ok(())
        } else {
            Err("Address out of Bounds")
        }
    }

    fn write_at_address_long(&mut self, address: u32, data: u32) -> Result<(), &str> {
        let address = address as usize >> 1;
        if self.len() >= address + 1 {
            self[address] = (data >> 16) as u16;
            self[address + 1] = data as u16;
            Ok(())
        } else {
            Err("Address out of Bounds")
        }
    }
}

impl Memory for Box<[u8]> {
    fn read_at_address_byte(&self, address: u32) -> Option<u8> {
        let address = address as usize;
        if self.len() >= address {
            Some(self[address])
        } else {
            None
        }
    }

    fn read_at_address_word(&self, address: u32) -> Option<u16> {
        let address = address as usize;
        if self.len() >= address + 1 {
            Some((self[address] as u16) << 8 | self[address + 1] as u16)
        } else {
            None
        }
    }

    fn read_at_address_long(&self, address: u32) -> Option<u32> {
        let address = address as usize;
        if self.len() >= address + 3 {
            Some(
                (self[address] as u32) << 24
                    | (self[address + 1] as u32) << 16
                    | (self[address + 2] as u32) << 8
                    | self[address + 3] as u32,
            )
        } else {
            None
        }
    }

    fn write_at_address_byte(&mut self, address: u32, data: u8) -> Result<(), &str> {
        let address = address as usize;
        if self.len() >= address {
            self[address] = data;
            Ok(())
        } else {
            Err("Address out of Bounds")
        }
    }

    fn write_at_address_word(&mut self, address: u32, data: u16) -> Result<(), &str> {
        let address = address as usize;
        if self.len() >= address + 1 {
            self[address] = (data >> 8) as u8;
            self[address + 1] = data as u8;
            Ok(())
        } else {
            Err("Address out of Bounds")
        }
    }

    fn write_at_address_long(&mut self, address: u32, data: u32) -> Result<(), &str> {
        let address = address as usize;
        if self.len() >= address + 3 {
            self[address] = (data >> 24) as u8;
            self[address + 1] = (data >> 16) as u8;
            self[address + 2] = (data >> 8) as u8;
            self[address + 3] = data as u8;
            Ok(())
        } else {
            Err("Address out of Bounds")
        }
    }
}
