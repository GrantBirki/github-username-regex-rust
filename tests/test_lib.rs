#[cfg(test)]

mod tests {
    const VALID_MSG: &str = "the provided handle should be valid";
    // Import the check function from the crate root (lib.rs)
    use github_username_regex::valid;

    #[test]
    fn monalisa_is_valid() {
        assert!(valid("monalisa"), "{VALID_MSG}");
    }
}
