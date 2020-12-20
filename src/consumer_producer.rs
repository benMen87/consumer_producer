use std::vec;

pub enum Capacity {
    Empty,
    Full,
}

pub trait ConsumerProducer<T> where T: Copy {
    fn read(&mut self) -> Result<T, Capacity>;
    fn write(&mut self, data: T) -> Result<u8, Capacity>;
    fn capacity(&self) -> usize;
}

pub struct CyclicBuffer<T> where T: Copy {
    size: usize,
    data: vec::Vec<T>,
    consumer: usize,
    producer: usize,
}

impl<T> CyclicBuffer<T> where T: Copy {
    pub fn new(size: usize) -> CyclicBuffer<T> {
        CyclicBuffer {
            size: size,
            data: vec::Vec::with_capacity(size),
            consumer: 0,
            producer: 0,
        }
    }

    pub fn buff_full_capacity(&self) -> usize {
        self.size
    }

    pub fn get_data(&self) -> vec::Vec<T> {
        self.data.clone()
    }

    fn increment(&self, value: usize) -> usize {
        (value + 1) % self.size
    }

    pub fn is_empty(&self) -> bool {
        self.consumer == self.producer
    }

    pub fn is_full(&self) -> bool {
        self.increment(self.producer) == self.consumer
    }
}

impl<T> ConsumerProducer<T> for CyclicBuffer<T> where T: Copy {

    fn read(&mut self) -> Result<T, Capacity> {
        if self.is_empty() {
            return Err(Capacity::Empty);
        }
        let return_data = self.data[self.consumer];
        self.consumer = self.increment(self.consumer);
        Ok(return_data)
    }

    fn write(&mut self, data: T) -> Result<u8, Capacity> {
        if self.is_full() {
            return Err(Capacity::Full)
        }
        self.data.insert(self.producer, data);
        self.producer = self.increment(self.producer);
        Ok(1)
    }

    fn capacity(&self) -> usize {
        self.size - (self.producer - self.consumer)
    }
}

