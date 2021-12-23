pub(crate) fn chars_of(s: &str) -> Vec<char> {
    Iterator::collect(s.chars())
}
