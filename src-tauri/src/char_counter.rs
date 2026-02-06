use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharCountResult {
    pub char_count: usize,
    pub char_count_no_spaces: usize,
    pub word_count: usize,
    pub line_count: usize,
    pub paragraph_count: usize,
    pub byte_count_utf8: usize,
    pub byte_count_sjis: usize,
    pub alphanumeric_count: usize,
    pub hiragana_count: usize,
    pub katakana_count: usize,
    pub kanji_count: usize,
    pub fullwidth_count: usize,
    pub halfwidth_count: usize,
}

pub fn count_chars(text: &str) -> CharCountResult {
    let char_count = text.chars().count();
    let char_count_no_spaces = text.chars().filter(|c| !c.is_whitespace()).count();

    let word_count = text.split_whitespace().count();

    let line_count = if text.is_empty() {
        0
    } else {
        text.lines().count()
    };

    let paragraph_count = if text.is_empty() {
        0
    } else {
        text.split("\n\n").filter(|p| !p.trim().is_empty()).count()
    };

    let byte_count_utf8 = text.len();

    let byte_count_sjis = estimate_sjis_bytes(text);

    let mut alphanumeric_count = 0;
    let mut hiragana_count = 0;
    let mut katakana_count = 0;
    let mut kanji_count = 0;
    let mut fullwidth_count = 0;
    let mut halfwidth_count = 0;

    for c in text.chars() {
        if c.is_ascii_alphanumeric() {
            alphanumeric_count += 1;
            halfwidth_count += 1;
        } else if is_hiragana(c) {
            hiragana_count += 1;
            fullwidth_count += 1;
        } else if is_katakana(c) {
            katakana_count += 1;
            if is_halfwidth_katakana(c) {
                halfwidth_count += 1;
            } else {
                fullwidth_count += 1;
            }
        } else if is_kanji(c) {
            kanji_count += 1;
            fullwidth_count += 1;
        } else if c.is_ascii() {
            halfwidth_count += 1;
        } else {
            fullwidth_count += 1;
        }
    }

    CharCountResult {
        char_count,
        char_count_no_spaces,
        word_count,
        line_count,
        paragraph_count,
        byte_count_utf8,
        byte_count_sjis,
        alphanumeric_count,
        hiragana_count,
        katakana_count,
        kanji_count,
        fullwidth_count,
        halfwidth_count,
    }
}

fn is_hiragana(c: char) -> bool {
    ('\u{3040}'..='\u{309F}').contains(&c)
}

fn is_katakana(c: char) -> bool {
    ('\u{30A0}'..='\u{30FF}').contains(&c) || is_halfwidth_katakana(c)
}

fn is_halfwidth_katakana(c: char) -> bool {
    ('\u{FF65}'..='\u{FF9F}').contains(&c)
}

fn is_kanji(c: char) -> bool {
    ('\u{4E00}'..='\u{9FFF}').contains(&c)
        || ('\u{3400}'..='\u{4DBF}').contains(&c)
        || ('\u{F900}'..='\u{FAFF}').contains(&c)
}

fn estimate_sjis_bytes(text: &str) -> usize {
    let mut bytes = 0;
    for c in text.chars() {
        if c.is_ascii() {
            bytes += 1;
        } else if is_halfwidth_katakana(c) {
            bytes += 1;
        } else {
            bytes += 2;
        }
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_ascii() {
        let result = count_chars("Hello World");
        assert_eq!(result.char_count, 11);
        assert_eq!(result.char_count_no_spaces, 10);
        assert_eq!(result.word_count, 2);
        assert_eq!(result.line_count, 1);
        assert_eq!(result.alphanumeric_count, 10);
    }

    #[test]
    fn test_count_japanese() {
        let result = count_chars("こんにちは世界");
        assert_eq!(result.char_count, 7);
        assert_eq!(result.hiragana_count, 5);
        assert_eq!(result.kanji_count, 2);
        assert_eq!(result.byte_count_utf8, 21);
        assert_eq!(result.byte_count_sjis, 14);
    }

    #[test]
    fn test_count_mixed() {
        let result = count_chars("Hello こんにちは");
        assert_eq!(result.char_count, 11);
        assert_eq!(result.char_count_no_spaces, 10);
        assert_eq!(result.alphanumeric_count, 5);
        assert_eq!(result.hiragana_count, 5);
    }

    #[test]
    fn test_count_lines_and_paragraphs() {
        let result = count_chars("Line 1\nLine 2\n\nParagraph 2");
        assert_eq!(result.line_count, 4);
        assert_eq!(result.paragraph_count, 2);
    }

    #[test]
    fn test_empty_string() {
        let result = count_chars("");
        assert_eq!(result.char_count, 0);
        assert_eq!(result.word_count, 0);
        assert_eq!(result.line_count, 0);
        assert_eq!(result.paragraph_count, 0);
    }

    #[test]
    fn test_katakana() {
        let result = count_chars("カタカナ");
        assert_eq!(result.katakana_count, 4);
        assert_eq!(result.fullwidth_count, 4);
    }
}
