FROM rustlang/rust:nightly AS builder
WORKDIR /usr/src/go
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
WORKDIR /usr/local/bin/go
COPY extensions extensions
COPY initialisations initialisations
COPY migrations migrations
COPY models models
COPY --from=builder /usr/local/cargo/bin/go .
RUN apt update && apt install -y

EXPOSE 8080
CMD ["./go"]