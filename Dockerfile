# Use the latest LTS version of Node.js
FROM rust:latest

RUN apt-get update && apt-get install -y \
    net-tools \
    libssl-dev \
    pkg-config \
    build-essential \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /app

COPY . .

WORKDIR /app/main_func

RUN cargo build --release

CMD ["./target/release/LostInLafferre", "0.0.0.0", "8080", "nodes_edges.json"]