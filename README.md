# m-league-data-service

backend for m-league-data

## run (Development)

- generate sqlite db file with [m-league-data-fetcher](https://github.com/Debonex/m-league-data-fetcher)

- modify `DATABASE_URL` in `.env` to point to the generated `.db` file

- `cargo run`

## run with docker image

- `docker pull debonex/m-league-data-service:latest`

- `docker run --rm -v ${HOST_DATABASE_PATH}:/workspace/database/mleague.db -p 7878:7878 debonex/m-league-data-service:latest`

