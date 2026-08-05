CREATE TABLE IF NOT EXISTS grammar_cache (
  sentence_hash TEXT PRIMARY KEY,
  issues_json TEXT NOT NULL,
  cached_at INTEGER DEFAULT (unixepoch())
);
