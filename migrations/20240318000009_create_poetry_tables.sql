CREATE TABLE IF NOT EXISTS plugin_poetry (
    id INT PRIMARY KEY AUTO_INCREMENT,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    author VARCHAR(255) NOT NULL,
    dynasty VARCHAR(255) NOT NULL,
    category VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    KEY `idx_title` (title),
    KEY `idx_author` (author),
    KEY `idx_dynasty` (dynasty),
    KEY `idx_category` (category)
);

CREATE TABLE IF NOT EXISTS plugin_poetry_favorites (
    id INT PRIMARY KEY AUTO_INCREMENT,
    user_id INT NOT NULL,
    poetry_id INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES MARKTAB_users(id) ON DELETE CASCADE,
    FOREIGN KEY (poetry_id) REFERENCES plugin_poetry(id) ON DELETE CASCADE,
    UNIQUE KEY `unique_favorite` (user_id, poetry_id),
    KEY `idx_user_created` (user_id, created_at)
); 