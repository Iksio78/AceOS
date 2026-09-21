pub struct Bitmap<'a> {
    pub data: &'a mut [u8],
    pub bits: usize,
}
impl<'a> Bitmap<'a>{
    pub fn new(data: &'a mut [u8]) -> Self {
        data.fill(0xFF);

        Self {
            bits: data.len() * 8,
            data,
        }
    }
    pub fn get(&self, index: usize) -> bool {
        let byte = index / 8;
        let bit = index % 8;

        (self.data[byte] & (1 << bit)) != 0
    }

    pub fn set(&mut self, index: usize, value: bool) {
        let byte = index / 8;
        let bit = index % 8;

        if value {
            self.data[byte] |= 1 << bit;
        } else {
            self.data[byte] &= !(1 << bit);
        }
    }

    pub fn is_used(&self, page: usize) -> bool {
        self.get(page)
    }

    pub fn mark_used(&mut self, page: usize) {
        self.set(page, true);
    }

    pub fn mark_free(&mut self, page: usize) {
        self.set(page, false);
    }

    pub fn find_first_zero(&self) -> Option<usize> {
    for i in 0..self.data.len() {
        let byte = self.data[i];

        if byte != u8::MAX {
            let zero = (!byte).trailing_zeros() as usize;
            let bit = i * 8 + zero;

            if bit < self.bits {
                return Some(bit);
            }
        }
    }

    None
}
}