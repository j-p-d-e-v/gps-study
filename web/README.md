# GPS Simulator Web Server

This is the web server for the GPS Simulator. It leverages Actix to expose APIs over HTTP, utilizes WebSockets to stream data to the frontend, and integrates SurrealDB for data storage and management.

## Build
```sh
cargo build --release
```

## Testing
```sh
cargo test -- --nocapture
```

## Command
```sh
cargo run -- --config-path=../config.toml
```
