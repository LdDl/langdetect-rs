use langdetect_rs::detector_factory::DetectorFactory;
use langdetect_rs::utils::lang_profile::LangProfileJson;

#[test]
fn default_contains_exactly_56_valid_profiles_including_uzbek() {
    let expected: Vec<String> = "af ar bg bn ca cs cy da de el en es et fa fi fr gu he hi hr hu id it ja kn ko lt lv mk ml mr ne nl no pa pl pt ro ru sk sl so sq sv sw ta te th tl tr uk ur uz vi zh-cn zh-tw"
        .split_whitespace()
        .map(str::to_owned)
        .collect();
    let profiles = DetectorFactory::get_default_profiles_json();
    let names: Vec<_> = profiles
        .iter()
        .map(|json| {
            let profile: LangProfileJson = serde_json::from_str(json).unwrap();
            assert_eq!(profile.n_words.len(), 3);
            assert!(profile.n_words.iter().all(|count| *count > 0));
            assert!(!profile.freq.is_empty());
            profile.name
        })
        .collect();
    assert_eq!(names, expected);

    let factory = DetectorFactory::default().build();
    assert_eq!(factory.get_lang_list(), expected);
    assert!(!factory.word_lang_prob_map.is_empty());
    for probabilities in factory.word_lang_prob_map.values() {
        assert_eq!(probabilities.len(), expected.len());
        assert!(
            probabilities
                .iter()
                .all(|probability| probability.is_finite() && *probability >= 0.0)
        );
    }
    for custom_language in ["az", "sah", "tg"] {
        assert!(
            !factory
                .get_lang_list()
                .iter()
                .any(|name| name == custom_language)
        );
    }
}

#[test]
fn default_detects_languages_using_embedded_profiles() {
    let factory = DetectorFactory::default().with_seed(Some(0)).build();
    let samples = [
        (
            "ru",
            "Сегодня в городе открылась новая библиотека. Жители посетили читальные залы, сотрудники рассказали о новых книгах и расписании мероприятий.",
        ),
        (
            "en",
            "The city opened a new public library today. Visitors explored the reading rooms and learned about the schedule of community events.",
        ),
        (
            "uz",
            "Bugun shaharda yangi kutubxona ochildi. Odamlar kitoblarni o‘qish va yangi bilimlarni olish uchun kelishdi.",
        ),
        ("zh-cn", "你好世界。今天我们去图书馆读书。"),
    ];
    for (expected, text) in samples {
        assert_eq!(factory.detect(text, None).unwrap(), expected);
    }
}

#[test]
fn custom_factory_can_extend_embedded_profiles_without_changing_defaults() {
    let custom = r#"{"name":"test","freq":{"x":1},"n_words":[1,1,1]}"#;
    let mut profiles = DetectorFactory::get_default_profiles_json().to_vec();
    profiles.push(custom);
    let mut factory = DetectorFactory::new().build();
    factory.load_json_profile(&profiles).unwrap();
    assert_eq!(factory.get_lang_list().len(), 57);
    assert_eq!(factory.get_lang_list().last().unwrap(), "test");
    assert!(
        factory
            .word_lang_prob_map
            .values()
            .all(|probabilities| probabilities.len() == 57)
    );

    let mut independent = DetectorFactory::default().with_seed(Some(42)).build();
    independent.clear();
    let original = DetectorFactory::default().build();
    assert_eq!(original.get_lang_list().len(), 56);
    assert_eq!(original.seed, None);
}
