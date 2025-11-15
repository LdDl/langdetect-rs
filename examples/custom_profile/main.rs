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