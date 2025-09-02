# CLIC x S4S - 2024

## Dev

### Dependencies

You will need the following:

- [docker](https://www.docker.com/get-started/)
- A rust toolchain [(most likely cargo)](https://github.com/rust-lang/cargo)
- An operating system supporting UNIX sockets.

### Run locally

Run `docker build -t s4s-2024-runner:dev ./runner`

Run `docker compose up`

You can access the website on <http://localhost/s4s>.

Note: due to the website being http when run locally, there are some issues with cookies. Each time you restart or refresh the website, you must delete the cookie and go to <http://localhost/s4s/login>.

## Deploy

The CI builds all 3 necessary images (frontend, api, runner), that are then pulled when the service is deployed on the server. Note that there is no webhook on this repo, the service must be deployed by hand once the images are built and pushed by the CI.
