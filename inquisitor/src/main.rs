use pnet::{
    datalink::{self, Channel::Ethernet},
    packet::ethernet::{EtherTypes, EthernetPacket},
};

use crate::parse_args::Args;
use crate::{arp::handle_arp_packet, ipv4::handle_ipv4_packet};

mod arp;
mod ipv4;
mod parse_args;

fn main() {
    let args = Args::new();
    args.verify_args();

    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.ips.iter().any(|ip| ip.ip() == args.ip_src))
        .or_else(|| {
            eprintln!("Error: No network interface found with the specified source IP address.");
            std::process::exit(1);
        })
        .unwrap();

    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => {
            eprintln!("Error: Unsupported channel type");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Error creating datalink channel: {e}");
            std::process::exit(1);
        }
    };

    loop {
        let mut _buf = [0u8; 1500];
        match rx.next() {
            Ok(packet) => {
                if let Some(eth) = EthernetPacket::new(packet) {
                    match eth.get_ethertype() {
                        EtherTypes::Arp => {
                            handle_arp_packet(&eth, &args, &mut tx);
                            continue;
                        }
                        EtherTypes::Ipv4 => {
                            handle_ipv4_packet(&eth, &args, &mut tx);
                            continue;
                        }
                        _ => (),
                    }
                }
            }
            Err(e) => {
                eprintln!("Error receiving packet: {e}");
            }
        }
    }
}
