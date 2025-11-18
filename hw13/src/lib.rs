use std::sync::{Arc, Mutex};

#[derive(Debug, PartialEq)]
pub enum WriteError {
    NoSpaceLeft,
}

pub struct RingBuffer {
    read_idx: usize,
    write_idx: usize,
    is_full: bool,
    data: Vec<u8>,
}

impl RingBuffer {
    pub fn new(size: usize) -> Self {
        RingBuffer {
            read_idx: 0,
            write_idx: 0,
            is_full: false,
            data: vec![0; size],
        }
}

pub fn write(&mut self, buf: &str) -> Result<usize, WriteError> {
    if self.is_full {
        return Err(WriteError::NoSpaceLeft);
    }

    let buf = buf.as_bytes();

    let capacity = self.data.len();

    let len: usize = if !self.is_full {
        (self.write_idx + capacity - self.read_idx) % capacity
    } else {
        capacity
    };

    let free_space = self.data.len() - len;
    let bytes_to_write = std::cmp::min(buf.len(), free_space);

    for element in buf.iter().take(bytes_to_write) {
        self.data[self.write_idx] = *element;
        self.write_idx = (self.write_idx + 1) % capacity;
    }

    if self.write_idx == self.read_idx {
        self.is_full = true
    }

    Ok(bytes_to_write)
}

pub fn read(&mut self, b: usize) -> Option<String> {
    if self.read_idx == self.write_idx && !self.is_full {
        return None;
    }
    let mut result = vec![0; b];
    let capacity = self.data.len();

    for element in result.iter_mut().take(b) {
        *element = self.data[self.read_idx];
        self.read_idx = (self.read_idx + 1) % capacity;
    }

    self.is_full = false;

    Some(String::from_utf8_lossy(&result).to_string())
    }
}

#[derive(Clone)]
pub struct ThreadSafeRingBuffer {
    inner: Arc<Mutex<RingBuffer>>,
}

impl ThreadSafeRingBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            inner: Arc::new(Mutex::new(RingBuffer::new(size)))
        }
    }
    pub fn write(&self, buf: &str)  -> Result<usize, WriteError> {
        let mut guard = self.inner.lock().unwrap();
        guard.write(buf)
    }

    pub fn read(&self, b: usize) -> Option<String> {
        self.inner.lock().unwrap().read(b)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::thread::scope;
    use super::*;

    #[test]
    fn test_create_buffer() {
        let rb = RingBuffer::new(3);
        assert_eq!(rb.data.capacity(), 3);
        assert!(!rb.is_full);
    }

    #[test]
    fn test_simple_write() {
        let mut rb: RingBuffer = RingBuffer::new(3);

        let b = rb.write("ab");
        assert_eq!(b, Ok(2));
    }

    #[test]
    fn test_write_full_buffer() {
        let mut rb: RingBuffer = RingBuffer::new(3);
        let b = rb.write("abc");
        assert_eq!(b, Ok(3));
        assert!(rb.is_full);
    }

    #[test]
    fn test_simple_read() {
        let mut rb: RingBuffer = RingBuffer::new(3);
        let _b = rb.write("ab");
        let d = rb.read(1);
        assert_eq!(d, Some("a".to_string()));
        assert_eq!(rb.read_idx, 1);
    }

    #[test]
    fn test_read_full_buffer() {
        let mut rb: RingBuffer = RingBuffer::new(3);
        let _b = rb.write("abc");
        let d = rb.read(1);

        assert_eq!(d, Some("a".to_string()));
        assert_eq!(rb.read_idx, 1);
        assert_eq!(rb.write_idx, 0);
        assert!(!rb.is_full);
    }

    #[test]
    fn test_rw_full() {
        let mut rb: RingBuffer = RingBuffer::new(3);

        let mut b = rb.write("ab");
        assert_eq!(b, Ok(2));
        assert_eq!(rb.write_idx, 2);

        b = rb.write("cd");
        assert_eq!(b, Ok(1));
        assert_eq!(rb.write_idx, 0);
        assert!(rb.is_full);

        let d = rb.read(1);
        assert_eq!(d, Some("a".to_string()));
        assert_eq!(rb.read_idx, 1);
        assert!(!rb.is_full);

        b = rb.write("e");
        assert_eq!(b, Ok(1));
        assert_eq!(rb.write_idx, 1);
        assert!(rb.is_full);

        let d = rb.read(2);
        assert_eq!(d, Some("bc".to_string()));
        assert_eq!(rb.read_idx, 0);
        assert!(!rb.is_full);
    }

    #[test]
    fn test_read_empty_buffer() {
        let mut rb = RingBuffer::new(3);
        let d = rb.read(1);

        assert_eq!(d, None)
    }

    #[test]
    fn test_write_to_full_buffer() {
        let mut rb = RingBuffer::new(3);
        let _ = rb.write("abc");
        let b = rb.write("d");

        assert_eq!(b, Err(WriteError::NoSpaceLeft))
    }

    #[test]
    fn test_simple_write_thread_safe() {
        let tsrb = ThreadSafeRingBuffer::new(3);

        let b = tsrb.write("ab");
        assert_eq!(b, Ok(2));
    }

    #[test]
    fn test_simple_read_thread_safe() {
        let tsrb = ThreadSafeRingBuffer::new(3);
        let _b = tsrb.write("ab");
        let d = tsrb.read(1);
        assert_eq!(d, Some("a".to_string()));
    }

    #[test]
    fn test_multithread_write_read() {
        let tsrb = ThreadSafeRingBuffer::new(100);
        let num_threads = 10;
        let items_per_thread = 10_000;

        let (counts, total_items) = scope(|s| {
            for i in 0..num_threads {
                let writer_buffer = tsrb.clone();
                s.spawn(move || {
                    for _ in 0..items_per_thread {
                        let item = i.to_string();
                        loop {
                            if writer_buffer.write(&item).is_ok() {
                                break;
                            }
                        }
                    }
                });
            }

            let mut reader_handles = Vec::new();
            for _ in 0..num_threads {
                let reader_buffer = tsrb.clone();
                let handle = s.spawn(move || {
                    let mut my_reads = Vec::with_capacity(items_per_thread);
                    for _ in 0..items_per_thread {
                        loop {
                            if let Some(value) = reader_buffer.read(1) {
                                my_reads.push(value);
                                break;
                            }
                        }
                    }
                    my_reads
                });
                reader_handles.push(handle);
            }

            let mut counts = HashMap::new();
            let mut total_items = 0;

            for handle in reader_handles {
                let thread_vec = handle.join().unwrap();
                for item in thread_vec {
                    *counts.entry(item).or_insert(0) += 1;
                    total_items += 1;
                }
            }

            (counts, total_items)
        });

        let total_written = num_threads * items_per_thread;
        assert_eq!(total_items, total_written);

        for i in 0..num_threads {
            let expected_str = i.to_string();
            let count = counts.get(&expected_str).unwrap_or(&0);
            assert_eq!(*count, items_per_thread,
                       "Ошибка подсчета для строки '{}'", expected_str);
        }
    }
}
