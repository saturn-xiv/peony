CREATE TABLE ops_crawler_logs (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    url VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX ops_crawler_logs_name ON ops_crawler_logs(name);
CREATE INDEX ops_crawler_logs_url ON ops_crawler_logs(url);