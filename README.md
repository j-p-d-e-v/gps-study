# Car GPS App

This is for studying packets, and UDP using Rust Programming Language. The app is just a basic gps tracker where in car sends gps locations to the gps server via UDP while the data is represented in packets.

# Sending Packets

## Login Packets

```sh
cat login_packet.bin | nc -u 127.0.0.1 34256

```

# Notes:

- Use mio library for UDP
