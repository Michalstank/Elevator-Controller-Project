pub mod network_driver {
    use std::net::TcpListener;

    const DELAY_DUR: u32 = 1000;
    
    pub struct TcpServer{
        tcp_server: TcpListener
    }

    impl TcpServer {
        pub fn spawn() -> Self{
            let _server = TcpListener::bind("0.0.0.0:8000")
                    .expect("Failed to initalize TCP Server");
            TcpServer {
                tcp_server: _server
            }
        }
    }
    
}