# netdog

A tiny packet sniffer written in Rust.

`netdog` captures live network traffic and prints TCP, UDP, and ICMP packets in a clean terminal format. It supports filtering by network interface, protocol, and port.

## Features

- Capture live packets from a network interface
- Parse and print:
  - TCP
  - UDP
  - ICMP
- Filter by:
  - interface
  - protocol
  - port
- Simple CLI output with packet counter and timestamp

## Example Output

```text
[1][21:04:33] TCP 192.168.31.174:53122 -> 104.29.138.175:443
[2][21:04:34] UDP 192.168.31.174:53000 -> 8.8.8.8:53
[3][21:04:35] ICMP 192.168.31.174 -> 1.1.1.1
```

## Requirements

- Linux
- Rust
- Root privileges or raw socket capabilities

## Build

```bash
cargo build
```

## Run

Since packet sniffing requires special permissions, run the built binary with `sudo`:

```bash
sudo ./target/debug/netdog
```

## Usage

```bash
netdog [OPTIONS]
```

### Options

- `-i, --interface <INTERFACE>`
  Network interface to listen on

- `-p, --protocol <PROTOCOL>`
  Protocol filter: `tcp`, `udp`, or `icmp`

- `--port <PORT>`
  Port filter

## Examples

Listen on the default interface:

```bash
sudo ./target/debug/netdog
```

Listen on a specific interface:

```bash
sudo ./target/debug/netdog --interface enp3s0f0
```

Show only TCP packets:

```bash
sudo ./target/debug/netdog --protocol tcp
```

Show only UDP packets on port 53:

```bash
sudo ./target/debug/netdog --protocol udp --port 53
```

Show only traffic on port 443:

```bash
sudo ./target/debug/netdog --port 443
```

## Notes

If you use `sudo cargo run`, your system may fail to find the Rust toolchain depending on your `rustup` setup. In that case, build first and run the binary directly:

```bash
cargo build
sudo ./target/debug/netdog
```

You can also grant the binary the required capabilities:

```bash
sudo setcap cap_net_raw,cap_net_admin=eip target/debug/netdog
```

Then run without `sudo`:

```bash
./target/debug/netdog
```

## Project Structure

```text
src/
├── main.rs
├── cli.rs
└── parser.rs
```

## Current Scope

`netdog` is a lightweight CLI sniffer focused on being small, readable, and useful as a learning project. It is not intended to replace tools like Wireshark or tcpdump.

## Why I built this

I built `netdog` to get hands-on practice with Rust, packet parsing, and low-level networking before moving deeper into eBPF and systems programming.

## Future Ideas

- IPv6 support
- packet statistics
- colored output
- TUI mode
- protocol/service labeling

## License

MIT
