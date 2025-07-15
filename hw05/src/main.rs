#[allow(dead_code)]
struct RingBuffer {
    read_idx: usize,
    write_idx: usize,
    is_full: bool,
    data: Vec<u8>,
}

#[allow(dead_code)]
fn create(size: usize) -> RingBuffer {
    RingBuffer {
        read_idx: 0,
        write_idx: 0,
        is_full: false,
        data: vec![0; size],
    }
}

#[allow(dead_code)]
fn write(rb: &mut RingBuffer, buf: &[u8]) -> usize {
    if rb.is_full {
        return 0;
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

    bytes_to_write
}

#[allow(dead_code)]
fn read(rb: &mut RingBuffer, b: usize) -> Vec<u8> {
    let mut result = vec![0; b];
    let capacity = rb.data.len();

    for element in result.iter_mut().take(b) {
        *element = rb.data[rb.read_idx];
        rb.read_idx = (rb.read_idx + 1) % capacity;
    }

    rb.is_full = false;

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // обязательно добавьте тесты
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
        assert_eq!(b, 2);
    }

    #[test]
    fn test_write_full_buffer() {
        let mut rb: RingBuffer = create(3);
        let b = write(&mut rb, &[b'a', b'b', b'c']);
        assert_eq!(b, 3);
        assert!(rb.is_full);
    }

    #[test]
    fn test_simple_read() {
        let mut rb: RingBuffer = create(3);
        let b = write(&mut rb, &[b'a', b'b']);
        let d = read(&mut rb, 1);
        assert_eq!(d, vec![b'a']);
        assert_eq!(rb.read_idx, 1);
    }

    #[test]
    fn test_read_full_buffer() {
        let mut rb: RingBuffer = create(3);
        let b = write(&mut rb, &[b'a', b'b', b'c']);
        let d = read(&mut rb, 1);

        assert_eq!(d, vec![b'a']);
        assert_eq!(rb.read_idx, 1);
        assert_eq!(rb.write_idx, 0);
        assert!(!rb.is_full);
    }

    #[test]
    fn test_rw_full() {
        let mut rb: RingBuffer = create(3);

        let mut b = write(&mut rb, &[b'a', b'b']);
        assert_eq!(b, 2);
        assert_eq!(rb.write_idx, 2);
        println!("{:?}", rb.data);

        b = write(&mut rb, &[b'c', b'd']);
        assert_eq!(b, 1);
        assert_eq!(rb.write_idx, 0);
        assert!(rb.is_full);
        println!("{:?}", rb.data);

        let d = read(&mut rb, 1);
        assert_eq!(d, vec![b'a']);
        assert_eq!(rb.read_idx, 1);
        assert!(!rb.is_full);
        println!("{:?}", rb.data);

        b = write(&mut rb, &[b'e']);
        assert_eq!(b, 1);
        assert_eq!(rb.write_idx, 1);
        assert!(rb.is_full);
        println!("{:?}", rb.data);

        let d = read(&mut rb, 2);
        assert_eq!(d, vec![b'b', b'c']);
        assert_eq!(rb.read_idx, 0);
        assert!(!rb.is_full);
        println!("{:?}", rb.data);
    }
}

fn main() {
    print!("Hello, Bender!")
}
