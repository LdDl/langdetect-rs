use std::fs;
use std::path::Path;
use serde::{Deserialize};
use serde_json;
use std::collections::HashMap;
use crate::utils::lang_profile::LangProfile;
use crate::detector::{Detector, DetectorError};
use crate::language::Language;

#[derive(Deserialize)]
pub struct LangProfileJson {
    pub freq: HashMap<String, usize>,
    pub n_words: Vec<usize>,
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum DetectorFactoryError {
    DuplicatedLanguage(String),
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

#[derive(Clone)]
pub struct DetectorFactory {
    pub word_lang_prob_map: HashMap<String, Vec<f64>>,
    pub langlist: Vec<String>,
    pub seed: Option<u64>,
}

impl DetectorFactory {
    /// Use `.build()` after preparing all needed options to obtain the `DetectorFactory`.
    pub fn new() -> DetectorFactoryBuilder {
        DetectorFactoryBuilder {
            factory: DetectorFactory {
                word_lang_prob_map: HashMap::new(),
                langlist: Vec::new(),
                seed: None,
            },
        }
    }

    /// Create a DetectorFactory with profiles loaded from crate-level profiles folder
    pub fn default() -> Self {
        use std::sync::Mutex;
        use lazy_static::lazy_static;
        lazy_static! {
            static ref DEFAULT_FACTORY: Mutex<Option<DetectorFactory>> = Mutex::new(None);
        }
        {
            let factory_guard = DEFAULT_FACTORY.lock().unwrap();
            if let Some(factory) = &*factory_guard {
                return factory.clone();
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
        factory
    }

    pub fn clear(&mut self) {
        self.langlist.clear();
        self.word_lang_prob_map.clear();
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.seed = Some(seed);
    }

    pub fn get_lang_list(&self) -> Vec<String> {
        self.langlist.clone()
    }

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

    pub fn add_profile(&mut self, profile: LangProfile, index: usize, langsize: usize) -> Result<(), DetectorFactoryError> {
        let lang = profile.name.clone().unwrap();
        if self.langlist.contains(&lang) {
            return Err(DetectorFactoryError::DuplicatedLanguage(lang));
        }
        self.override_profile(profile, index, langsize)
    }

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

    /// Shortcut method
    pub fn detect(&self, text: &str, alpha: Option<f64>) -> Result<String, DetectorError> {
        let mut detector = self.create(alpha);
        detector.append(text);
        detector.detect()
    }

    /// Shortcut method
    pub fn get_probabilities(&self, text: &str, alpha: Option<f64>) -> Result<Vec<Language>, DetectorError> {
        let mut detector = self.create(alpha);
        detector.append(text);
        detector.get_probabilities()
    }

    /// Load all language profiles from a directory of JSON files
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