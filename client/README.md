# GPS Simulator UDP Client

This is a UDP client designed to simulate data transmission to a UDP server.
## Build
```sh
cargo build -- --release
```

## Testing
```sh
cargo test -- --nocapture
```

## Sample Command:
```sh
./target/release/gps-tracker-client \
--udp-client-address 0.0.0.0:7001 \
--udp-client-username test1 \
--udp-client-password notsecurepassword \
--udp-client-heartbeat-address 0.0.0.0:7002 \
--udp-client-data-path storage/sample_data_1.json \
--udp-server-config-path config.toml \ 
--coordinates-loop=30
```

