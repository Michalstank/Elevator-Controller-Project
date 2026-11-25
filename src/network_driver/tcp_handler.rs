pub mod tcp_handler {
    use std::net::TcpListener;

    pub struct TcpServer {
        tcp_server: TcpListener
    }

    impl TcpServer {
        pub fn spawn() -> Self {
            let _server = TcpListener::bind("0.0.0.0:8000")
                .expect("Failed to initalize TCP Server.");

            TcpServer {
                tcp_server: _server
            }
        }
    }
}