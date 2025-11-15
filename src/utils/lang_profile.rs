use std::fs;
use std::collections::HashMap;
use std::path::Path;
use crate::utils::ngram::NGram;
use serde::{Deserialize};
use serde_json;

/// Errors that can occur when working with LangProfileJson.
#[derive(Debug, Clone)]
pub enum LangProfileJsonError {
    /// Input/Output error.
    IoError(String),
    /// JSON parsing error.
    ParseError(String),
}

/// JSON representation of a language profile loaded from disk.
#[derive(Deserialize)]
pub struct LangProfileJson {
    /// Frequency map of n-grams to their counts.
    pub freq: HashMap<String, usize>,
    /// Total counts for each n-gram length: [1-gram, 2-gram, 3-gram].
    pub n_words: Vec<usize>,
    /// Language identifier (ISO 639-1 code).
    pub name: String,
}

impl LangProfileJson {
    /// Loads a LangProfileJson from a file.
    ///     
    /// # Arguments
    /// * `file_path` - Path to the JSON file containing the language profile.
    /// 
    /// # Returns
    /// A Result containing the LangProfileJson or a LangProfileJsonError.
    /// 
    /// # Errors
    /// Returns `LangProfileJsonError` if reading or parsing fails.
    /// 
    /// # Examples
    /// ```
    /// use langdetect_rs::utils::lang_profile::LangProfileJson;
    /// use std::path::Path;
    /// let profile_json = LangProfileJson::new_from_file(Path::new("./profiles/en"));
    ///
    /// match profile_json {
    ///    Ok(json) => println!("Loaded profile for language: {}", json.name),
    ///    Err(e) => println!("Error loading profile: {:?}", e),
    /// }
    /// ```
    pub fn new_from_file<P: AsRef<Path>>(file_path: P) -> Result<LangProfileJson, LangProfileJsonError> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| LangProfileJsonError::IoError(format!("Failed to read file: {}", e)))?;
        let json_profile: LangProfileJson = serde_json::from_str(&content)
            .map_err(|e| LangProfileJsonError::ParseError(format!("Failed to parse JSON: {}", e)))?;
        Ok(json_profile)
    }
}

/// Language profile which stores name, frequency map and counts of n-grams lengths.
///
/// A language profile contains statistical information about n-gram frequencies
/// for a specific language, used for training the language detection model.
pub struct LangProfile {
    /// Optional language name/identifier.
    pub name: Option<String>,
    /// Frequency map of n-grams to their occurrence counts.
    pub freq: HashMap<String, usize>,
    /// Total counts for each n-gram length: [1-gram, 2-gram, 3-gram].
    pub n_words: [usize; NGram::N_GRAM],
}

impl LangProfile {
    /// Minimum frequency threshold for including n-grams.
    pub const MINIMUM_FREQ: usize = 2;
    /// Frequency ratio threshold for omitting less frequent n-grams.
    pub const LESS_FREQ_RATIO: usize = 100_000;

    /// Creates a new LangProfile builder.
    ///
    /// Use the builder pattern to configure the profile before calling `build()`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use langdetect_rs::utils::lang_profile::LangProfile;
    ///
    /// let profile = LangProfile::new()
    ///     .with_name("en")
    ///     .build();
    /// ```
    pub fn new() -> LangProfileBuilder {
        LangProfileBuilder {
            profile: LangProfile {
                name: None,
                freq: HashMap::new(),
                n_words: [0usize; NGram::N_GRAM],
            },
        }
    }

    /// Creates a LangProfile from JSON data.
    ///
    /// # Arguments
    /// * `json` - Parsed JSON profile data.
    ///
    /// # Errors
    /// Returns an error string if the n_words array has incorrect length.
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

    /// Adds an n-gram to the profile's frequency counts.
    ///
    /// # Arguments
    /// * `gram` - The n-gram string to add.
    ///
    /// # Notes
    /// Only n-grams of length 1-3 are accepted. Requires a profile name to be set.
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

    /// Removes n-grams that appear less frequently than the threshold.
    ///
    /// This optimization reduces profile size and improves detection speed.
    /// Also handles Roman character filtering for non-Latin languages.
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

    /// Updates the profile by analyzing text and extracting n-grams.
    ///
    /// # Arguments
    /// * `text` - The text to analyze and add to the profile.
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
///
/// Provides a convenient way to configure a LangProfile before building it.
pub struct LangProfileBuilder {
    profile: LangProfile,
}

impl LangProfileBuilder {
    /// Sets the profile name.
    ///
    /// # Arguments
    /// * `name` - The language identifier.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use langdetect_rs::utils::lang_profile::LangProfile;
    ///
    /// let builder = LangProfile::new().with_name("en");
    /// ```
    pub fn with_name(mut self, name: &str) -> Self {
        self.profile.name = Some(name.to_string());
        self
    }

    /// Sets the frequency map directly.
    ///
    /// # Arguments
    /// * `freq` - Pre-computed frequency map.
    pub fn with_freq(mut self, freq: HashMap<String, usize>) -> Self {
        self.profile.freq = freq;
        self
    }

    /// Sets the n_words counts array directly.
    ///
    /// # Arguments
    /// * `n_words` - Array of n-gram counts [1-gram, 2-gram, 3-gram].
    pub fn with_n_words(mut self, n_words: [usize; NGram::N_GRAM]) -> Self {
        self.profile.n_words = n_words;
        self
    }

    /// Builds the final LangProfile with the configured properties.
    ///
    /// # Returns
    /// The fully constructed LangProfile.
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