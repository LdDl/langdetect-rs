# langdetect-rs

Port of Mimino666's [langdetect](https://github.com/Mimino666/langdetect) which is Python-based port of Nakatani Shuyo's [language-detection](https://github.com/shuyo/language-detection) Java-based library. Even this README is mostly a copy of the Mimino666's one.

Language identification library for Rust.

W.I.P.
- Allow to add new languages
- Comprehensive documentation for developers
- Threadsafe API (do we need it though?)
- Publish to crates.io

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

You can run the included example:

```sh
cargo run --example simple
```


Example code (`examples/simple/main.rs`):

```rust
use langdetect_rs::detector_factory::DetectorFactory;

fn main() {
    let factory = DetectorFactory::default();

    let mut detector = factory.create(None);
    detector.append("War doesn't show who's right, just who's left.");
    match detector.detect() {
        Ok(lang) => println!("Detected language: {}", lang),
        Err(e) => println!("Detection error: {:?}", e),
    }

    let mut detector = factory.create(None);
    detector.append("Ein, zwei, drei, vier");
    match detector.detect() {
        Ok(lang) => println!("Detected language: {}", lang),
        Err(e) => println!("Detection error: {:?}", e),
    }

    let mut detector = factory.create(None);
    // For reproducibility
    detector.seed = Some(42);
    detector.append("Otec matka syn.");
    match detector.get_probabilities() {
        Ok(probs) => println!("Language probabilities: {:?}", probs),
        Err(e) => println!("Detection error: {:?}", e),
    }
}
```

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
