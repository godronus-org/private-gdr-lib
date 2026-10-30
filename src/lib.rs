//! POC org library (spike 4). Consumers depend on the public mirror at a tag:
//! `gdr-lib = { git = "https://github.com/godronus-org/gdr-lib", tag = "vX.Y.Z" }`.

/// The greeting line a consumer returns.
pub fn greeting(word: &str) -> String {
    format!("{} from gdr-lib", word.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_names_the_library() {
        assert_eq!(greeting("hello"), "hello from gdr-lib");
    }

    #[test]
    fn greeting_ignores_surrounding_whitespace() {
        assert_eq!(greeting(" hi\n"), "hi from gdr-lib");
    }
}
