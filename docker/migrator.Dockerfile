FROM rust:1.62.1-slim-bullseye
COPY ./migrations ./migrations
COPY ./scripts/init_db_cluster.sh ./

RUN chmod +x init_db_cluster.sh

ENTRYPOINT ["./entry_point.sh"]
