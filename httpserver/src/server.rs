// 导入必要的模块
use super::router::Router; // 导入路由模块
use http::httprequest::HttpRequest; // 导入 HTTP 请求结构
use std::io::prelude::*; // 导入 IO 预备函数
use std::net::TcpListener; // 导入 TCP 监听器
use std::str; // 导入字符串处理模块

// 定义 Server 结构体
pub struct Server<'a> {
    socket_addr: &'a str, // 服务器的 socket 地址
}

impl<'a> Server<'a> {
    // 创建一个新的 Server 实例
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr } // 返回新的 Server 实例
    }

    // 运行服务器
    pub fn run(&self) {
        // 在指定的 socket 地址上启动服务器
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr); // 打印服务器运行地址

        // 循环监听传入的连接
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap(); // 解包连接流
            println!("Connection established"); // 打印连接建立信息
            let mut read_buffer = [0; 1024]; // 创建一个缓冲区用于读取数据
            stream.read(&mut read_buffer).unwrap(); // 从流中读取数据到缓冲区

            // 将读取的 HTTP 请求转换为 Rust 数据结构
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();

            // 将请求路由到适当的处理器
            Router::route(req, &mut stream);
        }
    }
}
