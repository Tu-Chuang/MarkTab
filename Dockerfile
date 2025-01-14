# 构建阶段
FROM rust:1.70 as builder

WORKDIR /usr/src/MARKTAB
COPY . .

RUN cargo build --release

# 运行阶段
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /opt/MARKTAB

COPY --from=builder /usr/src/MARKTAB/target/release/MARKTAB /opt/MARKTAB/
COPY --from=builder /usr/src/MARKTAB/migrations /opt/MARKTAB/migrations
COPY --from=builder /usr/src/MARKTAB/.env.example /opt/MARKTAB/.env

RUN useradd -r -s /bin/false MARKTAB \
    && chown -R MARKTAB:MARKTAB /opt/MARKTAB

USER MARKTAB

EXPOSE 8080

CMD ["./MARKTAB"] 