CREATE TABLE IF NOT EXISTS plugin_weather_cache (
    id INT PRIMARY KEY AUTO_INCREMENT,
    location VARCHAR(255) NOT NULL,
    data TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    KEY `idx_location` (location),
    KEY `idx_created_at` (created_at)
); 