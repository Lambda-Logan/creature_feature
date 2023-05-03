/// Besides internal usage, the Criterion benchmark uses it:
pub fn chars_of(s: &str) -> Vec<char> {
    Iterator::collect(s.chars())
}
