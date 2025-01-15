#!/bin/bash
set -e

# 等待数据库就绪
echo "Waiting for database to be ready..."
./wait-for-it.sh db:3306 -t 60

# 运行数据库迁移
echo "Running database migrations..."
sqlx database create
sqlx migrate run

# 创建必要的目录
mkdir -p /opt/marktab/uploads
mkdir -p /opt/marktab/backups

# 启动应用
echo "Starting MARKTAB..."
./marktab 