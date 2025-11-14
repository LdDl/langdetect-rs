use crate::utils::unicode_block::*;
use crate::utils::messages;

use std::collections::HashMap;

pub struct NGram {
    pub grams: String,
    pub capitalword: bool,
}

lazy_static::lazy_static! {
    static ref LATIN1_EXCLUDED: String = messages::get_string("NGram.LATIN1_EXCLUDE");
    static ref CJK_MAP: HashMap<char, char> = {
        let mut map = HashMap::new();
        let cjk_classes = vec![
            messages::get_string("NGram.KANJI_1_0"),
            messages::get_string("NGram.KANJI_1_2"),
            messages::get_string("NGram.KANJI_1_4"),
            messages::get_string("NGram.KANJI_1_8"),
            messages::get_string("NGram.KANJI_1_11"),
            messages::get_string("NGram.KANJI_1_12"),
            messages::get_string("NGram.KANJI_1_13"),
            messages::get_string("NGram.KANJI_1_14"),
            messages::get_string("NGram.KANJI_1_16"),
            messages::get_string("NGram.KANJI_1_18"),
            messages::get_string("NGram.KANJI_1_22"),
            messages::get_string("NGram.KANJI_1_27"),
            messages::get_string("NGram.KANJI_1_29"),
            messages::get_string("NGram.KANJI_1_31"),
            messages::get_string("NGram.KANJI_1_35"),
            messages::get_string("NGram.KANJI_2_0"),
            messages::get_string("NGram.KANJI_2_1"),
            messages::get_string("NGram.KANJI_2_4"),
            messages::get_string("NGram.KANJI_2_9"),
            messages::get_string("NGram.KANJI_2_10"),
            messages::get_string("NGram.KANJI_2_11"),
            messages::get_string("NGram.KANJI_2_12"),
            messages::get_string("NGram.KANJI_2_13"),
            messages::get_string("NGram.KANJI_2_15"),
            messages::get_string("NGram.KANJI_2_16"),
            messages::get_string("NGram.KANJI_2_18"),
            messages::get_string("NGram.KANJI_2_21"),
            messages::get_string("NGram.KANJI_2_22"),
            messages::get_string("NGram.KANJI_2_23"),
            messages::get_string("NGram.KANJI_2_28"),
            messages::get_string("NGram.KANJI_2_29"),
            messages::get_string("NGram.KANJI_2_30"),
            messages::get_string("NGram.KANJI_2_31"),
            messages::get_string("NGram.KANJI_2_32"),
            messages::get_string("NGram.KANJI_2_35"),
            messages::get_string("NGram.KANJI_2_36"),
            messages::get_string("NGram.KANJI_2_37"),
            messages::get_string("NGram.KANJI_2_38"),
            messages::get_string("NGram.KANJI_3_1"),
            messages::get_string("NGram.KANJI_3_2"),
            messages::get_string("NGram.KANJI_3_3"),
            messages::get_string("NGram.KANJI_3_4"),
            messages::get_string("NGram.KANJI_3_5"),
            messages::get_string("NGram.KANJI_3_8"),
            messages::get_string("NGram.KANJI_3_9"),
            messages::get_string("NGram.KANJI_3_11"),
            messages::get_string("NGram.KANJI_3_12"),
            messages::get_string("NGram.KANJI_3_13"),
            messages::get_string("NGram.KANJI_3_15"),
            messages::get_string("NGram.KANJI_3_16"),
            messages::get_string("NGram.KANJI_3_18"),
            messages::get_string("NGram.KANJI_3_19"),
            messages::get_string("NGram.KANJI_3_22"),
            messages::get_string("NGram.KANJI_3_23"),
            messages::get_string("NGram.KANJI_3_27"),
            messages::get_string("NGram.KANJI_3_29"),
            messages::get_string("NGram.KANJI_3_30"),
            messages::get_string("NGram.KANJI_3_31"),
            messages::get_string("NGram.KANJI_3_32"),
            messages::get_string("NGram.KANJI_3_35"),
            messages::get_string("NGram.KANJI_3_36"),
            messages::get_string("NGram.KANJI_3_37"),
            messages::get_string("NGram.KANJI_3_38"),
            messages::get_string("NGram.KANJI_4_0"),
            messages::get_string("NGram.KANJI_4_9"),
            messages::get_string("NGram.KANJI_4_10"),
            messages::get_string("NGram.KANJI_4_16"),
            messages::get_string("NGram.KANJI_4_17"),
            messages::get_string("NGram.KANJI_4_18"),
            messages::get_string("NGram.KANJI_4_22"),
            messages::get_string("NGram.KANJI_4_24"),
            messages::get_string("NGram.KANJI_4_28"),
            messages::get_string("NGram.KANJI_4_34"),
            messages::get_string("NGram.KANJI_4_39"),
            messages::get_string("NGram.KANJI_5_10"),
            messages::get_string("NGram.KANJI_5_11"),
            messages::get_string("NGram.KANJI_5_12"),
            messages::get_string("NGram.KANJI_5_13"),
            messages::get_string("NGram.KANJI_5_14"),
            messages::get_string("NGram.KANJI_5_18"),
            messages::get_string("NGram.KANJI_5_26"),
            messages::get_string("NGram.KANJI_5_29"),
            messages::get_string("NGram.KANJI_5_34"),
            messages::get_string("NGram.KANJI_5_39"),
            messages::get_string("NGram.KANJI_6_0"),
            messages::get_string("NGram.KANJI_6_3"),
            messages::get_string("NGram.KANJI_6_9"),
            messages::get_string("NGram.KANJI_6_10"),
            messages::get_string("NGram.KANJI_6_11"),
            messages::get_string("NGram.KANJI_6_12"),
            messages::get_string("NGram.KANJI_6_16"),
            messages::get_string("NGram.KANJI_6_18"),
            messages::get_string("NGram.KANJI_6_20"),
            messages::get_string("NGram.KANJI_6_21"),
            messages::get_string("NGram.KANJI_6_22"),
            messages::get_string("NGram.KANJI_6_23"),
            messages::get_string("NGram.KANJI_6_25"),
            messages::get_string("NGram.KANJI_6_28"),
            messages::get_string("NGram.KANJI_6_29"),
            messages::get_string("NGram.KANJI_6_30"),
            messages::get_string("NGram.KANJI_6_32"),
            messages::get_string("NGram.KANJI_6_34"),
            messages::get_string("NGram.KANJI_6_35"),
            messages::get_string("NGram.KANJI_6_37"),
            messages::get_string("NGram.KANJI_6_39"),
            messages::get_string("NGram.KANJI_7_0"),
            messages::get_string("NGram.KANJI_7_3"),
            messages::get_string("NGram.KANJI_7_6"),
            messages::get_string("NGram.KANJI_7_7"),
            messages::get_string("NGram.KANJI_7_9"),
            messages::get_string("NGram.KANJI_7_11"),
            messages::get_string("NGram.KANJI_7_12"),
            messages::get_string("NGram.KANJI_7_13"),
            messages::get_string("NGram.KANJI_7_16"),
            messages::get_string("NGram.KANJI_7_18"),
            messages::get_string("NGram.KANJI_7_19"),
            messages::get_string("NGram.KANJI_7_20"),
            messages::get_string("NGram.KANJI_7_21"),
            messages::get_string("NGram.KANJI_7_23"),
            messages::get_string("NGram.KANJI_7_25"),
            messages::get_string("NGram.KANJI_7_28"),
            messages::get_string("NGram.KANJI_7_29"),
            messages::get_string("NGram.KANJI_7_32"),
            messages::get_string("NGram.KANJI_7_33"),
            messages::get_string("NGram.KANJI_7_35"),
            messages::get_string("NGram.KANJI_7_37"),
        ];
        for cjk_list in cjk_classes {
            let mut chars = cjk_list.chars();
            if let Some(rep) = chars.next() {
                map.insert(rep, rep);
                for ch in chars {
                    map.insert(ch, rep);
                }
            }
        }
        map
    };
}

impl NGram {
    /// Vietnamese normalization: converts combining diacritics to precomposed characters
    pub fn normalize_vi(input: &str) -> String {
        // Load normalization tables from messages.properties
        let bases = messages::get_string("TO_NORMALIZE_VI_CHARS");
        let dmarks = messages::get_string("DMARK_CLASS");
        let norm_0300 = messages::get_string("NORMALIZED_VI_CHARS_0300");
        let norm_0301 = messages::get_string("NORMALIZED_VI_CHARS_0301");
        let norm_0303 = messages::get_string("NORMALIZED_VI_CHARS_0303");
        let norm_0309 = messages::get_string("NORMALIZED_VI_CHARS_0309");
        let norm_0323 = messages::get_string("NORMALIZED_VI_CHARS_0323");
        let mut result = String::new();
        let mut chars = input.chars().peekable();
        while let Some(c) = chars.next() {
            if let Some(&next) = chars.peek() {
                // Check if c is a base and next is a diacritic
                let base_idx = bases.chars().position(|b| b == c);
                let dmark_idx = dmarks.chars().position(|d| d == next);
                if let (Some(bi), Some(di)) = (base_idx, dmark_idx) {
                    let composed = match di {
                        0 => norm_0300.chars().nth(bi),
                        1 => norm_0301.chars().nth(bi),
                        2 => norm_0303.chars().nth(bi),
                        3 => norm_0309.chars().nth(bi),
                        4 => norm_0323.chars().nth(bi),
                        _ => None,
                    };
                    if let Some(pre) = composed {
                        result.push(pre);
                        // consume combining
                        chars.next();
                        continue;
                    }
                }
            }
            result.push(c);
        }
        result
    }
    pub const N_GRAM: usize = 3;

    pub fn new() -> Self {
        NGram {
            grams: " ".to_string(),
            capitalword: false,
        }
    }

    pub fn add_char(&mut self, ch: char) {
        let ch = Self::normalize(ch);
        let last_char = self.grams.chars().last().unwrap_or(' ');
        if last_char == ' ' {
            self.grams = " ".to_string();
            self.capitalword = false;
            if ch == ' ' {
                return;
            }
        } else if self.grams.chars().count() >= Self::N_GRAM {
            self.grams = self.grams.chars().skip(1).collect();
        }
        self.grams.push(ch);

        if ch.is_uppercase() {
            if last_char.is_uppercase() {
                self.capitalword = true;
            }
        } else {
            self.capitalword = false;
        }
    }

    pub fn get(&self, n: usize) -> Option<String> {
        if self.capitalword {
            return None;
        }
        if n < 1 || n > Self::N_GRAM || self.grams.chars().count() < n {
            return None;
        }
        if n == 1 {
            let ch = self.grams.chars().last()?;
            if ch == ' ' {
                return None;
            }
            return Some(ch.to_string());
        } else {
            let chars: Vec<char> = self.grams.chars().collect();
            return Some(chars[chars.len()-n..].iter().collect());
        }
    }

    pub fn normalize(ch: char) -> char {
        let block = unicode_block(ch).unwrap_or(0);
        match block {
            UNICODE_BASIC_LATIN => {
                if ch < 'A' || ('Z' < ch && ch < 'a') || ch > 'z' {
                    ' '
                } else {
                    ch
                }
            }
            UNICODE_LATIN_1_SUPPLEMENT => {
                if LATIN1_EXCLUDED.contains(ch) {
                    ' '
                } else {
                    ch
                }
            }
            UNICODE_LATIN_EXTENDED_B => {
                match ch {
                    '\u{0219}' => '\u{015F}',
                    '\u{021B}' => '\u{0163}',
                    _ => ch,
                }
            }
            UNICODE_GENERAL_PUNCTUATION => ' ',
            UNICODE_ARABIC => {
                if ch == '\u{06CC}' {
                    '\u{064A}'
                } else {
                    ch
                }
            }
            UNICODE_LATIN_EXTENDED_ADDITIONAL => {
                if ch >= '\u{1EA0}' {
                    '\u{1EC3}'
                } else {
                    ch
                }
            }
            UNICODE_HIRAGANA => '\u{3042}',
            UNICODE_KATAKANA => '\u{30A2}',
            UNICODE_BOPOMOFO | UNICODE_BOPOMOFO_EXTENDED => '\u{3105}',
            UNICODE_CJK_UNIFIED_IDEOGRAPHS => {
                CJK_MAP.get(&ch).copied().unwrap_or(ch)
            }
            UNICODE_HANGUL_SYLLABLES => '\u{AC00}',
            _ => ch,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(NGram::N_GRAM, 3);
    }

    #[test]
    fn test_normalize_with_latin() {
        assert_eq!(NGram::normalize('\u{0000}'), ' ');
        assert_eq!(NGram::normalize('\u{0009}'), ' ');
        assert_eq!(NGram::normalize('\u{0020}'), ' ');
        assert_eq!(NGram::normalize('\u{0030}'), ' ');
        assert_eq!(NGram::normalize('\u{0040}'), ' ');
        assert_eq!(NGram::normalize('A'), 'A');
        assert_eq!(NGram::normalize('Z'), 'Z');
        assert_eq!(NGram::normalize('\u{005B}'), ' ');
        assert_eq!(NGram::normalize('\u{0060}'), ' ');
        assert_eq!(NGram::normalize('a'), 'a');
        assert_eq!(NGram::normalize('z'), 'z');
        assert_eq!(NGram::normalize('\u{007B}'), ' ');
        assert_eq!(NGram::normalize('\u{007F}'), ' ');
        assert_eq!(NGram::normalize('\u{0080}'), '\u{0080}');
        assert_eq!(NGram::normalize('\u{00A0}'), ' ');
        assert_eq!(NGram::normalize('\u{00A1}'), '\u{00A1}');
    }

    #[test]
    fn test_normalize_with_cjk_kanji() {
        assert_eq!(NGram::normalize('\u{4E00}'), '\u{4E00}');
        assert_eq!(NGram::normalize('\u{4E01}'), '\u{4E01}');
        assert_eq!(NGram::normalize('\u{4E02}'), '\u{4E02}');
        assert_eq!(NGram::normalize('\u{4E03}'), NGram::normalize('\u{4E01}'));
        assert_eq!(NGram::normalize('\u{4E04}'), '\u{4E04}');
        assert_eq!(NGram::normalize('\u{4E05}'), '\u{4E05}');
        assert_eq!(NGram::normalize('\u{4E06}'), '\u{4E06}');
        assert_eq!(NGram::normalize('\u{4E07}'), '\u{4E07}');
        assert_eq!(NGram::normalize('\u{4E08}'), '\u{4E08}');
        assert_eq!(NGram::normalize('\u{4E09}'), '\u{4E09}');
        assert_eq!(NGram::normalize('\u{4E10}'), '\u{4E10}');
        assert_eq!(NGram::normalize('\u{4E11}'), '\u{4E11}');
        assert_eq!(NGram::normalize('\u{4E12}'), '\u{4E12}');
        assert_eq!(NGram::normalize('\u{4E13}'), '\u{4E13}');
        assert_eq!(NGram::normalize('\u{4E14}'), '\u{4E14}');
        assert_eq!(NGram::normalize('\u{4E15}'), '\u{4E15}');
        assert_eq!(NGram::normalize('\u{4E1E}'), '\u{4E1E}');
        assert_eq!(NGram::normalize('\u{4E1F}'), '\u{4E1F}');
        assert_eq!(NGram::normalize('\u{4E20}'), '\u{4E20}');
        assert_eq!(NGram::normalize('\u{4E21}'), '\u{4E21}');
        assert_eq!(NGram::normalize('\u{4E22}'), '\u{4E22}');
        assert_eq!(NGram::normalize('\u{4E23}'), '\u{4E23}');
        assert_eq!(NGram::normalize('\u{4E24}'), NGram::normalize('\u{4E13}'));
        assert_eq!(NGram::normalize('\u{4E25}'), NGram::normalize('\u{4E13}'));
        assert_eq!(NGram::normalize('\u{4E30}'), '\u{4E30}');
    }

    #[test]
    fn test_normalize_for_romanian() {
        assert_eq!(NGram::normalize('\u{015F}'), '\u{015F}');
        assert_eq!(NGram::normalize('\u{0163}'), '\u{0163}');
        assert_eq!(NGram::normalize('\u{0219}'), '\u{015F}');
        assert_eq!(NGram::normalize('\u{021B}'), '\u{0163}');
    }

    #[test]
    fn test_ngram() {
        let mut ngram = NGram::new();
        assert_eq!(ngram.get(0), None);
        assert_eq!(ngram.get(1), None);
        assert_eq!(ngram.get(2), None);
        assert_eq!(ngram.get(3), None);
        assert_eq!(ngram.get(4), None);
        ngram.add_char(' ');
        assert_eq!(ngram.get(1), None);
        assert_eq!(ngram.get(2), None);
        assert_eq!(ngram.get(3), None);
        ngram.add_char('A');
        assert_eq!(ngram.get(1), Some("A".to_string()));
        assert_eq!(ngram.get(2), Some(" A".to_string()));
        assert_eq!(ngram.get(3), None);
        ngram.add_char('\u{06CC}');
        assert_eq!(ngram.get(1), Some('\u{064A}'.to_string()));
        assert_eq!(ngram.get(2), Some(format!("A{}", '\u{064A}')));
        assert_eq!(ngram.get(3), Some(format!(" {}{}", 'A', '\u{064A}')));
        ngram.add_char('\u{1EA0}');
        assert_eq!(ngram.get(1), Some('\u{1EC3}'.to_string()));
        assert_eq!(ngram.get(2), Some(format!("{}{}", '\u{064A}', '\u{1EC3}')));
        assert_eq!(ngram.get(3), Some(format!("{}{}{}", 'A', '\u{064A}', '\u{1EC3}')));
        ngram.add_char('\u{3044}');
        assert_eq!(ngram.get(1), Some('\u{3042}'.to_string()));
        assert_eq!(ngram.get(2), Some(format!("{}{}", '\u{1EC3}', '\u{3042}')));
        assert_eq!(ngram.get(3), Some(format!("{}{}{}", '\u{064A}', '\u{1EC3}', '\u{3042}')));
        ngram.add_char('\u{30A4}');
        assert_eq!(ngram.get(1), Some('\u{30A2}'.to_string()));
        assert_eq!(ngram.get(2), Some(format!("{}{}", '\u{3042}', '\u{30A2}')));
        assert_eq!(ngram.get(3), Some(format!("{}{}{}", '\u{1EC3}', '\u{3042}', '\u{30A2}')));
        ngram.add_char('\u{3106}');
        assert_eq!(ngram.get(1), Some('\u{3105}'.to_string()));
        assert_eq!(ngram.get(2), Some(format!("{}{}", '\u{30A2}', '\u{3105}')));
        assert_eq!(ngram.get(3), Some(format!("{}{}{}", '\u{3042}', '\u{30A2}', '\u{3105}')));
        ngram.add_char('\u{AC01}');
        assert_eq!(ngram.get(1), Some('\u{AC00}'.to_string()));
        assert_eq!(ngram.get(2), Some(format!("{}{}", '\u{3105}', '\u{AC00}')));
        assert_eq!(ngram.get(3), Some(format!("{}{}{}", '\u{30A2}', '\u{3105}', '\u{AC00}')));
        ngram.add_char('\u{2010}');
        assert_eq!(ngram.get(1), None);
        assert_eq!(ngram.get(2), Some(format!("{} ", '\u{AC00}')));
        assert_eq!(ngram.get(3), Some(format!("{}{} ", '\u{3105}', '\u{AC00}')));
        ngram.add_char('a');
        assert_eq!(ngram.get(1), Some("a".to_string()));
        assert_eq!(ngram.get(2), Some(" a".to_string()));
        assert_eq!(ngram.get(3), None);
    }

    #[test]
    fn test_ngram3() {
        let mut ngram = NGram::new();
        ngram.add_char('A');
        assert_eq!(ngram.get(1), Some("A".to_string()));
        assert_eq!(ngram.get(2), Some(" A".to_string()));
        assert_eq!(ngram.get(3), None);
        ngram.add_char('1');
        assert_eq!(ngram.get(1), None);
        assert_eq!(ngram.get(2), Some("A ".to_string()));
        assert_eq!(ngram.get(3), Some(" A ".to_string()));
        ngram.add_char('B');
        assert_eq!(ngram.get(1), Some("B".to_string()));
        assert_eq!(ngram.get(2), Some(" B".to_string()));
        assert_eq!(ngram.get(3), None);
    }

    #[test]
    fn test_normalize_vietnamese() {
        assert_eq!(NGram::normalize_vi(""), "");
        assert_eq!(NGram::normalize_vi("ABC"), "ABC");
        assert_eq!(NGram::normalize_vi("012"), "012");
        assert_eq!(NGram::normalize_vi("\u{00C0}"), "\u{00C0}");

        // All combinations
        let bases = messages::get_string("TO_NORMALIZE_VI_CHARS");
        let dmarks = messages::get_string("DMARK_CLASS");
        let norm_0300 = messages::get_string("NORMALIZED_VI_CHARS_0300");
        let norm_0301 = messages::get_string("NORMALIZED_VI_CHARS_0301");
        let norm_0303 = messages::get_string("NORMALIZED_VI_CHARS_0303");
        let norm_0309 = messages::get_string("NORMALIZED_VI_CHARS_0309");
        let norm_0323 = messages::get_string("NORMALIZED_VI_CHARS_0323");
        for (di, norm_table) in [norm_0300, norm_0301, norm_0303, norm_0309, norm_0323].iter().enumerate() {
            for (bi, base) in bases.chars().enumerate() {
                let composed = norm_table.chars().nth(bi).unwrap();
                let dmark = dmarks.chars().nth(di).unwrap();
                let input = format!("{}{}", base, dmark);
                assert_eq!(NGram::normalize_vi(&input), composed.to_string());
            }
        }
    }
}