# Builder
FROM rust:1.44 as builder

COPY . .

RUN cargo build --release

# Run Tine
FROM rust:1.44-slim-stretch

COPY --from=builder /target/release/dreamin-graphql .

EXPOSE 8080

ENTRYPOINT ["/dreamin-graphql"]