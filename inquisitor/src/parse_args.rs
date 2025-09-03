use std::net::Ipv4Addr;

use clap::Parser;
use macaddr::MacAddr6;

#[derive(Parser, Debug)]
pub struct Args {
    pub ip_src: Ipv4Addr,
    pub mac_src: MacAddr6,
    pub ip_target: Ipv4Addr,
    pub mac_target: MacAddr6,
}

impl Args {
    pub fn new() -> Self {
        Args::parse()
    }
}
