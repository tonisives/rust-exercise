// todo!(
//     "Using the binary search algorithm, find the element '{key}' in the array '{array:?}' and return its index."
// );
pub fn find(array: &[i32], key: i32) -> Option<usize> {
    // divide and conquer. binary search: a sorted list is split in the middle, and then either
    // goes toward left or right again in the middle and comparing the new number until it finds
    // the target
    find_offset(array, key, 0)
}

fn find_offset(array: &[i32], key: i32, offset: usize) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let mid = array.len() / 2;
    let mid_val = array[mid];

    return if mid_val == key {
        Some(mid + offset as usize)
    } else if key > mid_val {
        let right = &array[mid + 1..];
        find_offset(right, key, offset + mid + 1)
    } else {
        let left = &array[0..mid];
        find_offset(left, key, offset)
    };
}
