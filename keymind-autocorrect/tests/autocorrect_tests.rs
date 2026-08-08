use keymind_autocorrect::{AutocorrectEngine, Correction};

async fn setup_test_engine() -> AutocorrectEngine {
    let tmp_path = std::env::temp_dir().join(format!("test_autocorrect_{}.json", std::time::UNIX_EPOCH.elapsed().unwrap().as_nanos()));
    let engine = AutocorrectEngine::new(tmp_path);
    engine.initialize().await.unwrap();
    engine
}

#[tokio::test]
async fn test_personal_dictionary_bypass() {
    let engine = setup_test_engine().await;
    engine.add_to_personal_dict("customword");

    let result = engine.check("customword", "this is ");
    assert!(result.is_none(), "Personal dictionary word should never be corrected");
}

#[tokio::test]
async fn test_learned_corrections_layer() {
    let engine = setup_test_engine().await;
    engine.record_user_correction_in_memory("teh", "the", 3);

    let result = engine.check("teh", "in ");
    assert_eq!(
        result,
        Some(Correction {
            original: "teh".to_string(),
            corrected: "the".to_string(),
            confidence: 1.0,
        })
    );
}

#[tokio::test]
async fn test_homophone_resolution() {
    let engine = setup_test_engine().await;
    let result = engine.check("their", "going over ");

    assert_eq!(
        result,
        Some(Correction {
            original: "their".to_string(),
            corrected: "there".to_string(),
            confidence: 0.95,
        })
    );
}

#[tokio::test]
async fn test_symspell_misspelling_correction() {
    let engine = setup_test_engine().await;
    let result = engine.check("teh", "in ");

    assert!(result.is_some());
    let correction = result.unwrap();
    assert_eq!(correction.corrected, "the");
}

#[tokio::test]
async fn test_single_character_bypass() {
    let engine = setup_test_engine().await;
    let result = engine.check("a", "is ");
    assert!(result.is_none());
}

#[tokio::test]
async fn test_high_frequency_word_typo_corrections() {
    let engine = setup_test_engine().await;

    let test_cases = vec![
        // User requested high-frequency words
        ("teh", "the"),
        ("hte", "the"),
        ("hallo", "hello"),
        ("helo", "hello"),
        ("corect", "correct"),
        ("woudl", "would"),
        ("wuld", "would"),
        ("coudl", "could"),
        ("shoudl", "should"),
        ("becuase", "because"),
        ("definately", "definitely"),
        ("seperate", "separate"),
        ("recieve", "receive"),
        ("beleive", "believe"),
        ("tommorow", "tomorrow"),
        ("buisness", "business"),
        ("goverment", "government"),
        ("enviroment", "environment"),
        ("expierence", "experience"),
        ("langauge", "language"),
        ("truely", "truly"),
        ("freind", "friend"),
        ("untill", "until"),
        ("welcom", "welcome"),
        ("pleas", "please"),
        ("thx", "thanks"),
    ];

    for (typo, expected_correction) in test_cases {
        let result = engine.check(typo, "this is ");
        assert!(
            result.is_some(),
            "Expected autocorrect result for typo '{}', but got None",
            typo
        );
        let correction = result.unwrap();
        assert_eq!(
            correction.corrected, expected_correction,
            "Expected typo '{}' to be corrected to '{}', but got '{}'",
            typo, expected_correction, correction.corrected
        );
    }
}
