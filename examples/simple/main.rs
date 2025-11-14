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