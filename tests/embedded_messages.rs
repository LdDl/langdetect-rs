use langdetect_rs::utils::{messages, ngram::NGram};

#[test]
fn embedded_properties_preserve_unicode_and_missing_key_behavior() {
    let local = messages::Messages::new();
    assert_eq!(local.get_string("NGram.LATIN1_EXCLUDE"), "\u{00a0}«°»");
    assert_eq!(local.get_string("NGram.KANJI_1_0"), "佼椴");
    assert_eq!(local.get_string("missing.property"), "!missing.property!");
    assert_eq!(messages::get_string("NGram.LATIN1_EXCLUDE"), "\u{00a0}«°»");
}

#[test]
fn normalization_uses_embedded_character_tables() {
    for character in ['\u{00a0}', '«', '°', '»'] {
        assert_eq!(NGram::normalize(character), ' ');
    }
    assert_eq!(NGram::normalize('椴'), NGram::normalize('佼'));
    assert_ne!(NGram::normalize('椴'), '椴');
}
