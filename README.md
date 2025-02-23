# Car GPS App

This is for studying packets, and UDP using Rust Programming Language. The app is just a basic gps tracker where in car sends gps locations to the gps server via UDP while the data is represented in packets.

# Sending Packets

```
printf "01 00 14 72 6F 6F 74 00 00 00 00 00 00 6E 6F 74 73 65 63 75 72 65 70 61 73 73 77 6F 72 64" | xxd -r -p > login_packet_1.bin
printf "03 00 04 00 00 6B 43" | xxd -r -p > heartbeat_packet_1.bin

14.650221904817329, 121.04681722724743
printf "02 00 04 00 00 6B 43 40 2D 4C F3 81 7F 57 D2 40 5E 42 FE CE 09 D7 FB" | xxd -r -p > coordinates_packet_1.bin

```

## Login Packets

 ``sh
cat login_packet.bin | nc -u 127.0.0.1 34256

```

# Starting SurrealDB:

```sh
surreal start --user root --pass root --bind 0.0.0.0:8080 rocksdb:gps.db
```

# Notes:

- Use mio library for UDP
- create tools to generate payloads easily
- make the server not exit for errors.
- use logging instead of printing
- check if xxd is need to send data or I can just use the generated payload instead.
