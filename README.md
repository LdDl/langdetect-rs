# langdetect-rs

Port of Mimino666's [langdetect](https://github.com/Mimino666/langdetect) which is Python-based port of Nakatani Shuyo's [language-detection](https://github.com/shuyo/language-detection) Java-based library. Even this README is mostly a copy of the Mimino666's one.

Language identification library for Rust.

W.I.P.
- Benchmarking in term of speed (via hyperfine?)
- Threadsafe API (do we need it though?)

## Table of Contents

- [Installation](#installation)
- [Supported Rust Versions](#supported-rust-versions)
- [Languages](#languages)
- [Example](#example)
    - [All examples:](#all-examples)
    - [Using default detector](#using-default-detector)
    - [Custom detection factory](#custom-detection-factory)
- [How to Add a New Language?](#how-to-add-a-new-language)
- [Original project](#original-project)

## Installation

Add to your `Cargo.toml`:

```toml
langdetect-rs = "*"
```

or run
```
cargo add langdetect-rs
```

## Supported Rust Versions

Tested on Rust 1.91.0 (`rustc 1.91.0 (f8297e351 2025-10-28)`)

## Languages

`langdetect-rs` supports 55 languages out of the box ([ISO 639-1 codes](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes)):

    af, ar, bg, bn, ca, cs, cy, da, de, el, en, es, et, fa, fi, fr, gu, he,
    hi, hr, hu, id, it, ja, kn, ko, lt, lv, mk, ml, mr, ne, nl, no, pa, pl,
    pt, ro, ru, sk, sl, so, sq, sv, sw, ta, te, th, tl, tr, uk, ur, vi, zh-cn,
    zh-tw


## Example

### All examples:

```sh
cargo run --example simple
cargo run --example custom_profile
```

### Using default detector
- Simple good-to-go example code in [examples/simple/main.rs](examples/simple/main.rs):

    ```rust
    use langdetect_rs::detector_factory::DetectorFactory;

    fn main() {
        let factory = DetectorFactory::default().build();

        // let mut detector = factory.create(None);
        match factory.detect("War doesn't show who's right, just who's left.", None) {
            Ok(lang) => println!("Detected language: {}", lang),
            Err(e) => println!("Detection error: {:?}", e),
        }

        // let mut detector = factory.create(None);
        match factory.detect("Ein, zwei, drei, vier", None) {
            Ok(lang) => println!("Detected language: {}", lang),
            Err(e) => println!("Detection error: {:?}", e),
        }

        match factory.get_probabilities("Otec matka syn.", None) {
            Ok(probs) => println!("Language probabilities: {:?}", probs),
            Err(e) => println!("Detection error: {:?}", e),
        }

        // For reproducibility use a fixed seed within explicitly defined detector
        let mut detector = factory.create(None);
        detector.seed = Some(42);
        detector.append("Otec matka syn.");
        match detector.get_probabilities() {
            Ok(probs) => println!("Language probabilities with seed: {:?}", probs),
            Err(e) => println!("Detection error: {:?}", e),
        }

        // Or you can set the seed for the factory itself and it will be inherited by detectors
        let factory_with_seed = DetectorFactory::default()
            .with_seed(Some(43))
            .build();
        match factory_with_seed.get_probabilities("Otec matka syn.", None) {
            Ok(probs) => println!("Language probabilities with seed: {:?}", probs),
            Err(e) => println!("Detection error: {:?}", e),
        }
    }
    ```

### Custom detection factory
- Defining `DetectorFactory` from scratch for specific languages - [./examples/custom_profile/main.rs](examples/custom_profile/main.rs)

    ```rust
    use langdetect_rs::detector_factory::DetectorFactory;
    use langdetect_rs::utils::lang_profile::{LangProfileJson, LangProfile};
    use std::path::Path;

    fn main() {
        // Create an empty factory
        let mut factory = DetectorFactory::new().build();

        // Load language profiles from the crate's profiles directory
        let profiles_dir = Path::new("./").join("profiles");

        println!("Read JSON profiles from {}", profiles_dir.display());

        // Load Russian profile
        let ru_json = LangProfileJson::new_from_file(profiles_dir.join("ru"));
        match &ru_json {
            Ok(_) => println!("\tRead Russian JSON profile"),
            Err(e) => {
                println!("Error reading Russian JSON profile: {:?}", e);
                return;
            }
        }
        let ru_profile = match LangProfile::from_json(ru_json.unwrap()) {
            Ok(profile) => profile,
            Err(e) => {
                println!("Error creating Russian LangProfile: {}", e);
                return;
            }
        };

        // Load English profile
        let en_json = LangProfileJson::new_from_file(profiles_dir.join("en"));
        match &en_json {
            Ok(_) => println!("\tRead English JSON profile"),
            Err(e) => {
                println!("Error reading English JSON profile: {:?}", e);
                return;
            }
        }
        let en_profile = match LangProfile::from_json(en_json.unwrap()) {
            Ok(profile) => profile,
            Err(e) => {
                println!("Error creating English LangProfile: {}", e);
                return;
            }
        };
        
        println!("Adding custom language profiles to the factory...");
        // Add profiles to the factory
        // Make sure to use correct language IDs as per your profiles
        // And provide correct FINAL size of languages array
        let final_size = 2; // Update this if you add more profiles
        if let Err(e) = factory.add_profile(ru_profile, 0, final_size
        ) {
            println!("Error adding Russian profile: {:?}", e);
            return;
        }
        println!("\tLoaded Russian profile");
        if let Err(e) = factory.add_profile(en_profile, 1, final_size) {
            println!("Error adding English profile: {:?}", e);
            return;
        }
        println!("\tLoaded English profile");

        println!("Factory loaded with {} languages: {:?}", factory.get_lang_list().len(), factory.get_lang_list());

        println!("Testing language detection...");

        // Test Russian text
        match factory.detect("Привет, меня зовут Дима, и я разработчик", None) {
            Ok(lang) => println!("\tRussian text detected as: {}", lang),
            Err(e) => println!("Detection error: {:?}", e),
        }

        // Test English text
        match factory.detect("Hello world! My name is Dima and I am a developer", None) {
            Ok(lang) => println!("\tEnglish text detected as: {}", lang),
            Err(e) => println!("Detection error: {:?}", e),
        }

        // Test French text (will be detected as the closest match from available languages)
        // IMPORTANT: The algorithm always returns the best guess from loaded languages, never fails
        // EXCEPTIONS: Returns error if no recognizable n-grams found, or "unknown" if all probabilities ≤ 0.1
        // If you want to detect "unknown" languages, check probability thresholds or handle the error cases
        match factory.detect("Bonjour tout le monde! Je m'appelle Dima et je suis développeur", None) {
            Ok(lang) => println!("\tFrench text detected as: {} (closest match from ru/en)", lang),
            Err(e) => println!("Detection error: {:?}", e),
        }

        // Show probabilities for the French text to see why it was classified as English
        match factory.get_probabilities("Bonjour tout le monde! Je m'appelle Dima et je suis développeur", None) {
            Ok(probs) => {
                println!("\tFrench text probabilities:");
                for lang in probs {
                    println!("\t\t{}: {:.3}", lang.lang.unwrap_or_default(), lang.prob);
                }
            }
            Err(e) => println!("Probability error: {:?}", e),
        }
    }
    ```

### Adding new languages
- How to add language to existing `DetectorFactory` (either default initialized or custom)?
    - The way [add_profile](src/detector_factory.rs#L273-L303) works makes it is not possible to add new language profiles to the factory unless you know the final size of languages array in advance. E.g. you initialized custom factory with 5 languages, and now you want to add 2 more - you need to provide `langsize` parameter as 7 when adding EACH new profile. Failing to do so will result in error.
    - So it is needed to initialize the factory with all desired languages at once. In case if you want to add more languages to the default factory, you can create a new custom factory and add all default profiles from [profiles](./profiles/) folder plus your new ones.

**NOTE**

Language detection algorithm is non-deterministic, which means that if you try to run it on a text which is either too short or too ambiguous, you might get different results every time you run it.

To enforce consistent results, set the seed on the detector before detection:

```rust
let mut detector = factory.create(None);
detector.seed = Some(42); // Any u64 value
detector.append("your text");
println!("Detected language: {}", detector.detect());
```

## How to Add a New Language?

This work in progress.

Initially I will take an idea from original Python library: https://github.com/Mimino666/langdetect?tab=readme-ov-file#how-to-add-new-language

And then I will try it to simplify the process.


## Original project

Presentation of the language detection algorithm (on which original implementation is based): [http://www.slideshare.net/shuyo/language-detection-library-for-java](http://www.slideshare.net/shuyo/language-detection-library-for-java).
