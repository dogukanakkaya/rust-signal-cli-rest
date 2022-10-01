## Boot Signal CLI Container
Pull from Docker Hub (https://hub.docker.com/repository/docker/dogukanakkaya/signal-cli)
```
docker pull dogukanakkaya/signal-cli
docker run --name signal-cli -dit dogukanakkaya/signal-cli
```

<br>

## Start the Server
Use cargo to start the web server
```
cargo run

// or if you installed `cargo-watch` you can start the server in watch mode if you like
cargo watch -x 'run'
```