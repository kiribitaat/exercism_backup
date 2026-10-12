use std::cmp::Ordering;

pub fn find<T, A>(array: A, key: T) -> Option<usize>
where
    T: Ord,
    A: AsRef<[T]>,
{
    let mut lo = 0;
    let mut hi = array.as_ref().len();

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match array.as_ref()[mid].cmp(&key) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => lo = mid + 1,
            Ordering::Greater => hi = mid,
        }
    }

    None
}
