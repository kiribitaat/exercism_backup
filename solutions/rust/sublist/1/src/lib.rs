use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let first_list_len = first_list.len();
    let second_list_len = second_list.len();

    match first_list_len.cmp(&second_list_len) {
        Ordering::Equal => {
            if first_list == second_list {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Less => {
            if contains_slice(second_list, first_list) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Greater => {
            if contains_slice(first_list, second_list) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
    }
}

fn contains_slice<T: PartialEq>(haystack: &[T], needle: &[T]) -> bool {
    needle.is_empty()
        || haystack
            .windows(needle.len())
            .any(|window| window == needle)
}
