CREATE TABLE IF NOT EXISTS palette_settings (
    id INTEGER PRIMARY KEY DEFAULT 1,
    hotkey TEXT DEFAULT 'Ctrl+Alt+Space',
    context_window_chars INTEGER DEFAULT 500,
    model_preference TEXT DEFAULT 'groq'
);

INSERT OR IGNORE INTO palette_settings (id, hotkey, context_window_chars, model_preference)
VALUES (1, 'Ctrl+Alt+Space', 500, 'groq');
