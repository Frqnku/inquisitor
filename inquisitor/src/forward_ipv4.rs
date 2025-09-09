use std::sync::{Arc, Mutex};

use pnet::datalink;
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};
use pnet::packet::{MutablePacket, Packet};
use pnet::util::MacAddr;

pub fn forward_ipv4_packet(
    eth: &EthernetPacket,
    tx: Arc<Mutex<Box<dyn datalink::DataLinkSender>>>,
    attacker_mac: MacAddr,
    dst_mac: MacAddr,
) {
    let mut buf = vec![0u8; eth.packet().len()];
    let mut new_eth = MutableEthernetPacket::new(&mut buf).unwrap();

    new_eth.clone_from(eth);

    new_eth.set_source(attacker_mac);
    new_eth.set_destination(dst_mac);

    match tx.lock().unwrap().send_to(new_eth.packet(), None).unwrap() {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to forward packet: {e}"),
    }
}
