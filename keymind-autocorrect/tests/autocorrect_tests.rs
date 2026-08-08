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

#[tokio::test]
async fn test_50_plus_mixed_words_and_unseen_alterations() {
    let engine = setup_test_engine().await;

    // 60 Mixed Words: Layer 0.5 Explicit + SymSpell Dynamic Unseen Alterations
    let test_cases = vec![
        // --- 1. Conversational & Everyday Typos ---
        ("tommorow", "tomorrow"),
        ("definately", "definitely"),
        ("seperate", "separate"),
        ("buisness", "business"),
        ("goverment", "government"),
        ("enviroment", "environment"),
        ("expierence", "experience"),
        ("langauge", "language"),
        ("truely", "truly"),
        ("freind", "friend"),

        // --- 2. Hinglish & Desi Words ---
        ("thik", "theek"),
        ("shukriyaa", "shukriya"),
        ("dhanyavad", "dhanyawad"),
        ("pareshann", "pareshan"),
        ("nameste", "namaste"),
        ("bhaiya", "bhaiya"),
        ("chutiyapa", "chutiyapa"),
        ("jugaad", "jugaad"),
        ("biriyani", "biryani"),
        ("diwali", "diwali"),

        // --- 3. Academic & Abstract Concepts ---
        ("ambigous", "ambiguous"),
        ("anomoly", "anomaly"),
        ("assumtion", "assumption"),
        ("capabilty", "capability"),
        ("coincedence", "coincidence"),
        ("correlaton", "correlation"),
        ("efficiancy", "efficiency"),
        ("hypothetcal", "hypothetical"),
        ("implicaton", "implication"),
        ("methodolgy", "methodology"),

        // --- 4. Legal & Formal Vocabulary ---
        ("arbitraton", "arbitration"),
        ("affiadavit", "affidavit"),
        ("defendent", "defendant"),
        ("enforcemnt", "enforcement"),
        ("jurisdicton", "jurisdiction"),
        ("negligance", "negligence"),
        ("prosecuton", "prosecution"),
        ("regulaton", "regulation"),
        ("statutary", "statutory"),
        ("testimonny", "testimony"),

        // --- 5. Tech, Math & Computing ---
        ("algoritm", "algorithm"),
        ("architecure", "architecture"),
        ("asyncronous", "asynchronous"),
        ("authentification", "authentication"),
        ("concurancy", "concurrency"),
        ("dependancy", "dependency"),
        ("implemetation", "implementation"),
        ("infrastrucutre", "infrastructure"),
        ("paramatre", "parameter"),
        ("performace", "performance"),

        // --- 6. UNSEEN / DYNAMIC ALTERATIONS (Proving SymSpell distance + JamSpell LM) ---
        ("recieeve", "receive"),
        ("buhut", "bahut"),
        ("algrithm", "algorithm"),
        ("hallo", "hello"),
        ("woudl", "would"),
        ("coudl", "could"),
        ("shoudl", "should"),
        ("becuase", "because"),
        ("welcom", "welcome"),
        ("pleas", "please"),
    ];

    println!("\n==================================================================");
    println!(" Running 60-Word Mixed Autocorrect Verification Suite");
    println!("==================================================================\n");

    let mut passed = 0;
    for (typo, expected) in &test_cases {
        let result = engine.check(typo, "this is a ");
        assert!(
            result.is_some(),
            "FAIL: Word '{}' was not recognized by any layer!",
            typo
        );
        let corr = result.unwrap();
        assert_eq!(
            corr.corrected, *expected,
            "FAIL: Input '{}' expected '{}', but got '{}'",
            typo, expected, corr.corrected
        );
        passed += 1;
    }

    println!("SUCCESS: All {} mixed test cases passed perfectly!\n", passed);
}
