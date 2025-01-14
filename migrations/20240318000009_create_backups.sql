CREATE TABLE IF NOT EXISTS system_backups (
    id INT PRIMARY KEY AUTO_INCREMENT,
    filename VARCHAR(255) NOT NULL,
    size BIGINT NOT NULL,
    hash VARCHAR(64) NOT NULL,
    backup_type VARCHAR(20) NOT NULL, -- 'full' or 'incremental'
    status VARCHAR(20) NOT NULL, -- 'pending', 'running', 'completed', 'failed'
    error_message TEXT,
    started_at TIMESTAMP NULL,
    completed_at TIMESTAMP NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
); 