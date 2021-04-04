CREATE TABLE ops_cron_tasks (
    id BIGSERIAL PRIMARY KEY,
    schedule VARCHAR(255) NOT NULL,
    "user" VARCHAR(16),
    command VARCHAR(255) NOT NULL,    
    version BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL
);

CREATE INDEX ops_cron_tasks_user ON ops_cron_tasks("user");