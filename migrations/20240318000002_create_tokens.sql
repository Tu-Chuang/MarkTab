CREATE TABLE IF NOT EXISTS MARKTAB_tokens (
    id INT PRIMARY KEY AUTO_INCREMENT,
    user_id INT NOT NULL,
    token VARCHAR(255) NOT NULL,
    user_agent VARCHAR(255),
    ip_address VARCHAR(45),
    status TINYINT NOT NULL DEFAULT 1,
    expired_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES MARKTAB_users(id) ON DELETE CASCADE,
    UNIQUE KEY `unique_token` (token),
    KEY `idx_user_status` (user_id, status),
    KEY `idx_expired_at` (expired_at)
); 