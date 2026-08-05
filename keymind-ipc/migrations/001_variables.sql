CREATE TABLE IF NOT EXISTS variables (
  key TEXT PRIMARY KEY,
  var_type TEXT NOT NULL CHECK(var_type IN ('static','dynamic','ai')),
  value TEXT,
  ai_prompt TEXT,
  description TEXT,
  use_count INTEGER DEFAULT 0,
  created_at INTEGER DEFAULT (unixepoch()),
  updated_at INTEGER DEFAULT (unixepoch())
);
