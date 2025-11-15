use super::MemoryRegion;

const BYTES_PER_ROW: u16 = 8;

#[derive(Debug)]
pub struct GraphicBufferRegion<'a> {
    start: u16,
    end: u16,
    data: &'a mut [u8],
}

impl<'a> GraphicBufferRegion<'a> {
    pub fn new(start: u16, end: u16, data: &'a mut [u8]) -> Self {
        GraphicBufferRegion { start, end, data }
    }

    #[must_use]
    pub fn get_pixel(&self, x: u8, y: u8) -> bool {
        let x = x as u16;
        let y = y as u16;

        let byte_index = y * BYTES_PER_ROW + (x / 8);
        let bit_pos = x % 8;

        let byte = self.read(byte_index);

        let mask = 0x80u8 >> bit_pos;

        byte & mask != 0
    }
}

impl MemoryRegion for GraphicBufferRegion<'_> {
    fn begin(&self) -> u16 {
        self.start
    }

    fn end(&self) -> u16 {
        self.end
    }

    fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }
}
