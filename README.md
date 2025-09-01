# CLIC x S4S - 2024

## Dev

### Dependencies

You will need the following:

- [docker](https://www.docker.com/get-started/)
- A rust toolchain [(most likely cargo)](https://github.com/rust-lang/cargo)
- An operating system supporting UNIX sockets.

## Deploy

Run `docker build -t s4s_runner ./backend/deps`

Run `docker compose up`

You can access the website on <http://localhost/s4s>.
