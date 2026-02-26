-- Initial schema for Guardr

CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    name TEXT,
    subscription_tier TEXT NOT NULL DEFAULT 'free',
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    last_login TEXT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE IF NOT EXISTS api_keys (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL REFERENCES users(id),
    name TEXT NOT NULL,
    key_hash TEXT NOT NULL,
    key_prefix TEXT NOT NULL,
    last_used TEXT,
    created_at TEXT NOT NULL,
    expires_at TEXT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TABLE IF NOT EXISTS usage_tracking (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL REFERENCES users(id),
    month_year TEXT NOT NULL,
    endpoint TEXT NOT NULL,
    requests_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(user_id, month_year, endpoint)
);

CREATE TABLE IF NOT EXISTS security_reports (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL REFERENCES users(id),
    report_type TEXT NOT NULL,
    input_data_hash TEXT NOT NULL,
    results TEXT NOT NULL,
    risk_score INTEGER,
    created_at TEXT NOT NULL,
    expires_at TEXT
);

CREATE TABLE IF NOT EXISTS breach_data (
    id TEXT PRIMARY KEY NOT NULL,
    email_hash TEXT NOT NULL,
    password_hash TEXT,
    source_name TEXT NOT NULL,
    breach_date TEXT NOT NULL,
    data_types TEXT NOT NULL,
    severity TEXT NOT NULL,
    verified BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_api_keys_user_id ON api_keys(user_id);
CREATE INDEX IF NOT EXISTS idx_api_keys_key_hash ON api_keys(key_hash);
CREATE INDEX IF NOT EXISTS idx_usage_tracking_user_month ON usage_tracking(user_id, month_year);
CREATE INDEX IF NOT EXISTS idx_security_reports_user_id ON security_reports(user_id);
CREATE INDEX IF NOT EXISTS idx_breach_data_email_hash ON breach_data(email_hash);
CREATE INDEX IF NOT EXISTS idx_breach_data_password_hash ON breach_data(password_hash);
