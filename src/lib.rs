
//! # langdetect-rs
//!
//! A Rust port of the Python langdetect library - <https://github.com/Mimino666/langdetect>, which is itself a port of the Java language-detection library.
//!
//! This crate provides automatic language identification using n-gram based text categorization.
//! It supports 55 languages out of the box and allows for custom language profile loading.
//!
//! ## Features
//!
//! - **55 built-in languages** with prepared profiles (copied from Python library version)
//! - **High accuracy** for texts longer than 20-50 characters according to original presentation (49 languages with 99.8% precision): <https://www.slideshare.net/slideshow/language-detection-library-for-java/6014274>
//! - **Non-deterministic algorithm** with optional seeding for reproducibility
//! - **Extensible** - add custom language profiles
//!
//! ## Quick Start
//!
//! ```rust
//! use langdetect_rs::detector_factory::DetectorFactory;
//!
//! let factory = DetectorFactory::default().build();
//! match factory.detect("Hello world! My name is Dima and I am a developer", None) {
//!     Ok(lang) => println!("Detected language: {}", lang),
//!     Err(e) => println!("Detection error: {:?}", e),
//! }
//! ```
//!
//! ## Algorithm Overview
//!
//! The library uses a Bayesian approach with n-gram (1-3 character sequences) frequency analysis.
//! It employs an iterative expectation-maximization algorithm to estimate language probabilities.
//!
//! ## Modules
//!
//! - [`detector_factory`] - Factory with languages profiles for creating detectors
//! - [`detector`] - Core language detection logic
//! - [`language`] - Language probability data structure
//! - [`utils`] - Utility modules for profiles, n-grams, and Unicode handling
pub mod detector;
pub mod detector_factory;
pub mod language;
pub mod utils;
