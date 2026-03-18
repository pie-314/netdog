use pnet::datalink::interfaces;
use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::Packet;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;

fn main() {
    // Get a vector with all network interfaces found
    let all_interfaces = interfaces();

    // Search for the default interface - the one that is
    // up, not loopback and has an IP.
    let default_interface = all_interfaces
        .iter()
        .find(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty());

    // print all networks
    // for x in all_interfaces.iter() {
    //     println!("{}", x);
    // }
    //

    match default_interface {
        Some(interface) => {
            println!("Found default interface with [{}].", interface.name);
            match datalink::channel(interface, Default::default()) {
                Ok(Ethernet(_tx, mut rx)) => loop {
                    match rx.next() {
                        Ok(packet) => match EthernetPacket::new(packet) {
                            Some(eth) => {
                                ipv4_parser(eth.payload());
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
        None => println!("Error while finding the default interface."),
    }
}

fn ipv4_parser(packet: &[u8]) {
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

                        println!(
                            "TCP {}:{} -> {}:{}",
                            source, src_port, destination, dst_port
                        );
                    }
                }
                IpNextHeaderProtocols::Udp => {
                    if let Some(udp) = UdpPacket::new(ipv4.payload()) {
                        let src_port = udp.get_source();
                        let dst_port = udp.get_destination();

                        println!(
                            "UDP {}:{} -> {}:{}",
                            source, src_port, destination, dst_port
                        );
                    }
                }
                IpNextHeaderProtocols::Icmp => {
                    println!("ICMP {} -> {}", source, destination);
                }
                _ => {}
            }
        }
        None => println!("Can't detect Ipv4Packet"),
    }
}
