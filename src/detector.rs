use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;
use rand_distr::{Normal, Distribution};

use crate::language::Language;
use crate::utils::ngram::NGram;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum DetectorError {
    NoFeatures,
}

impl std::fmt::Display for DetectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DetectorError::NoFeatures => write!(f, "No features found in the input text"),
        }
    }
}

pub struct Detector {
    pub word_lang_prob_map: HashMap<String, Vec<f64>>,
    pub langlist: Vec<String>,
    pub seed: Option<u64>,
    pub text: String,
    pub langprob: Option<Vec<f64>>,
    pub alpha: f64,
    pub n_trial: usize,
    pub max_text_length: usize,
    pub prior_map: Option<Vec<f64>>,
    pub verbose: bool,
}

impl Detector {
    pub const ALPHA_DEFAULT: f64 = 0.5;
    pub const ALPHA_WIDTH: f64 = 0.05;
    pub const ITERATION_LIMIT: usize = 1000;
    pub const PROB_THRESHOLD: f64 = 0.1;
    pub const CONV_THRESHOLD: f64 = 0.99999;
    pub const BASE_FREQ: f64 = 10000.0;
    pub const UNKNOWN_LANG: &'static str = "unknown";

    pub fn new(word_lang_prob_map: HashMap<String, Vec<f64>>, langlist: Vec<String>, seed: Option<u64>) -> Self {
        Detector {
            word_lang_prob_map,
            langlist,
            seed,
            text: String::new(),
            langprob: None,
            alpha: Self::ALPHA_DEFAULT,
            n_trial: 7,
            max_text_length: 10000,
            prior_map: None,
            verbose: false,
        }
    }

    pub fn append(&mut self, text: &str) {
        // Remove URLs and emails (simple regex)
        let url_re = regex::Regex::new(r"https?://[-_.?&~;+=/#0-9A-Za-z]{1,2076}").unwrap();
        let mail_re = regex::Regex::new(r"[-_.0-9A-Za-z]{1,64}@[-_0-9A-Za-z]{1,255}[-_.0-9A-Za-z]{1,255}").unwrap();
        let mut text = url_re.replace_all(text, " ").to_string();
        text = mail_re.replace_all(&text, " ").to_string();
        text = NGram::normalize_vi(&text);
        let mut pre = ' ';
        for ch in text.chars().take(self.max_text_length) {
            if ch != ' ' || pre != ' ' {
                self.text.push(ch);
            }
            pre = ch;
        }
    }

    fn cleaning_text(&mut self) {
        let mut latin_count = 0;
        let mut non_latin_count = 0;
        for ch in self.text.chars() {
            if ('A'..='z').contains(&ch) {
                latin_count += 1;
            } else if ch >= '\u{0300}' {
                if let Some(block) = crate::utils::unicode_block::unicode_block(ch) {
                    if block != crate::utils::unicode_block::UNICODE_LATIN_EXTENDED_ADDITIONAL {
                        non_latin_count += 1;
                    }
                }
            }
        }
        if latin_count * 2 < non_latin_count {
            let mut text_without_latin = String::new();
            for ch in self.text.chars() {
                if ch < 'A' || ch > 'z' {
                    text_without_latin.push(ch);
                }
            }
            self.text = text_without_latin;
        }
    }


    pub fn detect(&mut self) -> Result<String, DetectorError> {
        let probabilities = self.get_probabilities()?;
        if !probabilities.is_empty() {
            Ok(probabilities[0].lang.clone().unwrap_or_else(|| Self::UNKNOWN_LANG.to_string()))
        } else {
            Ok(Self::UNKNOWN_LANG.to_string())
        }
    }


    pub fn get_probabilities(&mut self) -> Result<Vec<Language>, DetectorError> {
        if self.langprob.is_none() {
            self.detect_block()?;
        }
        Ok(self.sort_probability(self.langprob.as_ref().unwrap()))
    }


    fn detect_block(&mut self) -> Result<(), DetectorError> {
        self.cleaning_text();
        let ngrams = self.extract_ngrams();
        if ngrams.is_empty() {
            return Err(DetectorError::NoFeatures);
        }
        self.langprob = Some(vec![0.0; self.langlist.len()]);
        let mut rng = if let Some(seed) = self.seed {
            StdRng::seed_from_u64(seed)
        } else {
            let mut thread_rng = rand::rng();
            StdRng::from_rng(&mut thread_rng)
        };
        for _t in 0..self.n_trial {
            let mut prob = self.init_probability();
            let normal = Normal::new(0.0, 1.0).unwrap();
            let alpha = self.alpha + normal.sample(&mut rng) * Self::ALPHA_WIDTH;
            let mut i = 0;
            loop {
                let word = ngrams[rng.random_range(0..ngrams.len())].clone();
                self.update_lang_prob(&mut prob, &word, alpha);
                if i % 5 == 0 {
                    if self.normalize_prob(&mut prob) > Self::CONV_THRESHOLD || i >= Self::ITERATION_LIMIT {
                        break;
                    }
                }
                i += 1;
            }
            for j in 0..self.langprob.as_ref().unwrap().len() {
                self.langprob.as_mut().unwrap()[j] += prob[j] / self.n_trial as f64;
            }
        }
        Ok(())
    }

    fn init_probability(&self) -> Vec<f64> {
        if let Some(ref prior) = self.prior_map {
            prior.clone()
        } else {
            vec![1.0 / self.langlist.len() as f64; self.langlist.len()]
        }
    }

    fn extract_ngrams(&self) -> Vec<String> {
        let range = 1..=NGram::N_GRAM;
        let mut result = Vec::new();
        let mut ngram = NGram::new();
        for ch in self.text.chars() {
            ngram.add_char(ch);
            if ngram.capitalword {
                continue;
            }
            for n in range.clone() {
                if ngram.grams.len() < n {
                    break;
                }
                let w: String = ngram.grams.chars().rev().take(n).collect::<Vec<_>>().into_iter().rev().collect();
                if !w.is_empty() && w != " " && self.word_lang_prob_map.contains_key(&w) {
                    result.push(w);
                }
            }
        }
        result
    }

    fn update_lang_prob(&self, prob: &mut [f64], word: &str, alpha: f64) -> bool {
        if !self.word_lang_prob_map.contains_key(word) {
            return false;
        }
        let lang_prob_map = &self.word_lang_prob_map[word];
        let weight = alpha / Self::BASE_FREQ;
        for i in 0..prob.len() {
            prob[i] *= weight + lang_prob_map[i];
        }
        true
    }

    fn normalize_prob(&self, prob: &mut [f64]) -> f64 {
        let sump: f64 = prob.iter().sum();
        let mut maxp = 0.0;
        for p in prob.iter_mut() {
            *p /= sump;
            if maxp < *p {
                maxp = *p;
            }
        }
        maxp
    }

    fn sort_probability(&self, prob: &[f64]) -> Vec<Language> {
        let mut result: Vec<Language> = self.langlist.iter().zip(prob.iter())
            .filter(|(_, p)| **p > Self::PROB_THRESHOLD)
            .map(|(lang, &p)| Language::new(Some(lang.clone()), p)).collect();
        result.sort_by(|a, b| b.partial_cmp(a).unwrap());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detector_factory::DetectorFactory;
    use crate::utils::lang_profile::LangProfile;

    fn setup_factory() -> DetectorFactory {
        let mut factory = DetectorFactory::new();

        let mut profile_en = LangProfile::with_name("en");
        for w in ["a", "a", "a", "b", "b", "c", "c", "d", "e"].iter() {
            profile_en.add(w);
        }
        let result = factory.add_profile(profile_en, 0, 3);
        assert!(result.is_ok(), "Unexpected error in add_profile: {:?}", result);
        result.unwrap();

        let mut profile_fr = LangProfile::with_name("fr");
        for w in ["a", "b", "b", "c", "c", "c", "d", "d", "d"].iter() {
            profile_fr.add(w);
        }
        let result = factory.add_profile(profile_fr, 1, 3);
        assert!(result.is_ok(), "Unexpected error in add_profile: {:?}", result);
        result.unwrap();

        let mut profile_ja = LangProfile::with_name("ja");
        for w in ["\u{3042}", "\u{3042}", "\u{3042}", "\u{3044}", "\u{3046}", "\u{3048}", "\u{3048}"].iter() {
            profile_ja.add(w);
        }
        let result = factory.add_profile(profile_ja, 2, 3);
        assert!(result.is_ok(), "Unexpected error in add_profile: {:?}", result);
        result.unwrap();

        factory
    }

    #[test]
    fn test_detector1() {
        let factory = setup_factory();
        let mut detect = factory.create(None);
        detect.append("a");
        let result = detect.detect();
        assert!(result.is_ok(), "Unexpected error: {:?}", result);
        let lang = result.unwrap();
        assert_eq!(lang, "en");
    }

    #[test]
    fn test_detector2() {
        let factory = setup_factory();
        let mut detect = factory.create(None);
        detect.append("b d");
        let result = detect.detect();
        assert!(result.is_ok(), "Unexpected error: {:?}", result);
        let lang = result.unwrap();
        assert_eq!(lang, "fr");
    }

    #[test]
    fn test_detector3() {
        let factory = setup_factory();
        let mut detect = factory.create(None);
        detect.append("d e");
        let result = detect.detect();
        assert!(result.is_ok(), "Unexpected error: {:?}", result);
        let lang = result.unwrap();
        assert_eq!(lang, "en");
    }

    #[test]
    fn test_detector4() {
        let factory = setup_factory();
        let mut detect = factory.create(None);
        detect.append("\u{3042}\u{3042}\u{3042}\u{3042}a");
        let result = detect.detect();
        assert!(result.is_ok(), "Unexpected error: {:?}", result);
        let lang = result.unwrap();
        assert_eq!(lang, "ja");
    }

    #[test]
    fn test_lang_list() {
        let factory = setup_factory();
        let langlist = factory.get_lang_list();
        assert_eq!(langlist.len(), 3);
        assert_eq!(langlist[0], "en");
        assert_eq!(langlist[1], "fr");
        assert_eq!(langlist[2], "ja");
    }

    #[test]
    fn test_factory_from_json_string() {
        let mut factory = DetectorFactory::new();
        factory.clear();
        let json_lang1 = "{\"freq\":{\"A\":3,\"B\":6,\"C\":3,\"AB\":2,\"BC\":1,\"ABC\":2,\"BBC\":1,\"CBA\":1},\"n_words\":[12,3,4],\"name\":\"lang1\"}";
        let json_lang2 = "{\"freq\":{\"A\":6,\"B\":3,\"C\":3,\"AA\":3,\"AB\":2,\"ABC\":1,\"ABA\":1,\"CAA\":1},\"n_words\":[12,5,3],\"name\":\"lang2\"}";
        let profiles = vec![json_lang1, json_lang2];
        let profiles_ref: Vec<&str> = profiles.iter().map(|s| *s).collect();
        factory.load_json_profile(&profiles_ref).unwrap();
        let langlist = factory.get_lang_list();
        assert_eq!(langlist.len(), 2);
        assert_eq!(langlist[0], "lang1");
        assert_eq!(langlist[1], "lang2");
    }
}