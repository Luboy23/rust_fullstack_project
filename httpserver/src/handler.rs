// 导入所需的库和模块
use http::{httprequest::HttpRequest, httpresponse::HttpResponse}; // 导入 HTTP 请求和响应模块
use serde::{Deserialize, Serialize}; // 导入序列化和反序列化库
use std::collections::HashMap; // 导入 HashMap
use std::env; // 导入环境变量模块
use std::fs; // 导入文件系统模块

// 定义 Handler 特性，包含处理请求的方法
pub trait Handler {
    // 处理 HTTP 请求的方法
    fn handle(req: &HttpRequest) -> HttpResponse;

    // 加载文件的方法
    fn load_file(file_name: &str) -> Option<String> {
        // 默认的公共路径
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        // 从环境变量中获取 PUBLIC_PATH，如果不存在则使用默认路径
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        // 构建文件的完整路径
        let full_path = format!("{}/{}", public_path, file_name);

        // 尝试读取文件内容，成功返回 Some(contents)，失败返回 None
        let contents = fs::read_to_string(full_path);
        contents.ok()
    }
}

// 定义 OrderStatus 结构体，用于序列化和反序列化订单状态
#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,         // 订单 ID
    order_date: String,    // 订单日期
    order_status: String,   // 订单状态
}

// 定义处理器结构体
pub struct StaticPageHandler; // 处理静态页面的处理器
pub struct PageNotFoundHandler; // 处理 404 页面请求的处理器
pub struct WebServiceHandler; // 处理 Web 服务请求的处理器

// 实现 PageNotFoundHandler 的 Handler 特性
impl Handler for PageNotFoundHandler {
    fn handle(_req: &HttpRequest) -> HttpResponse {
        // 当找不到页面时，返回 404 响应，并加载 404.html 文件
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}

// 实现 StaticPageHandler 的 Handler 特性
impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        // 获取请求的静态页面资源的路径
        let http::httprequest::Resource::Path(s) = &req.resource;

        // 解析 URI
        let route: Vec<&str> = s.split("/").collect();
        match route[1] {
            "" => HttpResponse::new("200", None, Self::load_file("index.html")), // 根路径请求，返回 index.html
            "health" => HttpResponse::new("200", None, Self::load_file("health.html")), // health 路径请求，返回 health.html
            path => match Self::load_file(path) { // 对其他路径请求，尝试加载对应的文件
                Some(contents) => {
                    let mut map: HashMap<&str, &str> = HashMap::new(); // 创建请求头的 HashMap
                    // 根据文件类型设置 Content-Type
                    if path.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    } else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else {
                        map.insert("Content-Type", "text/html");
                    }
                    // 返回 200 响应和文件内容
                    HttpResponse::new("200", Some(map), Some(contents))
                }
                None => HttpResponse::new("404", None, Self::load_file("404.html")), // 文件未找到，返回 404 响应
            },
        }
    }
}

// 为 WebServiceHandler 定义 load_json() 方法，用于从磁盘加载 orders.json 文件
impl WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        // 默认的数据路径
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        // 从环境变量中获取 DATA_PATH，如果不存在则使用默认路径
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        // 构建 JSON 文件的完整路径
        let full_path = format!("{}/{}", data_path, "orders.json");
        // 尝试读取 JSON 文件内容并解析为 OrderStatus 结构体的向量
        let json_contents = fs::read_to_string(full_path);
        let orders: Vec<OrderStatus> =
            serde_json::from_str(json_contents.unwrap().as_str()).unwrap(); // 解析 JSON
        orders // 返回订单状态列表
    }
}

// 实现 WebServiceHandler 的 Handler 特性
impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        // 获取请求资源的路径
        let http::httprequest::Resource::Path(s) = &req.resource;

        // 解析 URI
        let route: Vec<&str> = s.split("/").collect();
        // 如果请求路径是 /api/shipping/orders，返回 JSON 数据
        match route[2] {
            "shipping" if route.len() > 2 && route[3] == "orders" => {
                // 加载订单数据并将其序列化为 JSON 字符串
                let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
                let mut headers: HashMap<&str, &str> = HashMap::new(); // 创建请求头的 HashMap
                headers.insert("Content-Type", "application/json"); // 设置 Content-Type 为 application/json
                HttpResponse::new("200", Some(headers), body) // 返回 200 响应和 JSON 数据
            }
            _ => HttpResponse::new("404", None, Self::load_file("404.html")), // 其他请求返回 404 响应
        }
    }
}
