FROM rust:1.62.1-slim-bullseye
WORKDIR /prj

COPY ./migrations ./migrations
COPY ./scripts/init_db_cluster.sh ./
RUN cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres
RUN chmod +x init_db_cluster.sh

ENTRYPOINT ["./init_db_cluster.sh"]
