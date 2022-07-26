FROM rust:1.62.1-slim-bullseye
WORKDIR /prj

RUN apt update -y \
    &&  apt install -y --no-install-recommends \
    libssl-dev \
    pkg-config \
    openssl \
    && cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY ./migrations ./migrations
COPY ./scripts/init_db_cluster.sh ./

RUN chmod +x init_db_cluster.sh

ENTRYPOINT ["./init_db_cluster.sh"]
