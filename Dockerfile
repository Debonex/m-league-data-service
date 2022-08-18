FROM rust:1.59.0 as builder
WORKDIR /workspace
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
WORKDIR /workspace
# today, suddenly I can't directly copy Rocket.toml and .env from the host, why?
COPY --from=builder /workspace/Rocket.toml .
COPY --from=builder /workspace/.env .
COPY --from=builder /usr/local/cargo/bin/m-league-data-service /usr/local/bin/m-league-data-service
ENV ROCKET_ADDRESS=0.0.0.0
CMD ["m-league-data-service"]

EXPOSE 7878