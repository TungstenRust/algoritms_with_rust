pub fn find_via_binary_search(array: &[i32], key: i32) -> Option<usize> {
    if array.len() == 0 {
       None
    } else {
        let mut left_point = 0;
        let mut right_point = array.len()-1;
        while left_point < right_point {
            let mid_point = (left_point + right_point) / 2;
            if array[mid_point] == key {
                return Some(mid_point);
            } else if array[mid_point] < key {
                left_point = mid_point + 1;
            } else if array[mid_point] > key {
                right_point = mid_point;
            }
        }
        None
    }
}