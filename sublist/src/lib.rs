#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    // first option: one by one detect equal, sublist, superlist. if not any, then return unequal

    let mut is_equal = true;

    let mut is_super_list = false;
    let mut super_list_ctr = 0;

    let mut is_sub_list = false;
    let mut sub_list_ctr: usize = 0;

    // short-circuit equal. does not overlap with other variants
    if first_list.len() == second_list.len() {
        for i in 0..first_list.len() {
            if is_equal && first_list[i] != second_list[i] {
                is_equal = false
            }
        }
        if is_equal {
            return Comparison::Equal;
        } else {
            return Comparison::Unequal;
        }
    }

    // first is superlist
    if first_list.len() >= second_list.len() {
        for i in 0..first_list.len() {
            detect_sublist(
                second_list,
                first_list,
                &mut super_list_ctr,
                &mut is_super_list,
                i,
            );

            if is_super_list {
                break;
            }
        }
    }

    if is_super_list {
        Comparison::Superlist
    } else {
        if second_list.len() >= first_list.len() {
            for i in 0..second_list.len() {
                // a is sub list
                detect_sublist(
                    first_list,
                    second_list,
                    &mut sub_list_ctr,
                    &mut is_sub_list,
                    i,
                );

                if is_sub_list {
                    break;
                }
            }
        }

        if is_sub_list {
            Comparison::Sublist
        } else {
            Comparison::Unequal
        }
    }
}

// detect a as sublist in b
fn detect_sublist(a: &[i32], b: &[i32], ctr: &mut usize, is_sub_list: &mut bool, i: usize) {
    if a.is_empty() {
        *is_sub_list = true;
        return;
    }

    if *is_sub_list || i + 1 < a.len() {
        return;
    }

    let start = i + 1 - a.len();
    *ctr = 0;

    for offset in 0..a.len() {
        if b[start + offset] != a[offset] {
            return;
        }

        *ctr += 1;
    }

    *is_sub_list = true;
}
