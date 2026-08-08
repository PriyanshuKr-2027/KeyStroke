use keymind_autocorrect::AutocorrectEngine;

#[tokio::main]
async fn main() {
    println!("==================================================================");
    println!(" KeyMind Autocorrect & Engine Real-Time Test");
    println!("==================================================================\n");

    let tmp_path = std::env::temp_dir().join("interactive_test_autocorrect.json");
    let engine = AutocorrectEngine::new(tmp_path);
    engine
        .initialize()
        .await
        .expect("Failed to initialize Autocorrect engine");

    let test_samples = vec![
        ("teh", "in ", "SymSpell Typo Correction"),
        ("recieve", "did you ", "SymSpell Typo Correction"),
        ("their", "going over ", "Homophone Contextual Resolution"),
        ("then", "more ", "Homophone Contextual Resolution"),
        ("you're", "is ", "Homophone Contextual Resolution"),
        ("hello", "say ", "Valid Word (No Change)"),
    ];

    println!(
        "{:<12} | {:<12} | {:<14} | {:<10} | {:<30}",
        "Typed Word", "Context", "Output Fix", "Confidence", "Rule / Layer Applied"
    );
    println!("--------------------------------------------------------------------------------------------------");

    for (word, context, label) in test_samples {
        if let Some(corr) = engine.check(word, context) {
            println!(
                "{:<12} | {:<12} | {:<14} | {:<10.0}% | {:<30}",
                corr.original,
                context,
                corr.corrected,
                corr.confidence * 100.0,
                label
            );
        } else {
            println!(
                "{:<12} | {:<12} | {:<14} | {:<10} | {:<30}",
                word, context, "[No change]", "N/A", label
            );
        }
    }

    println!("\n==================================================================");
}
