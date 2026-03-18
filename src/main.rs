use pnet::datalink::interfaces;
use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::Packet;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "netdog")]
struct Args {
    #[arg(short, long)]
    interface: Option<String>,

    #[arg(short, long)]
    protocol: Option<String>,

    #[arg(long)]
    port: Option<u16>,
}

fn main() {
    // Get a vector with all network interfaces found
    let all_interfaces = interfaces();

    // let user_interface =
    let args = Args::parse();
    // println!("{:?}", args);

    // Search for the default interface - the one that is
    // up, not loopback and has an IP.
    let default_interface = all_interfaces
        .iter()
        .find(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty());

    let chosen_interface = match args.interface.as_deref() {
        Some(name) => all_interfaces
            .iter()
            .find(|e| e.name == name)
            .unwrap_or(default_interface.unwrap()),
        None => default_interface.unwrap(),
    };

    // println!("{:?}", args.protocol.as_deref());
    // println!("{:?}", args.port);
    println!("Interface selected {:?}", chosen_interface.name);

    match datalink::channel(chosen_interface, Default::default()) {
        Ok(Ethernet(_tx, mut rx)) => loop {
            match rx.next() {
                Ok(packet) => match EthernetPacket::new(packet) {
                    Some(eth) => {
                        ipv4_parser(eth.payload(), args.protocol.as_deref(), args.port);
                    }
                    None => {}
                },
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        },
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Error: {}", e),
    }
}

fn ipv4_parser(packet: &[u8], protocol_filter: Option<&str>, port_filter: Option<u16>) {
    match Ipv4Packet::new(packet) {
        Some(ipv4) => {
            let source = ipv4.get_source();
            let destination = ipv4.get_destination();
            let protocol = ipv4.get_next_level_protocol();

            match protocol {
                IpNextHeaderProtocols::Tcp => {
                    if let Some(tcp) = TcpPacket::new(ipv4.payload()) {
                        let src_port = tcp.get_source();
                        let dst_port = tcp.get_destination();

                        if let Some("tcp") = protocol_filter {
                            if let Some(p) = port_filter {
                                if src_port != p && dst_port != p {
                                    return;
                                }
                            }
                            println!(
                                "TCP {}:{} -> {}:{}",
                                source, src_port, destination, dst_port
                            );
                        } else if protocol_filter.is_none() {
                            if let Some(p) = port_filter {
                                if src_port != p && dst_port != p {
                                    return;
                                }
                            }
                            println!(
                                "TCP {}:{} -> {}:{}",
                                source, src_port, destination, dst_port
                            );
                        }
                    }
                }

                IpNextHeaderProtocols::Udp => {
                    if let Some(udp) = UdpPacket::new(ipv4.payload()) {
                        let src_port = udp.get_source();
                        let dst_port = udp.get_destination();

                        if let Some("udp") = protocol_filter {
                            if let Some(p) = port_filter {
                                if src_port != p && dst_port != p {
                                    return;
                                }
                            }
                            println!(
                                "UDP {}:{} -> {}:{}",
                                source, src_port, destination, dst_port
                            );
                        } else if protocol_filter.is_none() {
                            if let Some(p) = port_filter {
                                if src_port != p && dst_port != p {
                                    return;
                                }
                            }
                            println!(
                                "UDP {}:{} -> {}:{}",
                                source, src_port, destination, dst_port
                            );
                        }
                    }
                }

                IpNextHeaderProtocols::Icmp => {
                    if protocol_filter.is_none() || protocol_filter == Some("icmp") {
                        println!("ICMP {} -> {}", source, destination);
                    }
                }

                _ => {}
            }
        }
        None => println!("Can't detect Ipv4Packet"),
    }
}
