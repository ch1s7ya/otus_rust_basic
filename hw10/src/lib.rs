use std::iter::from_fn;

pub fn merge_sort(input: &[u64]) -> Vec<u64> {
    if input.len() <= 1 {
        return input.to_vec();
    }

    let midpoint = input.len() / 2;

    let left_half = &input[..midpoint];
    let right_half = &input[midpoint..];

    let left_half = merge_sort(left_half);
    let right_half = merge_sort(right_half);

    merge(&left_half, &right_half)
}

fn merge(left: &[u64], right: &[u64]) -> Vec<u64> {
    let mut left_iter = left.iter().peekable();
    let mut right_iter = right.iter().peekable();

    let merged_iterator = from_fn(move || match (left_iter.peek(), right_iter.peek()) {
        (Some(&left_var), Some(&right_var)) => {
            if left_var <= right_var {
                left_iter.next()
            } else {
                right_iter.next()
            }
        }
        (Some(_), None) => left_iter.next(),
        (None, Some(_)) => right_iter.next(),
        (None, None) => None,
    });

    merged_iterator.cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_slice() {
        assert_eq!(merge_sort(&[]), vec![]);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(merge_sort(&[1]), vec![1]);
    }

    #[test]
    fn test_sorted_slice() {
        assert_eq!(merge_sort(&[1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted_slice() {
        assert_eq!(merge_sort(&[5, 4, 3, 2, 1]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_unsorted_slice() {
        let input = [8, 3, 5, 1, 9, 4, 7, 2, 6];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(merge_sort(&input), expected);
    }

    #[test]
    fn test_with_duplicates() {
        let input = [5, 2, 8, 2, 9, 5, 8];
        let expected = vec![2, 2, 5, 5, 8, 8, 9];
        assert_eq!(merge_sort(&input), expected);
    }

    #[test]
    fn test_uneven_split() {
        let input = [3, 1, 2];
        let expected = vec![1, 2, 3];
        assert_eq!(merge_sort(&input), expected);
    }
}
