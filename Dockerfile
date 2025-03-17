# Use the latest LTS version of Node.js
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

COPY . .

WORKDIR /app/main_func

RUN cargo build --release

CMD ["./target/release/LostInLafferre", "127.0.0.1", "8080", "nodes_edges.json"]