# testament-api
[![Build Status](https://travis-ci.com/himanoa/testament-api.svg?branch=master)](https://travis-ci.com/himanoa/testament-api) [![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

My blog API server written in Rust.

## Dependencies middleware

- Docker

## DB Setup

1. Execute `cp .env.example .env`
2. Write your database url to .env file.
3. Execute `cargo make setup-db --env-file=.env`
3. Execute `cargo run`

## License
MIT
