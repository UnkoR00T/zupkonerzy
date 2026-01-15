FROM rust:slim-bookworm as chef
RUN cargo install cargo-chef
WORKDIR /usr/src/app

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
RUN apt-get update && apt-get install -y \
  pkg-config \
  libssl-dev \
  gcc \
  && rm -rf /var/lib/apt/lists/*

COPY --from=planner /usr/src/app/recipe.json recipe.json
# Build dependencies - this is the cached layer
RUN cargo chef cook --release --recipe-path recipe.json

# Copy source and build
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update \
  && apt-get install -y --no-install-recommends libssl3 ca-certificates \
  && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/wp-server .
EXPOSE 8000
EXPOSE 8080
CMD ["./wp-server"]
