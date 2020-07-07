# Dreamin GraphQL
Maidreamin API written in Rust as GraphQL Server.

## Docker
Build Docker and run.
```bash
docker build -t rust-dremain-graphql

docker run -d -p 8080:8080 rust-dreamin-graphql
```

Then visit running container at: [http://127.0.0.1:8080/graphiql](http://127.0.0.1:8080/graphiql)