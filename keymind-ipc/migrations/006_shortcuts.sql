CREATE TABLE IF NOT EXISTS shortcuts (
  id TEXT PRIMARY KEY,
  label TEXT NOT NULL,
  default_binding TEXT NOT NULL,
  current_binding TEXT NOT NULL
);

INSERT OR IGNORE INTO shortcuts (id, label, default_binding, current_binding) VALUES
('copilot_palette', 'AI Copilot Palette', 'Ctrl+Space', 'Ctrl+Space'),
('grammar_fix', 'Grammar Fix Selection', 'Ctrl+Shift+G', 'Ctrl+Shift+G'),
('copilot_professional', 'Copilot Professional', 'Ctrl+Shift+P', 'Ctrl+Shift+P'),
('copilot_summarize', 'Copilot Summarize', 'Ctrl+Shift+S', 'Ctrl+Shift+S');
