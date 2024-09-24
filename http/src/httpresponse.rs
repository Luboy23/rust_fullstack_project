// 导入标准库中的 HashMap 和 Result, Write 模块
use std::collections::HashMap;
use std::io::{Result, Write};

// 定义 HttpResponse 结构体，表示 HTTP 响应
#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,                     // HTTP 版本
    status_code: &'a str,                 // 状态码
    status_text: &'a str,                  // 状态文本
    headers: Option<HashMap<&'a str, &'a str>>, // 可选的请求头
    body: Option<String>,                  // 可选的消息体
}

// 为 HttpResponse 实现 Default trait，提供默认值
impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),   // 默认版本为 HTTP/1.1
            status_code: "200".into(),     // 默认状态码为 200
            status_text: "OK".into(),      // 默认状态文本为 OK
            headers: None,                  // 默认无请求头
            body: None,                     // 默认无消息体
        }
    }
}

// 为 HttpResponse 实现相关方法
impl<'a> HttpResponse<'a> {
    // 创建一个新的 HttpResponse
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        // 使用默认构造函数创建响应
        let mut response: HttpResponse<'a> = HttpResponse::default();

        // 如果状态码不是 200，则设置状态码
        if status_code != "200" {
            response.status_code = status_code.into();
        };

        // 设置响应头，如果未提供则使用默认的 Content-Type
        response.headers = match &headers {
            Some(_h) => headers, // 如果提供了头部，直接使用
            None => {
                let mut h = HashMap::new(); // 创建一个新的 HashMap
                h.insert("Content-Type", "text/html"); // 设置默认 Content-Type
                Some(h) // 返回包含默认头部的 HashMap
            }
        };

        // 根据状态码设置状态文本
        response.status_text = match response.status_code {
            "200" => "OK".into(),                     // 200 状态返回 OK
            "400" => "Bad Request".into(),             // 400 状态返回 Bad Request
            "404" => "Not Found".into(),               // 404 状态返回 Not Found
            "500" => "Internal Server Error".into(),  // 500 状态返回 Internal Server Error
            _ => "Not Found".into(),                   // 其他状态返回 Not Found
        };

        // 设置消息体
        response.body = body;
        response // 返回创建的 HttpResponse
    }

    // 发送响应到写入流
    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone(); // 克隆响应
        let response_string: String = String::from(res); // 将响应转换为字符串
        let _ = write!(write_stream, "{}", response_string); // 将字符串写入流
        Ok(()) // 返回成功结果
    }
}

// 为 HttpResponse 实现访问器方法
impl<'a> HttpResponse<'a> {
    // 返回 HTTP 版本
    fn version(&self) -> &str {
        self.version
    }

    // 返回状态码
    fn status_code(&self) -> &str {
        self.status_code
    }

    // 返回状态文本
    fn status_text(&self) -> &str {
        self.status_text
    }

    // 返回请求头的字符串表示
    fn headers(&self) -> String {
        let map: HashMap<&str, &str> = self.headers.clone().unwrap(); // 克隆请求头 HashMap
        let mut header_string: String = "".into(); // 初始化头字符串
        // 遍历所有头部，将其格式化为字符串
        for (k, v) in map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v); // 添加头部
        }
        header_string // 返回格式化后的头字符串
    }

    // 返回消息体
    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(), // 如果有消息体，返回其字符串
            None => "", // 如果没有消息体，返回空字符串
        }
    }
}

// 为 HttpResponse 实现从 HttpResponse 转换为 String 的功能
impl <'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse<'a>) -> Self {
        let res1 = res.clone(); // 克隆响应
        let body_len = res.body.as_ref().map_or(0, |b| b.len()); // 计算消息体长度，如果为 None 则返回 0
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}", // 格式化字符串
            &res1.version(), // 添加版本
            &res1.status_code(), // 添加状态码
            &res1.status_text(), // 添加状态文本
            &res1.headers(), // 添加请求头
            body_len, // 添加消息体长度
            &res1.body() // 添加消息体
        )
    }
}

// 测试模块
#[cfg(test)]
mod tests {
    use super::*; // 引入外部模块

    // 测试创建 HTTP 响应结构体（状态码为 200）
    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new(
            "200", // 状态码
            None, // 无请求头
            Some("Item was shipped on 21st Dec 2020".into()), // 消息体
        );
        let response_expected = HttpResponse {
            version: "HTTP/1.1", // 默认版本
            status_code: "200", // 状态码
            status_text: "OK", // 状态文本
            headers: {
                let mut h = HashMap::new(); // 创建请求头 HashMap
                h.insert("Content-Type", "text/html"); // 设置默认 Content-Type
                Some(h) // 返回请求头
            },
            body: Some("Item was shipped on 21st Dec 2020".into()), // 消息体
        };
        assert_eq!(response_actual, response_expected); // 断言实际响应与预期响应相等
    }

    // 测试创建 HTTP 响应结构体（状态码为 404）
    #[test]
    fn test_response_struct_creation_404() {
        let response_actual = HttpResponse::new(
            "404", // 状态码
            None, // 无请求头
            Some("Item was shipped on 21st Dec 2020".into()), // 消息体
        );
        let response_expected = HttpResponse {
            version: "HTTP/1.1", // 默认版本
            status_code: "404", // 状态码
            status_text: "Not Found", // 状态文本
            headers: {
                let mut h = HashMap::new(); // 创建请求头 HashMap
                h.insert("Content-Type", "text/html"); // 设置默认 Content-Type
                Some(h) // 返回请求头
            },
            body: Some("Item was shipped on 21st Dec 2020".into()), // 消息体
        };
        assert_eq!(response_actual, response_expected); // 断言实际响应与预期响应相等
    }

    // 测试 HTTP 响应的字符串化
    #[test]
    fn test_http_response_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1", // 默认版本
            status_code: "404", // 状态码
            status_text: "Not Found", // 状态文本
            headers: {
                let mut h = HashMap::new(); // 创建请求头 HashMap
                h.insert("Content-Type", "text/html"); // 设置默认 Content-Type
                Some(h) // 返回请求头
            },
            body: Some("Item was shipped on 21st Dec 2020".into()), // 消息体
        };
        let http_string: String = response_expected.into(); // 将 HttpResponse 转换为字符串
        let response_actual = "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length: 33\r\n\r\nItem was shipped on 21st Dec 2020"; // 预期的 HTTP 字符串
        assert_eq!(http_string, response_actual); // 断言实际字符串与预期字符串相等
    }
}
