use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "netdog",
    version,
    about = "A tiny packet sniffer written in Rust",
    long_about = "netdog captures live network packets and lets you filter by interface, protocol, and port."
)]
pub struct Args {
    #[arg(
        short,
        long,
        help = "Network interface to listen on, for example enp3s0f0"
    )]
    pub interface: Option<String>,

    #[arg(short, long, help = "Protocol filter: tcp, udp, or icmp")]
    pub protocol: Option<String>,

    #[arg(long, help = "Port filter, for example 443 or 53")]
    pub port: Option<u16>,
}
