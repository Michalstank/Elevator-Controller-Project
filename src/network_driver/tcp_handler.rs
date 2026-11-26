pub mod tcp_handler {
    use std::net::{TcpListener, TcpStream};
    use std::io::{BufReader, prelude::*};

    pub struct TcpServer {
        tcp_server: TcpListener,
        msg_index: u64
    }

    impl TcpServer {
        pub fn spawn() -> Self {
            let _server = TcpListener::bind("0.0.0.0:8001")
                .expect("Failed to initalize TCP Server.");

            //_server.set_nonblocking(true).unwrap();

            TcpServer {
                tcp_server: _server,
                msg_index: 0
            }
        }

        fn inc(&mut self) {
            self.msg_index += 1;
        }

        fn handle_connection(mut stream: TcpStream) {
            let buf_reader = BufReader::new(&stream);
            let http_req: Vec<_> = buf_reader
                .lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();

            let response = "HTTP/1.1 200 OK\r\n\r\n";

            stream.write_all(response.as_bytes()).unwrap();
        }

        pub fn step(&mut self) {
            self.inc();
            let index = self.msg_index;
            println!("olleh {index}");

            for stream in self.tcp_server.incoming() {
                let s: TcpStream = stream.unwrap();
                println!("hello team");
                TcpServer::handle_connection(s);
            }
        }
    }
}