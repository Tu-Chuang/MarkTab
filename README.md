# MARKTAB

MARKTAB是一个模块化的个人工具集合平台，使用Rust语言开发。

## 部署指南 (小白版)

这是一个面向小白的详细部署教程，按照以下步骤即可完成部署。

### 一、服务器准备

1. 配置要求:
   - CPU: 1核心以上
   - 内存: 2GB以上
   - 硬盘: 20GB以上
   - 系统: Ubuntu 20.04/22.04 LTS (推荐)

2. 域名配置(可选):
   - 购买域名并解析到服务器IP
   - 申请SSL证书(推荐Let's Encrypt)

### 二、安装必要软件

1. 安装Docker:
bash
一键安装Docker
curl -fsSL https://get.docker.com | sh
启动Docker
sudo systemctl start docker
sudo systemctl enable docker

2. 安装Git:
bash
sudo apt update
sudo apt install git -y

### 三、部署步骤

1. 下载项目:
bash
克隆项目
git clone https://github.com/Tu-Chuang/MarkTab.git
cd MarkTab

2. 配置环境:
bash
复制配置文件
cp .env.example .env
修改配置文件
nano .env
需要修改的重要配置:
DATABASE_URL=mysql://MARKTAB:password@db/MARKTAB
JWT_SECRET=your_jwt_secret_here # 改成随机字符串
UPLOAD_DIR=/opt/MARKTAB/uploads
BACKUP_DIR=/opt/MARKTAB/backups

3. 启动服务:
bash
启动所有服务
docker-compose up -d
查看运行状态
docker-compose ps
查看日志
docker-compose logs -f

### 四、检查部署

1. 访问测试:
   - 打开浏览器访问: http://你的服务器IP:8080
   - 如果能看到登录页面就说明部署成功

2. 检查服务:
bash
查看所有容器状态
docker ps
查看服务日志
docker-compose logs -f

### 五、常见问题

1. 端口被占用:
yaml
修改docker-compose.yml中的端口
ports:
"8081:8080" # 改成其他端口

2. 数据库连接失败:
   - 检查.env中的数据库配置是否正确
   - 确保MySQL容器正常运行

3. 上传目录权限问题:
bash
修改目录权限
sudo chown -R 1000:1000 uploads/
sudo chmod -R 755 uploads/

4. 服务无法启动:
bash
查看详细日志
docker-compose logs -f app
重启服务
docker-compose restart

### 六、维护指南

1. 更新服务:
bash
拉取最新代码
git pull
重新构建并启动
docker-compose down
docker-compose up -d --build

2. 备份数据:
bash
备份数据库
docker exec MARKTAB-db mysqldump -u MARKTAB -p MARKTAB > backup.sql
备份上传文件
cp -r uploads/ backups/uploads_$(date +%Y%m%d)

3. 查看日志:
bash
实时查看日志
docker-compose logs -f
查看指定服务日志
docker-compose logs -f app
docker-compose logs -f db

### 七、安全建议

1. 修改默认端口
2. 使用强密码
3. 配置防火墙:
bash
安装防火墙
sudo apt install ufw
只开放必要端口
sudo ufw allow ssh
sudo ufw allow http
sudo ufw allow https
sudo ufw enable

4. 启用HTTPS:
bash
安装certbot
sudo apt install certbot python3-certbot-nginx
申请证书
sudo certbot --nginx -d your-domain.com

5. 定期更新系统:
bash
更新系统
sudo apt update
sudo apt upgrade
更新Docker镜像
docker-compose pull
docker-compose up -d


## 开发文档

### 技术栈
- Rust 1.70+
- Actix-web 4.0
- SQLx
- MySQL 8.0
- Docker & Docker Compose

### 项目结构
/
├── src/
│ ├── controllers/ # HTTP请求处理
│ ├── models/ # 数据模型
│ ├── services/ # 业务逻辑
│ ├── plugins/ # 插件系统
│ ├── middleware/ # 中间件
│ ├── utils/ # 工具函数
│ ├── config/ # 配置
│ └── error/ # 错误处理
├── migrations/ # 数据库迁移
├── tests/ # 测试文件
└── docs/ # 文档

### API文档
- Swagger UI: http://localhost:8080/swagger-ui/
- OpenAPI JSON: http://localhost:8080/api-docs/openapi.json

### 八、获取帮助

如果遇到问题:
- 1. 查看项目 [Wiki](https://github.com/Tu-Chuang/MarkTab/wiki)
- 2. 提交 [Issue](https://github.com/Tu-Chuang/MarkTab/issues)

