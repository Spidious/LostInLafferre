# Use the latest LTS version of Node.js
FROM rust:latest
 
# Set the working directory inside the container
WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["/app/target/release/LostInLafferre"]
