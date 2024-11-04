pub(crate) const fn usize_abs_diff(a: usize, b: usize) -> usize {
    if a >= b {
        return a - b;
    }

    b - a
}

pub(crate) const fn usize_min(a: usize, b: usize) -> usize {
    if a <= b {
        a
    } else {
        b
    }
}
