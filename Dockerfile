FROM rust:slim
COPY ./target/release/trading-api ./target/release/trading-api

ARG app_version
ARG app_compilation_date
ENV APP_VERSION=${app_version}
ENV APP_COMPILATION_DATE=${app_compilation_date}

ENTRYPOINT ["./target/release/trading-api"]