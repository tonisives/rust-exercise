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
            }
        }

        if is_sub_list {
            Comparison::Sublist
        } else {
            Comparison::Unequal
        }
    }
}

// a: sublist
fn detect_sublist(a: &[i32], b: &[i32], ctr: &mut usize, is_sub_list: &mut bool, i: usize) {
    if a.len() > 0 && *ctr < a.len() - 1 {
        if !*is_sub_list && b[i] == a[*ctr] {
            *is_sub_list = true;
            *ctr += 1;
        } else if *is_sub_list {
            let is_equal = b[i] == a[*ctr];
            *ctr += 1;

            if a.len() - 1 > i && b.len() - 1 > i {
                println!("[{}, {}],  {} isEqual {}", a[i], b[i], ctr, is_equal);
            }
            if !is_equal {
                *is_sub_list = false;
            }
        }
    }
}
