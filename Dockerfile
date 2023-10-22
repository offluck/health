FROM --platform=linux/amd64 rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM --platform=linux/amd64 debian:latest AS runner
COPY --from=builder /app/target/release/health .
EXPOSE 8000
CMD ["./health"]
