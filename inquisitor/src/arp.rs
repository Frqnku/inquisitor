use pnet::datalink;
use pnet::packet::arp::{ArpHardwareTypes, ArpOperations, ArpPacket, MutableArpPacket};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::{MutablePacket, Packet};
use pnet::util::MacAddr;

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

pub fn handle_arp_packet(
    eth: &EthernetPacket,
    args: &crate::parse_args::Args,
    tx: &mut Box<dyn datalink::DataLinkSender>,
) {
    if let Some(arp) = ArpPacket::new(eth.payload()) {
        let packet = build_arp_reply(args.mac_src, args.ip_target, args.mac_target, args.ip_src);
        let _ = tx.send_to(&packet, None).unwrap();
        if arp.get_operation() == ArpOperations::Request
            && args.ip_target == arp.get_target_proto_addr()
        {
            let reply_packet = build_arp_reply(
                args.mac_src,
                args.ip_target,
                arp.get_sender_hw_addr(),
                arp.get_sender_proto_addr(),
            );
            let _ = tx.send_to(&reply_packet, None).unwrap();
            println!("Sent ARP reply: {} is at {}", args.ip_target, args.mac_src);
        }
    }
}
