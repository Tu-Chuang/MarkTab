CREATE TABLE IF NOT EXISTS system_metrics (
    id INT PRIMARY KEY AUTO_INCREMENT,
    cpu_usage FLOAT NOT NULL,
    memory_total BIGINT NOT NULL,
    memory_used BIGINT NOT NULL,
    disk_total BIGINT NOT NULL,
    disk_used BIGINT NOT NULL,
    load_avg_1 FLOAT NOT NULL,
    load_avg_5 FLOAT NOT NULL,
    load_avg_15 FLOAT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS api_metrics (
    id INT PRIMARY KEY AUTO_INCREMENT,
    path VARCHAR(255) NOT NULL,
    method VARCHAR(10) NOT NULL,
    status_code INT NOT NULL,
    response_time INT NOT NULL, -- 毫秒
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_path_method (path, method)
); 