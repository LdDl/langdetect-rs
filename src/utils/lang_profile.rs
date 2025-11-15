use std::collections::HashMap;
use crate::utils::ngram::NGram;

pub struct LangProfile {
    pub name: Option<String>,
    pub freq: HashMap<String, usize>,
    pub n_words: [usize; NGram::N_GRAM],
}

impl LangProfile {
    pub const MINIMUM_FREQ: usize = 2;
    pub const LESS_FREQ_RATIO: usize = 100_000;

    pub fn new() -> Self {
        LangProfile {
            name: None,
            freq: HashMap::new(),
            n_words: [0; NGram::N_GRAM],
        }
    }

    pub fn with_name(name: &str) -> Self {
        LangProfile {
            name: Some(name.to_string()),
            freq: HashMap::new(),
            n_words: [0; NGram::N_GRAM],
        }
    }

    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        use std::fs;
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct LangProfileJson {
            freq: HashMap<String, usize>,
            n_words: Vec<usize>,
            name: String,
        }
        let content = fs::read_to_string(path)?;
        let json: LangProfileJson = serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let mut arr = [0; NGram::N_GRAM];
        for (i, v) in json.n_words.iter().enumerate().take(NGram::N_GRAM) {
            arr[i] = *v;
        }
        Ok(LangProfile {
            name: Some(json.name),
            freq: json.freq,
            n_words: arr,
        })
    }

    pub fn add(&mut self, gram: &str) {
        if self.name.is_none() || gram.is_empty() {
            return;
        }
        let length = gram.chars().count();
        if length < 1 || length > NGram::N_GRAM {
            return;
        }
        self.n_words[length - 1] += 1;
        *self.freq.entry(gram.to_string()).or_insert(0) += 1;
    }

    pub fn omit_less_freq(&mut self) {
        if self.name.is_none() {
            return;
        }
        let threshold = std::cmp::max(self.n_words[0] / Self::LESS_FREQ_RATIO, Self::MINIMUM_FREQ);
        let mut roman = 0;
        let roman_char_re = regex::Regex::new(r"^[A-Za-z]$").unwrap();
        let roman_substr_re = regex::Regex::new(r".*[A-Za-z].*").unwrap();
        let mut to_remove = Vec::new();
        for (key, &count) in self.freq.iter() {
            if count <= threshold {
                self.n_words[key.chars().count() - 1] -= count;
                to_remove.push(key.clone());
            } else if roman_char_re.is_match(key) {
                roman += count;
            }
        }
        for key in to_remove.iter() {
            self.freq.remove(key);
        }
        // Roman check
        if roman < self.n_words[0] / 3 {
            let mut to_remove2 = Vec::new();
            for (key, &count) in self.freq.iter() {
                if roman_substr_re.is_match(key) {
                    self.n_words[key.chars().count() - 1] -= count;
                    to_remove2.push(key.clone());
                }
            }
            for key in to_remove2.iter() {
                self.freq.remove(key);
            }
        }
    }

    pub fn update(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }
        let text = NGram::normalize_vi(text);
        let mut gram = NGram::new();
        for ch in text.chars() {
            gram.add_char(ch);
            for n in 1..=NGram::N_GRAM {
                if let Some(g) = gram.get(n) {
                    self.add(&g);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lang_profile() {
        let profile = LangProfile::new();
        assert!(profile.name.is_none());
    }

    #[test]
    fn test_lang_profile_string_int() {
        let profile = LangProfile::with_name("en");
        assert_eq!(profile.name.as_deref(), Some("en"));
    }

    #[test]
    fn test_add() {
        let mut profile = LangProfile::with_name("en");
        profile.add("a");
        assert_eq!(profile.freq.get("a"), Some(&1));
        profile.add("a");
        assert_eq!(profile.freq.get("a"), Some(&2));
        profile.omit_less_freq();
    }

    #[test]
    fn test_add_illegally1() {
        let mut profile = LangProfile::new();
        profile.add("a"); // ignore
        assert_eq!(profile.freq.get("a"), None); // ignored
    }

    #[test]
    fn test_add_illegally2() {
        let mut profile = LangProfile::with_name("en");
        profile.add("a");
        // Illegal (string's length of parameter must be between 1 and 3) but ignore
        profile.add("");
        // as well
        profile.add("abcd");
        assert_eq!(profile.freq.get("a"), Some(&1));
        // ignored
        assert_eq!(profile.freq.get(""), None);
        // ignored
        assert_eq!(profile.freq.get("abcd"), None);
    }

    #[test]
    fn test_omit_less_freq() {
        let mut profile = LangProfile::with_name("en");
        let grams = vec![
            "a", "b", "c", "\u{3042}", "\u{3044}", "\u{3046}", "\u{3048}", "\u{304a}",
            "\u{304b}", "\u{304c}", "\u{304d}", "\u{304e}", "\u{304f}"
        ];
        for _ in 0..5 {
            for g in &grams {
                profile.add(g);
            }
        }
        profile.add("\u{3050}");
        assert_eq!(profile.freq.get("a"), Some(&5));
        assert_eq!(profile.freq.get("\u{3042}"), Some(&5));
        assert_eq!(profile.freq.get("\u{3050}"), Some(&1));
        profile.omit_less_freq();
        // omitted
        assert_eq!(profile.freq.get("a"), None);
        assert_eq!(profile.freq.get("\u{3042}"), Some(&5));
        // omitted
        assert_eq!(profile.freq.get("\u{3050}"), None);
    }

    #[test]
    fn test_omit_less_freq_illegally() {
        let mut profile = LangProfile::new();
        // ignore
        profile.omit_less_freq();
    }
}