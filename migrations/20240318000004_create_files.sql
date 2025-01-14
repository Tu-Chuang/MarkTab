CREATE TABLE IF NOT EXISTS MARKTAB_files (
    id INT PRIMARY KEY AUTO_INCREMENT,
    user_id INT NOT NULL,
    filename VARCHAR(255) NOT NULL,
    mime_type VARCHAR(255) NOT NULL,
    size BIGINT NOT NULL,
    hash VARCHAR(32) NOT NULL,
    path VARCHAR(1024) NOT NULL,
    status TINYINT NOT NULL DEFAULT 1,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES MARKTAB_users(id) ON DELETE CASCADE,
    KEY `idx_user_created` (user_id, created_at),
    KEY `idx_hash` (hash),
    KEY `idx_status` (status)
); 