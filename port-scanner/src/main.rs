extern crate rayon;

use std::{ net, env, thread, time, fs, collections };
use pnet::packet::{ tcp, ip };
use pnet::transport::{ self, TransportProtocol };

const TCP_SIZE: usize = 20;
const MAXIMUM_PORT_NUM: u16 = 1023;

struct PacketInfo {
    my_ipaddr: net::Ipv4Addr,
    target_ipaddr: net::Ipv4Addr,
    my_port: u16,
    maximum_port: u16,
    scan_type: ScanType,
}

#[derive(Copy, Clone)]
enum ScanType {
    SynScan = tcp::TcpFlags::SYN as isize,
    FinScan = tcp::TcpFlags::FIN as isize,
    XmasScan = tcp::TcpFlags::FIN as isize | tcp::TcpFlags::URG as isize | tcp::TcpFlags::PSH as isize,
    NullScan = 0
}

fn main() {
    println!("Hello, world!");
}
