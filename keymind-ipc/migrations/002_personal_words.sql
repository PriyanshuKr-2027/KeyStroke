CREATE TABLE IF NOT EXISTS personal_words (
  word TEXT PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS corrections (
  from_word TEXT NOT NULL,
  to_word TEXT NOT NULL,
  count INTEGER DEFAULT 1,
  last_seen INTEGER DEFAULT (unixepoch()),
  PRIMARY KEY (from_word, to_word)
);
