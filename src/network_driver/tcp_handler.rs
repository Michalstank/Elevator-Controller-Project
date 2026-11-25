pub mod tcp_handler {
    use std::net::TcpListener;

    pub struct TcpServer {
        tcp_server: TcpListener,
        msg_index: u64
    }

    impl TcpServer {
        pub fn spawn() -> Self {
            let _server = TcpListener::bind("0.0.0.0:8001")
                .expect("Failed to initalize TCP Server.");

            _server.set_nonblocking(true).unwrap();

            TcpServer {
                tcp_server: _server,
                msg_index: 0
            }
        }

        fn inc(&mut self) {
            self.msg_index += 1;
        }

        pub fn step(&mut self) {
            self.inc();
            let index = self.msg_index;
            println!("olleh {index}");

            for _ in self.tcp_server.incoming() {
                println!("Connection");
            }
        }
    }
}