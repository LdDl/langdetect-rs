use std::fmt;

/// Represents a detected language with its probability score.
///
/// This struct is returned by detection methods to provide both the
/// language identifier and the confidence score for that detection.
#[derive(Debug, Clone, PartialEq)]
pub struct Language {
    /// The language identifier (ISO 639-1 code) or None for unknown.
    pub lang: Option<String>,
    /// The probability score between 0.0 and 1.0.
    pub prob: f64,
}

impl Language {
    /// Creates a new Language instance.
    ///
    /// # Arguments
    /// * `lang` - Optional language code (e.g., "ru", "en").
    /// * `prob` - Probability score between 0.0 and 1.0.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use langdetect_rs::language::Language;
    ///
    /// let english = Language::new(Some("en".to_string()), 0.95);
    /// let unknown = Language::new(None, 0.0);
    /// ```
    pub fn new(lang: Option<String>, prob: f64) -> Self {
        Language { lang, prob }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.lang {
            Some(lang) => write!(f, "{}:{:.1}", lang, self.prob),
            None => write!(f, ""),
        }
    }
}

impl PartialOrd for Language {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.prob.partial_cmp(&other.prob)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language() {
        let lang = Language::new(None, 0.0);
        assert_eq!(lang.lang, None);
        assert!((lang.prob - 0.0).abs() < 0.0001);
        assert_eq!(lang.to_string(), "");

        let lang2 = Language::new(Some("en".to_string()), 1.0);
        assert_eq!(lang2.lang.as_deref(), Some("en"));
        assert!((lang2.prob - 1.0).abs() < 0.0001);
        assert_eq!(lang2.to_string(), "en:1.0");
    }

    #[test]
    fn test_cmp() {
        let lang1 = Language::new(Some("a".to_string()), 0.1);
        let lang2 = Language::new(Some("b".to_string()), 0.5);

        assert!(lang1 < lang2);
        assert!(lang1 != lang2);
        assert!(!(lang1 > lang1));
    }
}