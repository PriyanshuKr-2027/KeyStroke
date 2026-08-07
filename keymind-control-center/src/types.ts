export type EngineStatusType = "running" | "stopped" | "error";
export type AiStatusType = "connected" | "offline";
export type GrammarStatusType = "ready" | "starting" | "error";

export interface EngineStatus {
  engine: EngineStatusType;
  ai: AiStatusType;
  grammar: GrammarStatusType;
}

export interface DailyStats {
  words_typed: number;
  corrections_made: number;
  variables_used: number;
  ai_requests: number;
}

export interface AutocorrectFeedItem {
  id: string;
  original: string;
  corrected: string;
  time_ago: string;
}

export type VariableType = "static" | "dynamic" | "ai";

export interface Variable {
  key: string;
  var_type: VariableType;
  value?: string;
  ai_prompt?: string;
  description?: string;
  use_count: number;
  last_used?: string;
}

export type GrammarMode = "Aggressive" | "Suggestions Only";

export interface GrammarStatus {
  enabled: boolean;
  mode: GrammarMode;
  language: string;
}

export interface GrammarFix {
  id: string;
  original: string;
  fixed: string;
  rule_id: string;
  category: "TYPOS" | "GRAMMAR" | "STYLE" | "PUNCTUATION";
  timestamp: string;
}

export interface GrammarIssueInline {
  offset: number;
  length: number;
  message: string;
  replacements: string[];
  rule_id: string;
  category: string;
}

export interface ShortcutBinding {
  id: string;
  name: string;
  action: string;
  shortcut: string;
}

export interface AppSettings {
  app_bundle_id: string;
  app_name: string;
  autocorrect_enabled: boolean;
  grammar_enabled: boolean;
  ai_copilot_enabled: boolean;
  is_blocked: boolean;
}

export interface CopilotAction {
  id: string;
  icon: string;
  name: string;
  description: string;
}

export interface CopilotStreamPayload {
  delta: string;
}

export interface CopilotDonePayload {
  final_text: string;
}

export interface LearnedPhraseItem {
  id: string;
  phrase: string;
  frequency: number;
  is_pinned: boolean;
  app_id?: string;
  date_added?: string;
}

export interface PersonalWordItem {
  id: string;
  word: string;
  date_added: string;
}

export interface UserProfile {
  first_name: string;
  last_name: string;
  email: string;
  date_of_birth?: string;
}

export interface ActivePrediction {
  candidate_word: string;
  full_suggestions: string[];
  confidence: number;
  context: string;
}
