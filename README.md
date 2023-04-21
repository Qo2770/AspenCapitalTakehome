# AspenCapitalTakehome
A simple RESTful API for simulating the card game War, built in Rust using Rocket

[![Build Status](https://app.travis-ci.com/Qo2770/AspenCapitalTakehome.svg?token=BDa1A6CDAf7MpzYvSe3o&branch=main)](https://app.travis-ci.com/Qo2770/AspenCapitalTakehome)

## Usage

1. Ensure that you have [Rust](https://www.rust-lang.org) and [Docker](http://docker.com) installed.
2. Run `docker run -p 5432:5432 --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -d postgres` in order to start a local Postgres database in Docker
3. Enter the `api` folder and run `cargo install sqlx-cli --no-default-features --features rustls,postgres` to install the sqlx cli
4. Run `sqlx migrate run --database-url postgres://postgres:mysecretpassword@localhost:5432/postgres` inside `api` to initialize the database
5. Run `cargo run` to start the API
6. The API should now be running at http://localhost:8000
7. Navigate to http://localhost:8000/start to run a game, and to http://localhost:8000/scores to view the scores

## Implementation

The API was implemented in Rust using the Rocket library. Code related to
serving the endpoints and connecting to the database can be found in `main.rs`,
while the game implementation rests in `war.rs`. The game functions are also
thoroughly covered with unit testing. 

