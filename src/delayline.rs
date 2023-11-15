pub struct DelayLine {
    buffer: Vec<f64>,
    length: usize,
    index: usize
}

impl DelayLine {
    pub fn new(buffer_length: usize) -> Self{
        Self { buffer: vec![0.0; buffer_length], length: buffer_length, index: 0 }
    }
    
    pub fn read(&self) -> f64 {
        self.buffer[self.index]
    }

    pub fn write_and_advance(&mut self, sample: &f64) {
        self.buffer[self.index] = *sample;
        self.index += 1;
        self.index %= self.length;
    }

    pub fn clear(&mut self) {
        for value in self.buffer.iter_mut() {
            *value = 0.0;
        }
        self.index = 0;
    }


}