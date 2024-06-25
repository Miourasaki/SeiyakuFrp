use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use crate::config::Configure;

pub struct Server {
    listener: TcpListener,
    stream_map: HashMap<u16, TcpStream>
}

impl Server {
    pub fn from(configure: Configure) -> Result<Self, Box<dyn Error>> {

        let server_address = format!("{}:{}", configure.host, configure.port);

        let server = TcpListener::bind(server_address.as_str())?;

        Ok(Self {
            listener: server,
            stream_map: HashMap::new()
        })
    }

    pub fn start(&self) {

        for stream in self.listener.incoming() {
            if let Ok(stream) = stream {
                self.wait_reg(stream)
            }
        }
    }

    fn wait_reg(&self, mut stream: TcpStream) {
        loop {
            let mut buffer = [0u8; 1024]; // 创建一个缓冲区来存储从客户端接收的数据

            // 读取客户端发送的数据
            match stream.read(&mut buffer) {
                Ok(bytes_read) if bytes_read > 0 => {
                    // 将读取到的数据回显给客户端
                    if let Ok(message) = std::str::from_utf8(&buffer[..bytes_read]) {
                        // 打印接收到的消息
                        println!("Received message: {}", message);
                    } else {
                        // 如果不是有效的UTF-8，则打印错误或忽略
                        eprintln!("Received non-UTF-8 data");
                    }
                },
                Ok(0) => {
                    // 客户端已经关闭了连接
                    eprintln!("Client closed the connection");
                    break
                },
                Err(e) => {
                    // 处理读取数据时发生的错误
                    eprintln!("Error reading from client: {}", e);
                    break
                },
                _ => ()
            }
        }

        // 关闭连接
        let _ = stream.shutdown(std::net::Shutdown::Both);
    }


}



