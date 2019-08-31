use std::io::{Read, Result, Write};

pub struct ReadStats<R>{
    reader : R,
    bytes: usize,
    count: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats{
            reader: wrapped,
            bytes: 0,
            count:0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.count
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes = self.reader.read(buf)?;
        self.bytes += bytes;
        self.count += 1;
        Ok(bytes)
    }
}

pub struct WriteStats<W>{
    writer: W,
    bytes: usize,
    count: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            writer: wrapped,
            bytes: 0,
            count:0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.count
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes = self.writer.write(buf)?;
        self.bytes += bytes;
        self.count +=1;
        Ok(bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
