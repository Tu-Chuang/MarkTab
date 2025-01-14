CREATE TABLE IF NOT EXISTS plugin_todo_folders (
    id INT PRIMARY KEY AUTO_INCREMENT,
    user_id INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES MARKTAB_users(id) ON DELETE CASCADE,
    KEY `idx_user_created` (user_id, created_at)
);

CREATE TABLE IF NOT EXISTS plugin_todos (
    id INT PRIMARY KEY AUTO_INCREMENT,
    user_id INT NOT NULL,
    folder_id INT NOT NULL,
    content TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES MARKTAB_users(id) ON DELETE CASCADE,
    FOREIGN KEY (folder_id) REFERENCES plugin_todo_folders(id) ON DELETE CASCADE,
    KEY `idx_user_folder` (user_id, folder_id),
    KEY `idx_completed` (completed)
); 