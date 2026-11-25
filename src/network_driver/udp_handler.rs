pub mod udp_handler {
    use std::net::UdpSocket;

    pub struct UdpServer {
        udp_server: UdpSocket
    }

    impl UdpServer {
        pub fn spawn() -> Self {
            let _server = UdpSocket::bind("0.0.0.0:8000")
                .expect("Failed to initalize UDP Server.");

            UdpServer { 
                udp_server: _server 
            }
        }
    }
}