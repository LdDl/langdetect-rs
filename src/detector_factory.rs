use std::fs;
use std::path::Path;
use serde_json;
use std::collections::HashMap;
use crate::utils::lang_profile::LangProfile;
use crate::detector::{Detector, DetectorError};
use crate::language::Language;
use crate::utils::lang_profile::LangProfileJson;

/// Errors that can occur when working with DetectorFactory.
#[derive(Debug, Clone)]
pub enum DetectorFactoryError {
    /// Attempted to add a language profile that already exists.
    DuplicatedLanguage(String),
    /// At least 2 languages are required for detection.
    NotEnoughProfiles,
}

impl std::fmt::Display for DetectorFactoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DetectorFactoryError::DuplicatedLanguage(lang) => {
                write!(f, "Duplicated language profile: {}", lang)
            }
            DetectorFactoryError::NotEnoughProfiles => {
                write!(f, "Two languages at least are required")
            }
        }
    }
}

/// Factory for creating language detectors with pre-loaded language profiles.
///
/// The DetectorFactory manages a collection of language profiles and provides
/// methods to create Detector instances for language identification.
///
/// # Examples
///
/// ```rust
/// use langdetect_rs::detector_factory::DetectorFactory;
///
/// // Create factory with built-in profiles
/// let factory = DetectorFactory::default().build();
///
/// // Create a detector
/// let detector = factory.create(None);
/// ```
#[derive(Clone)]
pub struct DetectorFactory {
    /// Word-to-language probability mapping for all loaded languages.
    pub word_lang_prob_map: HashMap<String, Vec<f64>>,
    /// List of language identifiers in the same order as probability vectors.
    pub langlist: Vec<String>,
    /// Optional seed for reproducible randomization.
    pub seed: Option<u64>,
}

impl DetectorFactory {
    /// Creates a new DetectorFactory builder.
    ///
    /// Use the builder pattern to configure the factory before calling `build()`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use langdetect_rs::detector_factory::DetectorFactory;
    ///
    /// let factory = DetectorFactory::new()
    ///     .with_seed(Some(42))
    ///     .build();
    /// ```
    pub fn new() -> DetectorFactoryBuilder {
        DetectorFactoryBuilder {
            factory: DetectorFactory {
                word_lang_prob_map: HashMap::new(),
                langlist: Vec::new(),
                seed: None,
            },
        }
    }

    /// Creates a DetectorFactoryBuilder with all built-in language profiles loaded.
    ///
    /// This method loads the 55 built-in language profiles from the crate's
    /// profiles directory and returns a builder that can be further re-configured.
    /// The profiles are cached for performance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use langdetect_rs::detector_factory::DetectorFactory;
    ///
    /// let factory = DetectorFactory::default()
    ///     .with_seed(Some(42))
    ///     .build();
    /// ```
    pub fn default() -> DetectorFactoryBuilder {
        use std::sync::Mutex;
        use lazy_static::lazy_static;
        lazy_static! {
            static ref DEFAULT_FACTORY: Mutex<Option<DetectorFactory>> = Mutex::new(None);
        }
        {
            let factory_guard = DEFAULT_FACTORY.lock().unwrap();
            if let Some(factory) = &*factory_guard {
                return DetectorFactoryBuilder { factory: factory.clone() };
            }
        }
        let mut factory = DetectorFactory::new().build();
        // Try to load profiles from crate-level "profiles" folder
        let crate_profiles = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("profiles");

        println!("Loading profiles from: {:?}", crate_profiles);
        let entries = std::fs::read_dir(&crate_profiles).unwrap();
        let count = entries.count();
        println!("Found {} profile files", count);

        let _ = factory.load_profile(&crate_profiles);
        // Cache the factory for future use
        let mut factory_guard = DEFAULT_FACTORY.lock().unwrap();
        *factory_guard = Some(factory.clone());
        DetectorFactoryBuilder { factory }
    }

    /// Returns the path to the default language profiles directory.
    ///
    /// This method provides the path to the built-in language profile files that ship
    /// with the crate. End-users can use this path to load default profiles when
    /// extending or customizing the factory.
    ///
    /// Note: This path is only accessible when the crate is used as a source dependency
    /// or when running from the crate's directory. When used as a published dependency,
    /// the profiles may not be available as filesystem files.
    ///
    /// # Returns
    /// A PathBuf pointing to the default profiles directory.
    ///
    /// # Example
    ///
    /// ```rust
    /// use langdetect_rs::detector_factory::DetectorFactory;
    /// use langdetect_rs::utils::lang_profile::{LangProfileJson, LangProfile};
    ///
    /// // Get path to default profiles
    /// let profiles_path = DetectorFactory::get_default_profiles_path();
    /// println!("Default profiles are located at: {:?}", profiles_path);
    ///
    /// // Load a specific profile
    /// let en_profile = LangProfileJson::new_from_file(profiles_path.join("en")).unwrap();
    /// let profile = LangProfile::from_json(en_profile).unwrap();
    ///
    /// // Add to custom factory
    /// let mut factory = DetectorFactory::new().build();
    /// factory.add_profile(profile, 0, 1).unwrap();
    /// ```
    pub fn get_default_profiles_path() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("profiles")
    }

    /// Clears all loaded language profiles and mappings.
    pub fn clear(&mut self) {
        self.langlist.clear();
        self.word_lang_prob_map.clear();
    }

    /// Sets the randomization seed for reproducible results.
    ///
    /// # Arguments
    /// * `seed` - The seed value to use for randomization.
    pub fn set_seed(&mut self, seed: u64) {
        self.seed = Some(seed);
    }

    /// Returns a list of all loaded language identifiers.
    ///
    /// # Returns
    /// A vector of language codes (ISO 639-1) in the order they were loaded.
    pub fn get_lang_list(&self) -> Vec<String> {
        self.langlist.clone()
    }

    /// Creates a new Detector instance with the current profiles.
    ///
    /// # Arguments
    /// * `alpha` - Optional alpha smoothing parameter (default: 0.5).
    ///
    /// # Returns
    /// A configured Detector ready for language detection.
    pub fn create(&self, alpha: Option<f64>) -> Detector {
        let mut detector = Detector::new(
            self.word_lang_prob_map.clone(),
            self.langlist.clone(),
            self.seed,
        );
        if let Some(a) = alpha {
            detector.alpha = a;
        }
        detector
    }

    /// Overrides an existing language profile at the specified index.
    ///
    /// This is an internal method used during profile loading.
    ///
    /// # Arguments
    /// * `profile` - The language profile to add.
    /// * `index` - The index in the language list.
    /// * `langsize` - Total number of languages.
    pub fn override_profile(&mut self, profile: LangProfile, index: usize, langsize: usize) -> Result<(), DetectorFactoryError> {
        let lang = profile.name.clone().unwrap();
        self.langlist.push(lang.clone());
        for (word, &count) in profile.freq.iter() {
            if !self.word_lang_prob_map.contains_key(word) {
                self.word_lang_prob_map.insert(word.clone(), vec![0.0; langsize]);
            }
            let length = word.chars().count();
            if length >= 1 && length <= 3 {
                let prob = count as f64 / profile.n_words[length - 1] as f64;
                if let Some(vec) = self.word_lang_prob_map.get_mut(word) {
                    vec[index] = prob;
                }
            }
        }
        Ok(())
    }

    /// Adds a new language profile to the factory.
    ///
    /// # Arguments
    /// * `profile` - The language profile to add.
    /// * `index` - The index position for this language.
    /// * `langsize` - Total number of languages in the profile set.
    ///
    /// # Errors
    /// Returns `DetectorFactoryError::DuplicatedLanguage` if the language already exists.
    pub fn add_profile(&mut self, profile: LangProfile, index: usize, langsize: usize) -> Result<(), DetectorFactoryError> {
        let lang = profile.name.clone().unwrap();
        if self.langlist.contains(&lang) {
            return Err(DetectorFactoryError::DuplicatedLanguage(lang));
        }
        self.override_profile(profile, index, langsize)
    }

    /// Removes a language profile from the factory.
    ///
    /// # Arguments
    /// * `lang` - The language code to remove.
    ///
    /// # Errors
    /// Returns `DetectorFactoryError::DuplicatedLanguage` if the language doesn't exist.
    pub fn delete_profile(&mut self, lang: &str) -> Result<(), DetectorFactoryError> {
        let pos = self.langlist.iter().position(|l| l == lang);
        if let Some(index) = pos {
            self.langlist.remove(index);
            // Remove the language's probabilities from word_lang_prob_map
            for vec in self.word_lang_prob_map.values_mut() {
                if vec.len() > index {
                    vec.remove(index);
                }
            }
            Ok(())
        } else {
            Err(DetectorFactoryError::DuplicatedLanguage(lang.to_string()))
        }
    }

    /// Loads language profiles from JSON strings.
    ///
    /// # Arguments
    /// * `json_profiles` - Array of JSON strings representing language profiles.
    ///
    /// # Errors
    /// Returns `DetectorFactoryError::NotEnoughProfiles` if fewer than 2 profiles provided.
    pub fn load_json_profile(&mut self, json_profiles: &[&str]) -> Result<(), DetectorFactoryError> {
        let langsize = json_profiles.len();
        if langsize < 2 {
            return Err(DetectorFactoryError::NotEnoughProfiles);
        }
        let mut index = 0;
        for json_profile in json_profiles {
            let json_data: LangProfileJson = serde_json::from_str(json_profile)
                .map_err(|_| DetectorFactoryError::NotEnoughProfiles)?;
            let profile = LangProfile {
                name: Some(json_data.name),
                freq: json_data.freq,
                n_words: {
                    let mut arr = [0; 3];
                    for (i, v) in json_data.n_words.iter().enumerate().take(3) {
                        arr[i] = *v;
                    }
                    arr
                },
            };
            self.add_profile(profile, index, langsize)?;
            index += 1;
        }
        Ok(())
    }

    /// Shortcut method to detect language from text in one call.
    ///
    /// # Arguments
    /// * `text` - The text to analyze.
    /// * `alpha` - Optional alpha smoothing parameter.
    ///
    /// # Returns
    /// The detected language code or an error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use langdetect_rs::detector_factory::DetectorFactory;
    ///
    /// let factory = DetectorFactory::default().build();
    /// let result = factory.detect("Hello world!", None);
    /// ```
    pub fn detect(&self, text: &str, alpha: Option<f64>) -> Result<String, DetectorError> {
        let mut detector = self.create(alpha);
        detector.append(text);
        detector.detect()
    }

    /// Shortcut method to get language probabilities from text in one call.
    ///
    /// # Arguments
    /// * `text` - The text to analyze.
    /// * `alpha` - Optional alpha smoothing parameter.
    ///
    /// # Returns
    /// A vector of languages with their probabilities, sorted by probability descending.
    ///
    /// # Example
    ///
    /// ```rust
    /// use langdetect_rs::detector_factory::DetectorFactory;
    ///
    /// let factory = DetectorFactory::default().build();
    /// let result = factory.get_probabilities("Hello world!", None);
    /// ```
    pub fn get_probabilities(&self, text: &str, alpha: Option<f64>) -> Result<Vec<Language>, DetectorError> {
        let mut detector = self.create(alpha);
        detector.append(text);
        detector.get_probabilities()
    }

    /// Loads all language profiles from a directory of JSON files.
    ///
    /// # Arguments
    /// * `profile_directory` - Path to directory containing JSON profile files.
    ///
    /// # Returns
    /// Ok(()) on success, or an error string on failure.
    ///
    /// # Example
    ///
    /// ```rust
    /// use langdetect_rs::detector_factory::DetectorFactory;
    ///
    /// let mut factory = DetectorFactory::new().build();
    /// factory.load_profile("profiles/").unwrap();
    /// ```
    pub fn load_profile<P: AsRef<Path>>(&mut self, profile_directory: P) -> Result<(), String> {
        let dir = profile_directory.as_ref();
        let entries = fs::read_dir(dir).map_err(|e| format!("Failed to read profile directory: {}", e))?;
        let mut json_profiles = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();
            if path.is_file() {
                let content = fs::read_to_string(&path)
                    .map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;
                json_profiles.push(content);
            }
        }
        let json_refs: Vec<&str> = json_profiles.iter().map(|s| s.as_str()).collect();
        self.load_json_profile(&json_refs)
            .map_err(|e| format!("Failed to parse JSON profiles: {:?}", e))?;
        Ok(())
    }
}

/// Builder for `DetectorFactory` with fluent setters.
///
/// Provides a convenient way to configure a DetectorFactory before building it.
///
/// # Examples
///
/// ```rust
/// use langdetect_rs::detector_factory::DetectorFactory;
/// use std::collections::HashMap;
///
/// let factory = DetectorFactory::new()
///     .with_langlist(vec!["en".to_string(), "fr".to_string()])
///     .with_seed(Some(42))
///     .build();
/// ```
pub struct DetectorFactoryBuilder {
    factory: DetectorFactory,
}

impl DetectorFactoryBuilder {
    /// Set the word language probability map.
    ///
    /// # Arguments
    /// * `word_lang_prob_map` - A HashMap of word to language probabilities.
    ///
    /// # Example
    /// ```
    /// use std::collections::HashMap;
    /// use langdetect_rs::detector_factory::DetectorFactory;
    /// let mut word_lang_prob_map = HashMap::new();
    /// word_lang_prob_map.insert("hello".to_string(), vec![0.5, 0.3]);
    /// let builder = DetectorFactory::new().with_word_lang_prob_map(word_lang_prob_map);
    /// ```
    pub fn with_word_lang_prob_map(mut self, word_lang_prob_map: HashMap<String, Vec<f64>>) -> Self {
        self.factory.word_lang_prob_map = word_lang_prob_map;
        self
    }

    /// Set the language list.
    ///
    /// # Arguments
    /// * `langlist` - A vector of language names.
    ///
    /// # Example
    /// ```
    /// use langdetect_rs::detector_factory::DetectorFactory;
    /// let builder = DetectorFactory::new().with_langlist(vec!["en".to_string(), "fr".to_string()]);
    /// ```
    pub fn with_langlist(mut self, langlist: Vec<String>) -> Self {
        self.factory.langlist = langlist;
        self
    }

    /// Set the seed for randomization.
    ///
    /// # Arguments
    /// * `seed` - An optional u64 seed value.
    ///
    /// # Example
    /// ```
    /// use langdetect_rs::detector_factory::DetectorFactory;
    /// let builder = DetectorFactory::new().with_seed(Some(42));
    /// ```
    pub fn with_seed(mut self, seed: Option<u64>) -> Self {
        self.factory.seed = seed;
        self
    }

    /// Builds the final `DetectorFactory` object with the configured properties.
    ///
    /// # Returns
    /// The fully constructed `DetectorFactory` object.
    ///
    /// # Example
    /// ```
    /// use langdetect_rs::detector_factory::DetectorFactory;
    /// let factory = DetectorFactory::new().with_seed(Some(123)).build();
    /// ```
    pub fn build(self) -> DetectorFactory {
        self.factory
    }
}