mod cli;
mod parser;

use clap::Parser;
use pnet::datalink::interfaces;
use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::Packet;
use pnet::packet::ethernet::EthernetPacket;

use cli::Args;
use parser::ipv4_parser;

fn main() {
    let all_interfaces = interfaces();
    let args = Args::parse();

    let default_interface = all_interfaces
        .iter()
        .find(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty())
        .expect("Error: unable to detect default interface");

    let chosen_interface = match args.interface.as_deref() {
        Some(name) => match all_interfaces.iter().find(|e| e.name == name) {
            Some(interface) => interface,
            None => {
                eprintln!("Error: interface '{}' not found", name);
                return;
            }
        },
        None => default_interface,
    };

    let protocol = match args.protocol.as_deref() {
        Some("tcp") => Some("tcp"),
        Some("udp") => Some("udp"),
        Some("icmp") => Some("icmp"),
        Some(other) => {
            eprintln!("Error: unsupported protocol '{}'", other);
            return;
        }
        None => None,
    };

    println!("Interface selected {}", chosen_interface.name);

    let mut counter = 0;

    match datalink::channel(chosen_interface, Default::default()) {
        Ok(Ethernet(_tx, mut rx)) => loop {
            match rx.next() {
                Ok(packet) => {
                    if let Some(eth) = EthernetPacket::new(packet) {
                        ipv4_parser(eth.payload(), protocol, args.port, &mut counter);
                    }
                }
                Err(e) => eprintln!("error: {}", e),
            }
        },
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Error: {}", e),
    }
}
