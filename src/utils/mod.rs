pub fn exclude<T: PartialEq + Clone>(arr: &[T], v: T) -> Vec<T> {
    arr.iter().cloned().filter(|item| *item != v).collect()
}
