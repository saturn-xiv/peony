CREATE TABLE crawlers_logs(
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(32) NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_crawlers_logs_name ON crawlers_logs(name);
