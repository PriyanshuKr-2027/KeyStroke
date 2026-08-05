use keymind_autocorrect::{AutocorrectEngine, Correction};
use sqlx::sqlite::SqlitePoolOptions;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

async fn setup_test_engine() -> AutocorrectEngine {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    let engine = AutocorrectEngine::new(Arc::new(pool));
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
