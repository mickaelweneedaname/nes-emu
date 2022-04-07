use crate::traits::Device;
use rand::Rng;

pub struct RandomGenerator {
    address: u16,
    range: std::ops::Range<u8>,
    memory: *mut u8,
}

impl RandomGenerator {
    pub fn new(address: u16, range: std::ops::Range<u8>) -> Self {
        Self {
            address,
            range,
            memory: std::ptr::null_mut(),
        }
    }

    fn generate(&self) -> u8 {
        rand::thread_rng().gen_range(self.range.clone())
    }
}

impl Device for RandomGenerator {
    fn mapping_def(&self) -> std::ops::Range<usize> {
        usize::from(self.address)..usize::from(self.address + 1)
    }

    fn map(&mut self, memory: &mut [u8]) {
        self.memory = &mut memory[0]
    }

    /// # Safety
    /// Make sure that `memory` ptr is valid
    unsafe fn mem_read(&mut self) {}

    /// # Safety
    /// Make sure that `memory` ptr is valid
    unsafe fn mem_write(&mut self) {
        *self.memory = self.generate();
    }
}

mod tests {
    use super::*;
}
