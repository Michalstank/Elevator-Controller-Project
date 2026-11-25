use crate::network_driver::{tcp_handler::tcp_handler::TcpServer, udp_handler::udp_handler::UdpServer};

pub mod tcp_handler;
pub mod udp_handler;

pub fn network_thread() {
    let mut tcp = TcpServer::spawn();
    let mut udp = UdpServer::spawn();

    loop {
        tcp.step();
        udp.step();
    }
}