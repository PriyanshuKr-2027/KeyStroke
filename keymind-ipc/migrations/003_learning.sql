CREATE TABLE IF NOT EXISTS phrase_candidates (
  phrase TEXT PRIMARY KEY,
  frequency INTEGER DEFAULT 1,
  first_seen INTEGER DEFAULT (unixepoch()),
  last_seen INTEGER DEFAULT (unixepoch()),
  app_context TEXT,
  promoted BOOLEAN DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS learned_memory (
  id TEXT PRIMARY KEY,
  phrase TEXT NOT NULL,
  frequency INTEGER,
  learned_at INTEGER DEFAULT (unixepoch()),
  pinned BOOLEAN DEFAULT FALSE,
  ignored BOOLEAN DEFAULT FALSE,
  category TEXT
);

CREATE TABLE IF NOT EXISTS app_blocklist (
  app_bundle_id TEXT PRIMARY KEY,
  added_at INTEGER DEFAULT (unixepoch())
);
