// 导入必要的模块
use std::net::TcpListener; // 导入 TCP 监听器模块
use std::io::{Read, Write}; // 导入读写模块

fn main() {
    // 创建一个 TCP 监听器，绑定到本地地址 127.0.0.1:3000
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000..."); // 输出服务器正在运行的消息

    // 使用循环处理来自客户端的连接请求
    for stream in listener.incoming() {
        // 尝试获取流，如果出错则 panic
        let mut stream = stream.unwrap();
        println!("Connection established!"); // 输出连接已建立的消息

        let mut buffer = [0; 1024]; // 创建一个 1024 字节的缓冲区

        // 从客户端读取数据并填充缓冲区
        stream.read(&mut buffer).unwrap();

        // 将读取到的数据写回给客户端（回显功能）
        stream.write(&mut buffer).unwrap();
    }
}
