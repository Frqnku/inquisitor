use pnet::{
    datalink,
    packet::{ethernet::EthernetPacket, ipv4::Ipv4Packet, Packet},
};

pub fn handle_ipv4_packet(
    eth: &EthernetPacket,
    args: &crate::parse_args::Args,
    tx: &mut Box<dyn datalink::DataLinkSender>,
) {
    if let Some(ip) = Ipv4Packet::new(eth.payload()) {
        println!(
            "Received IPv4 packet: {} -> {}",
            ip.get_source(),
            ip.get_destination()
        );
    }
}
