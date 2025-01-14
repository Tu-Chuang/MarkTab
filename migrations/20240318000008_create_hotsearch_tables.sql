CREATE TABLE IF NOT EXISTS plugin_hotsearch (
    id INT PRIMARY KEY AUTO_INCREMENT,
    platform VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    url VARCHAR(1024) NOT NULL,
    rank INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    KEY `idx_platform_rank` (platform, rank)
); 