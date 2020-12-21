CREATE TABLE IF NOT EXISTS schema_migrations(
    id INTEGER PRIMARY KEY NOT NULL,
    version VARCHAR(17) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_schema_migrations_version ON schema_migrations(version);