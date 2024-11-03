pub(crate) fn usize_abs_diff(a: usize, b: usize) -> usize {
    if a >= b {
        return a - b;
    }

    b - a
}
