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

<br>

## Endpoints
- [GET] `/api/register/{phone}`
- [GET] `/api/register/captcha/{phone}?token={token}`
  - To get captcha token, see readme of: https://github.com/dogukanakkaya/signal-cli-docker
- [GET] `/api/verify/{phone}?code={code}&pin={pin}`
  - Code will be sent to your phone after register
- [GET] `/api/link/{name}`
  - This endpoint will show you a QRCode to link your device
  - Open Signal App, go to "Settings", go to "Linked Devices", click the plus icon on the right bottom and scan the QRCode
- [GET] `/api/trust-unsafe/{phone}`
- [POST] `/api/send/{phone}`
  - Payload: 
    ```json
    {
        "recipient": "+90545xxxxxxx",
        "message": "My Signal message from API"
    }
    ```
