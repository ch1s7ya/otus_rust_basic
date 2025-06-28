pub fn get_tup_element_by_flag(t: &mut (i32, i32), b: bool) -> &mut i32 {
    if b { &mut t.1 } else { &mut t.0 }
}

pub fn get_element_at_index(s: &mut [i32], n: usize) -> &mut i32 {
    &mut s[n]
}

pub fn get_element_at_index_from_end(s: &[i32], n: usize) -> &i32 {
    &s[s.len() - n]
}

pub fn get_slices(s: &[i32], n: usize) -> (&[i32], &[i32]) {
    (&s[..n], &s[n..])
}

pub fn split_slice_by_four(s: &[i32]) -> [&[i32]; 4] {
    let base_size = s.len() / 4;
    let remainder = s.len() % 4;

    let mut result = [&s[0..0]; 4];

    let mut current_pos = 0;

    for (i, slot) in result.iter_mut().enumerate() {
        let chunk_size = base_size + if i < remainder { 1 } else { 0 };

        let new_slice = &s[current_pos..current_pos + chunk_size];

        current_pos += chunk_size;

        *slot = new_slice;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_element_by_flug() {
        let mut my_tuple = (1, 2);

        let result = get_tup_element_by_flag(&mut my_tuple, true);
        assert_eq!(*result, 2);

        let result = get_tup_element_by_flag(&mut my_tuple, false);
        assert_eq!(*result, 1);
    }

    #[test]
    fn test_get_element_at_index() {
        let mut my_slice = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = get_element_at_index(&mut my_slice, 5);
        assert_eq!(*result, 5)
    }

    #[test]
    fn test_get_element_at_index_from_end() {
        let my_slice = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = get_element_at_index_from_end(&my_slice, 2);
        assert_eq!(*result, 8)
    }

    #[test]
    fn test_get_slices() {
        let my_slice = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let (slice1, slice2) = get_slices(&my_slice, 4);
        assert_eq!(slice1, [0, 1, 2, 3]);
        assert_eq!(slice2, [4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_split_slice() {
        let my_slice = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result: [&[i32]; 4] = split_slice_by_four(&my_slice);
        let expected_result: [&[i32]; 4] = [&[0, 1, 2], &[3, 4, 5], &[6, 7], &[8, 9]];
        assert_eq!(result, expected_result);
    }
}
