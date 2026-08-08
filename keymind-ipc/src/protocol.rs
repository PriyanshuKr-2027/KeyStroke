use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VariableDto {
    pub key: String,
    pub var_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub use_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DailyStatsDto {
    pub words_typed: i64,
    pub corrections_made: i64,
    pub variables_used: i64,
    pub ai_requests: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LearnedPhraseDto {
    pub id: String,
    pub phrase: String,
    pub frequency: i64,
    pub pinned: bool,
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum IpcRequest {
    STATUS_REQUEST,
    VARIABLE_LIST,
    VARIABLE_UPSERT {
        variable: VariableDto,
    },
    VARIABLE_DELETE {
        key: String,
    },
    STATS_REQUEST,
    LEARNED_PHRASES,
    PIN_PHRASE {
        id: String,
    },
    DELETE_PHRASE {
        id: String,
    },
    TOGGLE_LEARNING {
        enabled: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum IpcResponse {
    STATUS_RESPONSE {
        engine: String,
        ai: String,
        grammar: String,
    },
    VARIABLE_LIST_RESPONSE {
        variables: Vec<VariableDto>,
    },
    STATS_RESPONSE {
        today: DailyStatsDto,
    },
    LEARNED_PHRASES_RESPONSE {
        phrases: Vec<LearnedPhraseDto>,
    },
    OK,
    ERROR {
        message: String,
    },
}
