CREATE TABLE weids (
                       id INTEGER PRIMARY KEY AUTOINCREMENT,
                       addr TEXT NOT NULL,
                       chain_id INTEGER,
                       created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                       updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);