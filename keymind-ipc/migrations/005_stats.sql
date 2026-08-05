CREATE TABLE IF NOT EXISTS daily_stats (
  date TEXT PRIMARY KEY,
  words_typed INTEGER DEFAULT 0,
  corrections_made INTEGER DEFAULT 0,
  variables_used INTEGER DEFAULT 0,
  ai_requests INTEGER DEFAULT 0
);
