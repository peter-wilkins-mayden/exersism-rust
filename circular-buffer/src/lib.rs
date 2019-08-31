use std::fmt::Debug;

pub struct CircularBuffer<T: Clone + Debug> {
    buf: Vec<Option<T>>,
    push_idx: usize,
    pop_idx: usize,
    cap: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone + Debug> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: vec![None; capacity],
            push_idx: capacity,
            pop_idx: capacity,
            cap: capacity,
        }
    }
    fn write_index(&mut self) -> usize {
        let i = self.push_idx % self.cap;
        self.push_idx += 1;
        i
    }

    fn buf_full(&self) -> bool {
        self.push_idx - self.pop_idx == self.cap
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.buf_full() {
            return Err(Error::FullBuffer);
        }
        let i = self.write_index();
        self.buf[i] = Some(element);
        Ok(())
    }

    fn read_index(&mut self) -> usize {
        let i = self.pop_idx % self.cap;
        self.pop_idx += 1;
        i
    }

    pub fn read(&mut self) -> Result<T, Error> {
        let i = self.read_index();
        match self.buf[i].take() {
            Some(x) => { dbg!(&i, &x); Ok(x)},
            None => Err(Error::EmptyBuffer),
        }
    }

    pub fn clear(&mut self) {
        self.buf = vec![None; self.cap];
        self.push_idx = self.cap;
        self.pop_idx = self.cap;
    }

    pub fn overwrite(&mut self, element: T) {
        if self.buf_full() {
            let i = self.read_index();
            self.buf[i] =  Some(element);
        } else {
            self.write(element).unwrap();
        }
    }
}
