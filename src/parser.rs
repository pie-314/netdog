use chrono::Local;
use pnet::packet::Packet;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;

fn protocol_allowed(filter: Option<&str>, current: &str) -> bool {
    match filter {
        Some(value) => value == current,
        None => true,
    }
}

fn port_allowed(filter: Option<u16>, src: u16, dst: u16) -> bool {
    match filter {
        Some(port) => src == port || dst == port,
        None => true,
    }
}

pub fn ipv4_parser(
    packet: &[u8],
    protocol_filter: Option<&str>,
    port_filter: Option<u16>,
    counter: &mut i32,
) {
    let now = Local::now().format("%H:%M:%S");

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

                        if !protocol_allowed(protocol_filter, "tcp")
                            || !port_allowed(port_filter, src_port, dst_port)
                        {
                            return;
                        }

                        *counter += 1;
                        println!(
                            "[{}][{}] TCP {}:{} -> {}:{}",
                            *counter, now, source, src_port, destination, dst_port
                        );
                    }
                }

                IpNextHeaderProtocols::Udp => {
                    if let Some(udp) = UdpPacket::new(ipv4.payload()) {
                        let src_port = udp.get_source();
                        let dst_port = udp.get_destination();

                        if !protocol_allowed(protocol_filter, "udp")
                            || !port_allowed(port_filter, src_port, dst_port)
                        {
                            return;
                        }

                        *counter += 1;
                        println!(
                            "[{}][{}] UDP {}:{} -> {}:{}",
                            *counter, now, source, src_port, destination, dst_port
                        );
                    }
                }

                IpNextHeaderProtocols::Icmp => {
                    if !protocol_allowed(protocol_filter, "icmp") {
                        return;
                    }

                    *counter += 1;
                    println!("[{}][{}] ICMP {} -> {}", *counter, now, source, destination);
                }

                _ => {}
            }
        }
        None => eprintln!("Can't detect Ipv4Packet"),
    }
}
