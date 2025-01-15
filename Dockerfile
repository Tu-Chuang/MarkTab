# 构建阶段
FROM rust:1.75 as builder

WORKDIR /usr/src/marktab
COPY . .

RUN cargo build --release

# 运行阶段
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /opt/marktab

COPY --from=builder /usr/src/marktab/target/release/marktab ./
COPY --from=builder /usr/src/marktab/migrations ./migrations
COPY --from=builder /usr/src/marktab/scripts/start.sh ./
COPY --from=builder /usr/src/marktab/scripts/wait-for-it.sh ./

RUN chmod +x start.sh wait-for-it.sh

ENV RUST_LOG=info

EXPOSE 8080

CMD ["./start.sh"] 