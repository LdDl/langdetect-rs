use langdetect_rs::detector_factory::DetectorFactory;
use langdetect_rs::utils::lang_profile::{LangProfileJson, LangProfile};
use std::path::Path;

fn main() {
    // Load language profiles from the crate's profiles directory
    let profiles_dir = DetectorFactory::get_default_profiles_path();

    println!("Read all default JSON profiles from {}", profiles_dir.display());

    let mut lang_profiles = vec![];
    // Load every profile in the directory
    for entry in std::fs::read_dir(&profiles_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let lang_json = LangProfileJson::new_from_file(path);
        match &lang_json {
            Ok(_) => println!("\tRead {} JSON profile", entry.file_name().to_string_lossy()),
            Err(e) => {
                println!("Error reading {} JSON profile: {:?}", entry.file_name().to_string_lossy(), e);
                return;
            }
        }
        let lang_profile = match LangProfile::from_json(lang_json.unwrap()) {
            Ok(profile) => profile,
            Err(e) => {
                println!("Error creating {} LangProfile: {}", entry.file_name().to_string_lossy(), e);
                return;
            }
        };
        lang_profiles.push(lang_profile);
    }

    println!("Adding all default languages profiles to the factory...");

    // Create an EMPTY factory
    let mut factory = DetectorFactory::new().build();

    // Get number of profiles to set final size
    let mut profile_count = lang_profiles.len();
    // Since we know that we are going to add another profile which is not in the default set
    // we increase the final size by 1
    profile_count += 1;

    println!("Final size (assuming we are going to extend default set) of languages array will be: {}", profile_count);
    
    for (i, profile) in lang_profiles.into_iter().enumerate() {
        let profile_name = if let Some(name) = &profile.name {
            name.clone()
        } else {
            "unknown".to_string()
        };
        println!("\tAdding profile: {} at index {}", profile_name, i);
        if let Err(e) = factory.add_profile(profile, i, profile_count) {
            println!("Error adding {} profile: {:?}", profile_name, e);
            return;
        }
    }

    // Load another profile (in documentation for generating profiles it is Sakha (Yakut) language - "sah")
    let sah_path = Path::new("./scripts/datasets/generated").join("sah_generated.json");
    let sah_json = LangProfileJson::new_from_file(sah_path);
    match &sah_json {
        Ok(_) => println!("Read Sakha JSON profile"),
        Err(e) => {
            println!("Error reading Sakha JSON profile: {:?}", e);
            return;
        }
    }
    let sah_profile = match LangProfile::from_json(sah_json.unwrap()) {
        Ok(profile) => profile,
        Err(e) => {
            println!("Error creating Sakha LangProfile: {}", e);
            return;
        }
    };
    println!("Adding Sakha language profile to the factory");
    if let Err(e) = factory.add_profile(sah_profile, profile_count - 1, profile_count) {
        println!("Error adding Sakha profile: {:?}", e);
        return;
    }

    println!("Testing language detection...");
    // Test Russian text
    match factory.detect("В своём глазу бревна не замечает, а в чужом соломинку видит", None) {
        Ok(lang) => println!("\tRussian text detected as: {}", lang),
        Err(e) => println!("Detection error: {:?}", e),
    }

    // Test English text
    match factory.detect("He pays no attention to the plank in his own eye", None) {
        Ok(lang) => println!("\tEnglish text detected as: {}", lang),
        Err(e) => println!("Detection error: {:?}", e),
    }

    // Test Sakha (Yakut) text
    match factory.detect("Айаҕыттан тахсар сытыканы билбэт.", None) {
        Ok(lang) => println!("\tSakha text detected as: {}", lang),
        Err(e) => println!("Detection error: {:?}", e),
    }
}