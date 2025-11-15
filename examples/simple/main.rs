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