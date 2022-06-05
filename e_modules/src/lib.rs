mod network {
    fn connect() {}
}

mod client {
    fn connect() {}
}

// Notice there's no main.rs
// Leave this at the bottom of lib.rs
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
