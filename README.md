# m-league-data-service

backend for m-league-data

## run

- generate sqlite db file with [m-league-data-fetcher](https://github.com/Debonex/m-league-data-fetcher)

- modify `url` in `Rocket.toml` to point to the generated `.db` file

- `cargo run`

## build docker image

- generate sqlite db file with [m-league-data-fetcher](https://github.com/Debonex/m-league-data-fetcher)

- modify `url` in `Rocket.toml` to point to the generated `.db` file

- modify database path in dockerfile `COPY ./database ./database` the first path is where the `.db` locate, and the second path should be the same as database directory in `Rocket.toml`

- `docker build -t {image-name} .`

## run with docker image

- `docker pull debonex/m-league-data-service:latest`
