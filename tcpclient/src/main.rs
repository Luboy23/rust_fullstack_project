// 导入必要的模块
use core::str; // 导入核心字符串处理模块
use std::net::TcpStream; // 导入 TCP 流模块
use std::io::{Read, Write}; // 导入读写模块

fn main() {
    // 尝试连接到指定的服务器地址
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    
    // 向服务器发送 "Hello" 消息
    stream.write("Hello".as_bytes()).unwrap();

    // 创建一个缓冲区用于接收服务器的响应
    let mut buffer = [0; 5]; // 创建一个 5 字节的缓冲区
    stream.read(&mut buffer).unwrap(); // 从服务器读取数据到缓冲区

    // 将缓冲区中的字节转换为字符串并打印
    println!("Response from server: {:?}", 
    str::from_utf8(&buffer).unwrap());
}
