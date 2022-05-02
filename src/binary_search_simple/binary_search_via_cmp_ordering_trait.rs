use std::cmp::ordering;
pub fn find_via_binary_search <T: Ord>(array: &[T], key: T) -> Option<usize> {
        let mut left_point = 0;
        let mut right_point = array.len();
        while left_point < right_point {
            let mid_point = (left_point + right_point) / 2;
            match array[mid_point].cmp(&key) {
                Ordering::Equal => return Some(mid_point),
                Ordering::Less => left_point = mid_point + 1,
                Ordering::Greater => right_point = mid_point
            }
        }
        None
    }
