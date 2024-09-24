// 导入标准库中的 HashMap，用于存储请求头
use std::collections::HashMap;

// 定义一个枚举类型 Method，表示 HTTP 方法
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,         // GET 方法
    Post,        // POST 方法
    Uninitialized, // 未初始化的状态
}

// 为 Method 实现从字符串转换的功能
impl From<&str> for Method {
    fn from(value: &str) -> Method {
        match value {
            "GET" => Method::Get,        // 将字符串 "GET" 转换为 Method::Get
            "POST" => Method::Post,      // 将字符串 "POST" 转换为 Method::Post
            _ => Method::Uninitialized,  // 其他情况返回未初始化状态
        }
    }
}

// 定义一个枚举类型 Version，表示 HTTP 版本
#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,             // HTTP/1.1 版本
    V2_0,             // HTTP/2.0 版本
    Uninitialized,    // 未初始化的状态
}

// 为 Version 实现从字符串转换的功能
impl From<&str> for Version {
    fn from(value: &str) -> Version {
        match value {
            "HTTP/1.1" => Version::V1_1, // 将字符串 "HTTP/1.1" 转换为 Version::V1_1
            _ => Version::Uninitialized,   // 其他情况返回未初始化状态
        }
    }
}

// 定义一个枚举类型 Resource，表示请求的资源
#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),  // 存储资源的路径
}

// 定义 HttpRequest 结构体，表示 HTTP 请求
#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,            // 请求方法
    pub version: Version,          // HTTP 版本
    pub resource: Resource,        // 请求资源
    pub headers: HashMap<String, String>, // 请求头
    pub msg_body: String,          // 请求消息体
}

// 为 HttpRequest 实现从字符串转换的功能
impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        // 初始化解析的变量
        let mut parsed_method = Method::Uninitialized; // 解析后的请求方法
        let mut parsed_version = Version::V1_1;        // 解析后的 HTTP 版本
        let mut parsed_resource = Resource::Path("".to_string()); // 解析后的资源路径
        let mut parsed_headers = HashMap::new();       // 解析后的请求头
        let mut parsed_msg_body = "";                   // 解析后的消息体
        
        // 按行解析请求字符串
        for line in req.lines() {
            if line.contains("HTTP") { // 检查是否是请求行
                let (method, resource, version) = process_req_line(line); // 处理请求行
                parsed_method = method; // 设置解析后的请求方法
                parsed_resource = resource; // 设置解析后的资源路径
                parsed_version = version; // 设置解析后的 HTTP 版本
            } else if line.contains(":") { // 检查是否是请求头行
                let (key, value) = process_header_line(line); // 处理请求头行
                parsed_headers.insert(key, value); // 将请求头插入 HashMap
            } else if line.len() == 0 { // 检查是否是空行，表示请求头结束
                break; // 退出循环
            } else { // 处理消息体
                parsed_msg_body = line; // 设置消息体
            }
        }

        // 返回解析后的 HttpRequest 结构体
        HttpRequest {
            method: parsed_method,      // 请求方法
            version: parsed_version,    // HTTP 版本
            resource: parsed_resource,   // 请求资源
            headers: parsed_headers,     // 请求头
            msg_body: parsed_msg_body.to_string(), // 消息体
        }
    }
}

// 处理请求行的函数，返回请求方法、资源和版本
fn process_req_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace(); // 按空白字符分割请求行
    let method = words.next().unwrap(); // 获取请求方法
    let resource = words.next().unwrap(); // 获取请求资源
    let version = words.next().unwrap(); // 获取 HTTP 版本

    (
        method.into(), // 转换请求方法
        Resource::Path(resource.to_string()), // 创建 Resource::Path
        version.into(), // 转换 HTTP 版本
    )
}

// 处理请求头行的函数，返回键值对
fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(':'); // 按冒号分割请求头
    let mut key = String::from(""); // 初始化键
    let mut value = String::from(""); // 初始化值
    if let Some(k) = header_items.next() { // 获取键
        key = k.trim().to_string();  // 去除键两侧的空格
    }
    if let Some(v) = header_items.next() { // 获取值
        value = v.trim().to_string();  // 去除值两侧的空格
    }

    (key, value) // 返回键值对
}

// 测试模块
#[cfg(test)]
mod tests {
    use super::*; // 引入外部模块

    // 测试 Method 从字符串转换的功能
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into(); // 从字符串转换为 Method
        assert_eq!(m, Method::Get); // 断言转换结果
    }

    // 测试 Version 从字符串转换的功能
    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into(); // 从字符串转换为 Version
        assert_eq!(v, Version::V1_1); // 断言转换结果
    }

    // 测试 HTTP 请求的解析功能
    #[test]
    fn test_read_http() {
        let s: String = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.71.1\r\nAccept: */*\r\n\r\n"); // 示例请求字符串
        let mut headers_expected = HashMap::new(); // 初始化预期的请求头 HashMap
        headers_expected.insert("Host".into(), "localhost:3000".into()); // 添加 Host 请求头
        headers_expected.insert("Accept".into(), "*/*".into()); // 添加 Accept 请求头
        headers_expected.insert("User-Agent".into(), "curl/7.71.1".into()); // 添加 User-Agent 请求头
        
        let req: HttpRequest = s.into(); // 将请求字符串转换为 HttpRequest 结构体

        // 断言请求方法、版本和资源的解析结果
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        
        // 直接比较 HashMap
        assert_eq!(headers_expected, req.headers); // 断言解析后的请求头与预期一致
    }
}
