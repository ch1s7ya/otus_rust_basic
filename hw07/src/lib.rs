#[derive(Debug,PartialEq)]
pub enum WriteError {
    NoSpaceLeft,
}

pub struct RingBuffer {
    read_idx: usize,
    write_idx: usize,
    is_full: bool,
    data: Vec<u8>,
}

pub fn create(size: usize) -> RingBuffer {
    RingBuffer {
        read_idx: 0,
        write_idx: 0,
        is_full: false,
        data: vec![0; size],
    }
}

pub fn write(rb: &mut RingBuffer, buf: &[u8]) -> Result<usize, WriteError> {
    if rb.is_full {
        return Err(WriteError::NoSpaceLeft)
    }

    let capacity = rb.data.len();

    let len: usize = if !rb.is_full {
        (rb.write_idx + capacity - rb.read_idx) % capacity
    } else {
        capacity
    };

    let free_space = rb.data.len() - len;
    let bytes_to_write = std::cmp::min(buf.len(), free_space);

    for element in buf.iter().take(bytes_to_write) {
        rb.data[rb.write_idx] = *element;
        rb.write_idx = (rb.write_idx + 1) % capacity;
    }

    if rb.write_idx == rb.read_idx {
        rb.is_full = true
    }

    Ok(bytes_to_write)
}

pub fn read(rb: &mut RingBuffer, b: usize) -> Option<Vec<u8>> {
    if rb.read_idx == rb.write_idx && !rb.is_full {
        return None
    }
    let mut result = vec![0; b];
    let capacity = rb.data.len();

    for element in result.iter_mut().take(b) {
        *element = rb.data[rb.read_idx];
        rb.read_idx = (rb.read_idx + 1) % capacity;
    }

    rb.is_full = false;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_buffer() {
        let rb = create(3);
        assert_eq!(rb.data.capacity(), 3);
        assert!(!rb.is_full);
    }

    #[test]
    fn test_simple_write() {
        let mut rb: RingBuffer = create(3);

        let b = write(&mut rb, &[b'a', b'b']);
        assert_eq!(b, Ok(2));
    }

    #[test]
    fn test_write_full_buffer() {
        let mut rb: RingBuffer = create(3);
        let b = write(&mut rb, &[b'a', b'b', b'c']);
        assert_eq!(b, Ok(3));
        assert!(rb.is_full);
    }

    #[test]
    fn test_simple_read() {
        let mut rb: RingBuffer = create(3);
        let _b = write(&mut rb, &[b'a', b'b']);
        let d = read(&mut rb, 1);
        assert_eq!(d, Some(vec![b'a']));
        assert_eq!(rb.read_idx, 1);
    }

    #[test]
    fn test_read_full_buffer() {
        let mut rb: RingBuffer = create(3);
        let _b = write(&mut rb, &[b'a', b'b', b'c']);
        let d = read(&mut rb, 1);

        assert_eq!(d, Some(vec![b'a']));
        assert_eq!(rb.read_idx, 1);
        assert_eq!(rb.write_idx, 0);
        assert!(!rb.is_full);
    }

    #[test]
    fn test_rw_full() {
        let mut rb: RingBuffer = create(3);

        let mut b = write(&mut rb, &[b'a', b'b']);
        assert_eq!(b, Ok(2));
        assert_eq!(rb.write_idx, 2);

        b = write(&mut rb, &[b'c', b'd']);
        assert_eq!(b, Ok(1));
        assert_eq!(rb.write_idx, 0);
        assert!(rb.is_full);

        let d = read(&mut rb, 1);
        assert_eq!(d, Some(vec![b'a']));
        assert_eq!(rb.read_idx, 1);
        assert!(!rb.is_full);

        b = write(&mut rb, &[b'e']);
        assert_eq!(b, Ok(1));
        assert_eq!(rb.write_idx, 1);
        assert!(rb.is_full);

        let d = read(&mut rb, 2);
        assert_eq!(d, Some(vec![b'b', b'c']));
        assert_eq!(rb.read_idx, 0);
        assert!(!rb.is_full);
    }

    #[test]
    fn test_read_empty_buffer() {
        let mut rb = create(3);
        let d = read(&mut rb, 1);
        
        assert_eq!(d, None)
    }

    #[test]
    fn test_write_to_full_buffer() {
        let mut rb = create(3);
        let mut b = write(&mut rb, &[b'c', b'd', b'c']);
        b = write(&mut rb, &[b'd']);
        
        assert_eq!(b, Err(WriteError::NoSpaceLeft))
    }
}