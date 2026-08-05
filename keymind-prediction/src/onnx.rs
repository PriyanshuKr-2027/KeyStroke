use std::path::PathBuf;

pub struct OnnxPredictor {
    _model_path: PathBuf,
}

use std::collections::HashMap;
use once_cell::sync::Lazy;

static FALLBACK_PREDICTIONS: Lazy<HashMap<&str, Vec<&str>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("the", vec!["best", "most", "first", "new", "same"]);
    m.insert("i", vec!["think", "would", "have", "am", "will"]);
    m.insert("to", vec!["be", "do", "get", "make", "have"]);
    m.insert("in", vec!["the", "order", "this", "a", "my"]);
    m.insert("is", vec!["a", "the", "not", "an", "that"]);
    m.insert("for", vec!["the", "a", "your", "this", "my"]);
    m.insert("it", vec!["is", "was", "would", "will", "can"]);
    m.insert("that", vec!["the", "is", "I", "we", "you"]);
    m.insert("you", vec!["can", "are", "have", "will", "would"]);
    m.insert("we", vec!["can", "are", "have", "will", "should"]);
    m.insert("this", vec!["is", "was", "will", "would", "should"]);
    m.insert("with", vec!["the", "a", "your", "our", "my"]);
    m.insert("have", vec!["a", "been", "to", "the", "not"]);
    m.insert("will", vec!["be", "have", "not", "need", "also"]);
    m.insert("are", vec!["you", "the", "not", "we", "there"]);
    m.insert("can", vec!["be", "you", "I", "we", "help"]);
    m.insert("was", vec!["a", "the", "not", "very", "also"]);
    m.insert("my", vec!["name", "team", "first", "own", "new"]);
    m.insert("would", vec!["like", "be", "have", "love", "appreciate"]);
    m.insert("please", vec!["let", "find", "note", "review", "check"]);
    m.insert("thank", vec!["you", "everyone", "them", "her", "him"]);
    m.insert("looking", vec!["forward", "for", "into", "at", "good"]);
    m.insert("how", vec!["are", "do", "can", "about", "much"]);
    m.insert("best", vec!["regards", "wishes", "practices", "way", "option"]);
    m.insert("good", vec!["morning", "afternoon", "evening", "luck", "news"]);
    m.insert("financial", vec!["report", "analysis", "data", "performance", "results"]);
    m.insert("project", vec!["management", "plan", "timeline", "update", "scope"]);
    m.insert("meeting", vec!["tomorrow", "today", "agenda", "notes", "scheduled"]);
    m.insert("need", vec!["to", "a", "your", "more", "help"]);
    m.insert("should", vec!["be", "have", "we", "I", "not"]);
    m
});

impl OnnxPredictor {
    pub fn new(model_path: PathBuf) -> Self {
        // TODO: Integrate `ort` crate for real GPT-2/ONNX model inference
        Self { _model_path: model_path }
    }

    pub async fn predict_next_words(&self, context: &str) -> Vec<String> {
        let last = context.split_whitespace().last().unwrap_or("").to_lowercase();
        FALLBACK_PREDICTIONS.get(last.as_str())
            .map(|v| v.iter().map(|s| s.to_string()).collect())
            .unwrap_or_else(|| vec!["the".into(), "and".into(), "to".into(), "is".into(), "a".into()])
    }
}
