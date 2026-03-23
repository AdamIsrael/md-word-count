/* A word count library for Markdown content */

use std::borrow::Cow;
use std::sync::LazyLock;

use regex::Regex;

static COMMENT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)%%\s+.*?\s+%%|<!--.+?-->").unwrap());

static SYMBOLS_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\p{S}\p{P}]").unwrap());

fn strip_comments(content: &str) -> Cow<'_, str> {
    COMMENT_RE.replace_all(content, "")
}

fn strip_symbols(content: &str) -> Cow<'_, str> {
    SYMBOLS_RE.replace_all(content, "")
}

pub fn count_words(text: &str) -> usize {
    strip_symbols(strip_comments(text).as_ref())
        .split_whitespace()
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text() {
        assert_eq!(count_words("hello world"), 2);
    }

    #[test]
    fn with_symbols() {
        assert_eq!(count_words("> hello — world"), 2);
    }

    #[test]
    fn with_comment() {
        assert_eq!(count_words("hello %% secret %% world"), 2);
        assert_eq!(count_words("hello %%\n\n secret %%\n\n world"), 2);
    }

    #[test]
    fn empty_input() {
        assert_eq!(count_words(""), 0);
    }

    #[test]
    fn multiline_comment() {
        assert_eq!(count_words("before %% multi\nline %% after"), 2);
    }
}
