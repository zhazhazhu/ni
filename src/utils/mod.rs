use which::which;

pub fn exclude<T: PartialEq + Clone>(arr: &[T], v: T) -> Vec<T> {
    arr.iter().cloned().filter(|item| *item != v).collect()
}

pub fn which_cmd(cmd: &str) -> bool {
    let b = which(cmd);
    match b {
        Ok(_) => true,
        Err(_) => false,
    }
}
