FROM rust:1.82 AS base
RUN cargo install cargo-chef --locked --version ^0.1

FROM base AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM base AS builder

WORKDIR /app

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc

WORKDIR /app

COPY --from=builder /app/target/release/scheduling-iam ./

CMD ["./scheduling-iam"]

