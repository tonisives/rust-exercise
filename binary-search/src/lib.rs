// todo!(
//     "Using the binary search algorithm, find the element '{key}' in the array '{array:?}' and return its index."
// );
pub fn find(array: &[i32], key: i32) -> Option<usize> {
    // divide and conquer. binary search: a sorted list is split in the middle, and then either
    // goes toward left or right again in the middle and comparing the new number until it finds
    // the target

    if array.len() == 0 {
        return None;
    }

    let mid = array.len() / 2;
    let mid_val = array[mid];

    if mid_val == key {
        return Some(mid);
    } else if key > mid_val {
        let right = &array[mid + 1..];
        find(right, (right.len() / 2) as i32);
    } else {
        let left = &array[0..mid];
        find(left, (left.len() / 2) as i32);
    }

    None
}
