pub mod udp_handler {
    use std::net::UdpSocket;

    pub struct UdpServer {
        udp_server: UdpSocket,
        pub msg_index: u64
    }

    impl UdpServer {
        pub fn spawn() -> Self {
            let _server = UdpSocket::bind("0.0.0.0:8000")
                .expect("Failed to initalize UDP Server.");

            UdpServer { 
                udp_server: _server,
                msg_index: 0 
            }
        }

        fn inc(&mut self) {
            self.msg_index += 1;
        }

        pub fn step(&mut self) {
            self.inc();
            let index = self.msg_index;
            println!("Hello: {index}");
        }
    }
}