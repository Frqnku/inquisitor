use std::net::Ipv4Addr;

use clap::Parser;
use pnet::util::MacAddr;

#[derive(Parser, Debug)]
pub struct Args {
    pub ip_src: Ipv4Addr,
    pub mac_src: MacAddr,
    pub ip_target: Ipv4Addr,
    pub mac_target: MacAddr,
}

impl Args {
    pub fn new() -> Self {
        Args::parse()
    }

    pub fn verify_args(&self) {
        if (self.ip_src == self.ip_target) || (self.mac_src == self.mac_target) {
            eprintln!("Error: Source and target IP/MAC addresses must be different.");
            std::process::exit(1);
        }
    }
}
