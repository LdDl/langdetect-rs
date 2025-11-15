use std::collections::HashMap;
use crate::utils::ngram::NGram;
use crate::detector_factory::LangProfileJson;

/// Language profile which stores name, frequency map and counts of n-grams lengths.
pub struct LangProfile {
    pub name: Option<String>,
    pub freq: HashMap<String, usize>,
    pub n_words: [usize; NGram::N_GRAM],
}

impl LangProfile {
    pub const MINIMUM_FREQ: usize = 2;
    pub const LESS_FREQ_RATIO: usize = 100_000;

    /// Use `.build()` after preparing all needed options to obtain the `LangProfile`.
    pub fn new() -> LangProfileBuilder {
        LangProfileBuilder {
            profile: LangProfile {
                name: None,
                freq: HashMap::new(),
                n_words: [0usize; NGram::N_GRAM],
            },
        }
    }

    pub fn from_json(json: LangProfileJson) -> Result<Self, &'static str> {
        let mut arr: [usize; NGram::N_GRAM] = [0usize; NGram::N_GRAM];
        if json.n_words.len() != NGram::N_GRAM {
            return Err("Invalid n_words length");
        }
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

/// Builder for `LangProfile` with fluent setters.
pub struct LangProfileBuilder {
    profile: LangProfile,
}

impl LangProfileBuilder {
    /// Set profile name.
    pub fn with_name(mut self, name: &str) -> Self {
        self.profile.name = Some(name.to_string());
        self
    }

    /// Set the frequency map directly.
    pub fn with_freq(mut self, freq: HashMap<String, usize>) -> Self {
        self.profile.freq = freq;
        self
    }

    /// Set the n_words counts array directly.
    pub fn with_n_words(mut self, n_words: [usize; NGram::N_GRAM]) -> Self {
        self.profile.n_words = n_words;
        self
    }

    pub fn build(self) -> LangProfile {
        self.profile
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lang_profile() {
        let profile = LangProfile::new().build();
        assert!(profile.name.is_none());
    }

    #[test]
    fn test_lang_profile_string_int() {
        let profile = LangProfile::new().with_name("en").build();
        assert_eq!(profile.name.as_deref(), Some("en"));
    }

    #[test]
    fn test_add() {
        let mut profile = LangProfile::new().with_name("en").build();
        profile.add("a");
        assert_eq!(profile.freq.get("a"), Some(&1));
        profile.add("a");
        assert_eq!(profile.freq.get("a"), Some(&2));
        profile.omit_less_freq();
    }

    #[test]
    fn test_add_illegally1() {
        let mut profile = LangProfile::new().build();
        profile.add("a"); // ignore
        assert_eq!(profile.freq.get("a"), None); // ignored
    }

    #[test]
    fn test_add_illegally2() {
        let mut profile = LangProfile::new().with_name("en").build();
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
        let mut profile = LangProfile::new().with_name("en").build();
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
        let mut profile = LangProfile::new().build();
        // ignore
        profile.omit_less_freq();
    }
}