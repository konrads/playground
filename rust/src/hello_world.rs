fn hello_world() -> &'static str {
    "hello world"
}

#[cfg(test)]
mod tests {
    use super::hello_world;

    #[test]
    fn hello_world_test() {
        assert_eq!(hello_world(), "hello world");
    }
}